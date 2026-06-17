mod mpe;
use std::sync::Arc;

use base_util::onnx::{new_session, Providers};
use interface_image::{ImageOp, RawImage, RawImageCow, RawImageView};
use interface_inpainter::{Inpainter, InpainterOptions};
use interface_model::{
    impl_model_helpers, impl_model_load_helpers, Model, ModelLoad, ModelRead, ModelSource,
    ModelWrap,
};
use maplit::hashmap;
use ndarray::{ArrayView4, Axis};
use ort::{inputs, session::RunOptions, value::Tensor};
use ort_parallel::AsyncSessionPool;
use util::{
    lama::{crop_mask, crop_rgb, lama_add_border, lama_mask_regions, lama_resize_image, paste_rgb},
    spawn_blocking,
};

/// Context margin (px) added around each text region before inpainting.
const LOCAL_INPAINT_PADDING: interface_image::DimType = 32;
/// If the mask splits into more regions than this, fall back to whole-page inpainting.
const MAX_LOCAL_REGIONS: usize = 32;
/// If the regions together cover more than this fraction of the page, whole-page is cheaper.
const MAX_LOCAL_COVERAGE: f64 = 0.6;

pub struct LamaLargeInpainter {
    model: ModelWrap<AsyncSessionPool>,
    providers: Arc<Vec<Providers>>,
}

impl LamaLargeInpainter {
    pub fn new(providers: Arc<Vec<Providers>>) -> Self {
        Self {
            model: Default::default(),
            providers,
        }
    }
}

#[async_trait::async_trait]
impl ModelLoad for LamaLargeInpainter {
    impl_model_load_helpers!(model, AsyncSessionPool);
    async fn reload(&self) -> anyhow::Result<ModelRead<'_, Self::T>> {
        let p = self.download_model("model", "model.onnx").await?;
        let m = AsyncSessionPool::commit_from_file(new_session(&self.providers)?, &p, 10)?;
        *self.model.write().await = Some(m);
        Ok(self.get_model().await.expect("set before"))
    }
}
impl Model for LamaLargeInpainter {
    impl_model_helpers!("inpainter", "lama_mpe", model);

    fn models(&self) -> std::collections::HashMap<&'static str, interface_model::ModelSource> {
        hashmap! {"model" => ModelSource{ url: "https://github.com/frederik-uni/manga-image-translator-rust/releases/download/lama_mpe/model.onnx", hash: "###" }}
    }
}

impl LamaLargeInpainter {
    /// Run LaMa+MPE on a single (whole-image or cropped) region and return an RGB image at
    /// the same size as the input view. Only the masked pixels matter to the caller; pixels
    /// outside the mask are discarded during compositing.
    async fn inpaint_region(
        &self,
        image: RawImageView<'_>,
        mask: interface_image::Mask,
        inpainting_size: u16,
        img_processor: &Arc<dyn ImageOp + Send + Sync>,
    ) -> anyhow::Result<RawImage> {
        let ho = image.height;
        let wo = image.width;
        let (image, mask, rel_pos, direct, w, h, new_w, new_h) = spawn_blocking!(|| {
            let (image, mask) = lama_resize_image(image, mask, inpainting_size, img_processor)?;
            let mut image = image.to_owned();

            let h = image.height;
            let w = image.width;
            image = interface_inpainter::remove_mask_area(image, &mask);

            let (image, mask, new_w, new_h) = lama_add_border(image, mask, img_processor);
            let (rel_pos, direct) = mpe::load_masked_position_encoding(mask.view(), img_processor)?;
            let mask = mask
                .as_nd()?
                .mapv(|v| if v >= 127 { 1.0f32 } else { 0.0f32 })
                .insert_axis(Axis(0))
                .insert_axis(Axis(0));
            let image = image
                .as_ndarray()
                .unwrap()
                .permuted_axes((2, 0, 1))
                .mapv(|v| v as f32 / 255.0)
                .insert_axis(Axis(0));
            let image = Tensor::from_array(image)?;
            let mask = Tensor::from_array(mask)?;
            let rel_pos = Tensor::from_array(rel_pos.insert_axis(Axis(0)))?;
            let direct = Tensor::from_array(direct.insert_axis(Axis(0)))?;
            Ok::<_, anyhow::Error>((image, mask, rel_pos, direct, w, h, new_w, new_h))
        })??;

        let opt = RunOptions::new()?;
        let model = self.load().await?;
        let out = model
            .run_async(
                inputs! {"image"=> image, "mask"=> mask, "rel_pos" => rel_pos, "direct" => direct},
                &opt,
            )
            .await?;
        let img = spawn_blocking!(|| {
            let out: ArrayView4<f32> = out[0].try_extract_array()?.into_dimensionality()?;
            let img_inpainted = out
                .remove_axis(Axis(0))
                .permuted_axes((1, 2, 0))
                .mapv(|v| (v * 255.0) as u8);
            let mut img_inpainted = RawImageCow::from(img_inpainted.view());
            if new_h != h || new_w != w {
                img_inpainted =
                    RawImageCow::Owned(img_processor.remove_border(img_inpainted.view(), w, h));
            }
            if h != ho || w != wo {
                img_inpainted = RawImageCow::Owned(img_processor.resize(
                    img_inpainted.view(),
                    wo,
                    ho,
                    interface_image::Interpolation::Bicubic,
                )?);
            }
            Ok::<_, anyhow::Error>(img_inpainted.to_owned())
        })??;

        Ok(img)
    }
}

#[async_trait::async_trait]
impl Inpainter for LamaLargeInpainter {
    async fn inpaint(
        &self,
        image: &interface_image::RawImage,
        mask: interface_image::Mask,
        options: InpainterOptions,
        img_processor: &Arc<dyn ImageOp + Send + Sync>,
    ) -> anyhow::Result<interface_image::RawImage> {
        let regions = lama_mask_regions(&mask, LOCAL_INPAINT_PADDING)?;

        // Nothing to inpaint: masked pixels are the only ones kept downstream.
        if regions.is_empty() {
            return Ok(image.view().to_owned());
        }

        // Decide whether local (bbox) inpainting is worthwhile. If the text is too
        // fragmented or covers most of the page, a single whole-page pass is cheaper.
        let page_area = image.width as f64 * image.height as f64;
        let region_area: f64 = regions.iter().map(|b| b.w as f64 * b.h as f64).sum::<f64>();
        let use_local = regions.len() <= MAX_LOCAL_REGIONS
            && (page_area <= 0.0 || region_area / page_area <= MAX_LOCAL_COVERAGE);

        if !use_local {
            return self
                .inpaint_region(image.view(), mask, options.inpainting_size, img_processor)
                .await;
        }

        // Local path: inpaint each text region on a small crop and paste it back. The
        // surrounding (unmasked) pixels of each crop are discarded during compositing,
        // so seams cannot appear in the final output.
        let mut output = image.view().to_owned();
        for b in &regions {
            let crop_img = crop_rgb(image.view(), b);
            let crop_msk = crop_mask(&mask, b);
            let inpainted = self
                .inpaint_region(
                    crop_img.view(),
                    crop_msk,
                    options.inpainting_size,
                    img_processor,
                )
                .await?;
            paste_rgb(&mut output, &inpainted, b);
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use interface_image::{CpuImageProcessor, Mask, RawImage};
    use ndarray::Array2;

    use super::*;

    #[tokio::test]
    async fn test_inpaint() {
        let img = RawImage::new("./imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
            .expect("Failed to load image");
        let img = RawImage::from(img);
        let img_processor =
            Arc::new(CpuImageProcessor::default()) as Arc<dyn ImageOp + Send + Sync>;
        let mask: Array2<u8> = ndarray_npy::read_npy("../lama_large/mask.npy").unwrap();
        let mask = Mask::from(mask);
        let inp = LamaLargeInpainter::new(Default::default());
        let v = inp
            .inpaint(&Arc::new(img), mask, Default::default(), &img_processor)
            .await
            .unwrap();
        v.to_image().unwrap().save("inpainted.png").unwrap()
    }
}
