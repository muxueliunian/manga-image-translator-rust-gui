use std::sync::Arc;

use interface_image::{DimType, ImageOp, Mask, RawImage, RawImageCow, RawImageView};
use opencv::{
    core::{Mat, MatTraitConst as _, Point, BORDER_CONSTANT, CV_32S},
    imgproc::{
        connected_components_with_stats, dilate, get_structuring_element,
        morphology_default_border_value, CC_STAT_AREA, CC_STAT_HEIGHT, CC_STAT_LEFT, CC_STAT_TOP,
        CC_STAT_WIDTH, MORPH_ELLIPSE,
    },
};

pub fn resize_keep_aspect(
    img: RawImageView,
    size: u16,
    img_processor: &Arc<dyn ImageOp + Send + Sync>,
) -> anyhow::Result<RawImage> {
    let ratio = size as f64 / img.width.max(img.height) as f64;
    let new_width = img.width as f64 * ratio;
    let new_height = img.height as f64 * ratio;

    img_processor.resize(
        img,
        new_width as DimType,
        new_height as DimType,
        interface_image::Interpolation::BilinearExact,
    )
}

pub fn resize_keep_aspect_mask(
    img: Mask,
    size: u16,
    img_processor: &Arc<dyn ImageOp + Send + Sync>,
) -> anyhow::Result<Mask> {
    let ratio = size as f64 / img.width.max(img.height) as f64;
    let new_width = img.width as f64 * ratio;
    let new_height = img.height as f64 * ratio;

    img_processor.resize_mask(
        img.view(),
        new_width as usize,
        new_height as usize,
        interface_image::Interpolation::BilinearExact,
    )
}

pub fn lama_resize_image<'a>(
    image: RawImageView<'a>,
    mut mask: Mask,
    inpainting_size: u16,
    img_processor: &Arc<dyn ImageOp + Send + Sync>,
) -> anyhow::Result<(RawImageCow<'a>, Mask)> {
    let w = image.width;
    let h = image.height;
    let mut image = RawImageCow::Borrowed(image);
    if w.max(h) > inpainting_size {
        image = RawImageCow::Owned(resize_keep_aspect(
            image.view(),
            inpainting_size,
            img_processor,
        )?);
        mask = resize_keep_aspect_mask(mask, inpainting_size, img_processor)?;
    }
    Ok((image, mask))
}

pub fn lama_add_border(
    mut image: RawImage,
    mut mask: Mask,
    img_processor: &Arc<dyn ImageOp + Send + Sync>,
) -> (RawImage, Mask, u16, u16) {
    let w = image.width;
    let h = image.height;
    let pad_size = 8;
    let new_h = if !h.is_multiple_of(pad_size) {
        (pad_size - (h % pad_size)) + h
    } else {
        h
    };
    let new_w = if !w.is_multiple_of(pad_size) {
        (pad_size - (w % pad_size)) + w
    } else {
        w
    };

    if new_h != h || new_w != w {
        let temp = img_processor.add_border_wh(image.view(), new_w, new_h);
        if let RawImageCow::Owned(o) = temp {
            image = o;
        }

        let mut m = RawImage {
            data: mask.data,
            width: mask.width,
            height: mask.height,
            channels: 1,
        };
        if let RawImageCow::Owned(o) = img_processor.add_border_wh(m.view(), new_w, new_h) {
            m = o;
        }
        mask = Mask {
            data: m.data,
            width: m.width,
            height: m.height,
        };
    }
    (image, mask, new_w, new_h)
}

/// Axis-aligned bounding box in image pixel coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BBox {
    pub x: DimType,
    pub y: DimType,
    pub w: DimType,
    pub h: DimType,
}

/// Find bounding boxes of mask foreground regions for local (bbox) inpainting.
///
/// The mask is dilated by `padding` before connected-component labeling, which both
/// merges neighbouring text into a single box and grows each box by `padding` on every
/// side to provide context for the inpainter. Boxes are clamped to the image bounds.
///
/// The returned boxes only describe *where* to crop; the actual inpaint mask must still
/// be taken from the original (un-dilated) `mask`.
pub fn lama_mask_regions(mask: &Mask, padding: DimType) -> anyhow::Result<Vec<BBox>> {
    let mw = mask.width;
    let mh = mask.height;
    if mw == 0 || mh == 0 {
        return Ok(Vec::new());
    }

    let src = mask.as_opencv_mat()?;
    let ksize = (padding as i32 * 2 + 1).max(1);
    let kernel = get_structuring_element(
        MORPH_ELLIPSE,
        opencv::core::Size::new(ksize, ksize),
        Point::new(-1, -1),
    )?;
    let mut dilated = Mat::default();
    dilate(
        &src,
        &mut dilated,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value()?,
    )?;

    let mut labels = Mat::default();
    let mut stats = Mat::default();
    let mut centroids = Mat::default();
    let num_labels = connected_components_with_stats(
        &dilated,
        &mut labels,
        &mut stats,
        &mut centroids,
        8,
        CV_32S,
    )?;

    let mut boxes = Vec::new();
    // label 0 is background.
    for label in 1..num_labels {
        if *stats.at_2d::<i32>(label, CC_STAT_AREA)? <= 0 {
            continue;
        }
        let x = (*stats.at_2d::<i32>(label, CC_STAT_LEFT)?).max(0) as i64;
        let y = (*stats.at_2d::<i32>(label, CC_STAT_TOP)?).max(0) as i64;
        let w = (*stats.at_2d::<i32>(label, CC_STAT_WIDTH)?).max(0) as i64;
        let h = (*stats.at_2d::<i32>(label, CC_STAT_HEIGHT)?).max(0) as i64;

        let x0 = x.clamp(0, mw as i64);
        let y0 = y.clamp(0, mh as i64);
        let x1 = (x + w).clamp(0, mw as i64);
        let y1 = (y + h).clamp(0, mh as i64);
        if x1 <= x0 || y1 <= y0 {
            continue;
        }
        boxes.push(BBox {
            x: x0 as DimType,
            y: y0 as DimType,
            w: (x1 - x0) as DimType,
            h: (y1 - y0) as DimType,
        });
    }
    Ok(boxes)
}

/// Crop an RGB sub-image (the box must lie within the image bounds).
pub fn crop_rgb(image: RawImageView<'_>, b: &BBox) -> RawImage {
    debug_assert_eq!(image.channels, 3);
    let stride = image.width as usize * 3;
    let row_bytes = b.w as usize * 3;
    let mut data = Vec::with_capacity(b.h as usize * row_bytes);
    for row in 0..b.h as usize {
        let src_y = b.y as usize + row;
        let start = src_y * stride + b.x as usize * 3;
        data.extend_from_slice(&image.data[start..start + row_bytes]);
    }
    RawImage {
        data,
        width: b.w,
        height: b.h,
        channels: 3,
    }
}

/// Paste an RGB patch into an RGB image at the box origin (box must lie within bounds).
pub fn paste_rgb(dst: &mut RawImage, patch: &RawImage, b: &BBox) {
    debug_assert_eq!(dst.channels, 3);
    debug_assert_eq!(patch.channels, 3);
    debug_assert_eq!(patch.width, b.w);
    debug_assert_eq!(patch.height, b.h);
    let dst_stride = dst.width as usize * 3;
    let row_bytes = b.w as usize * 3;
    for row in 0..b.h as usize {
        let dst_y = b.y as usize + row;
        let dst_start = dst_y * dst_stride + b.x as usize * 3;
        let src_start = row * row_bytes;
        dst.data[dst_start..dst_start + row_bytes]
            .copy_from_slice(&patch.data[src_start..src_start + row_bytes]);
    }
}

/// Crop a single-channel mask sub-image (the box must lie within the mask bounds).
pub fn crop_mask(mask: &Mask, b: &BBox) -> Mask {
    let stride = mask.width as usize;
    let row_bytes = b.w as usize;
    let mut data = Vec::with_capacity(b.h as usize * row_bytes);
    for row in 0..b.h as usize {
        let src_y = b.y as usize + row;
        let start = src_y * stride + b.x as usize;
        data.extend_from_slice(&mask.data[start..start + row_bytes]);
    }
    Mask {
        data,
        width: b.w,
        height: b.h,
    }
}
