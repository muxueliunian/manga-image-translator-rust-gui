use std::{io::Cursor, ops::Deref};

use image::{
    guess_format, write_buffer_with_format, DynamicImage, EncodableLayout, ImageBuffer,
    ImageFormat, Pixel, PixelWithColorType,
};
use interface_image::RawImage;
use textline_merge::TextBlock;
pub struct Export {
    img: Image,
    overlay: Image,
    pub blocks: Vec<TextBlock>,
}

impl Image {
    pub fn load(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 17 {
            return None;
        }
        let width = u16::from_le_bytes(data[0..2].try_into().ok()?);
        let height = u16::from_le_bytes(data[2..4].try_into().ok()?);
        let raw = data[4] != 0;
        let data_len = u64::from_le_bytes(data[5..13].try_into().ok()?) as usize;

        if data.len() < 13 + data_len {
            return None;
        }

        let image_data = data[13..13 + data_len].to_vec();
        let offset = 13 + data_len;

        Some((
            Self {
                width,
                height,
                raw,
                data: image_data,
            },
            offset,
        ))
    }

    pub fn export(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.width.to_le_bytes());
        bytes.extend(self.height.to_le_bytes());
        bytes.push(if self.raw { 1 } else { 0 });
        bytes.extend((self.data.len() as u64).to_le_bytes());
        bytes.extend(self.data);
        bytes
    }
}

impl Export {
    pub fn load(data: Vec<u8>) -> Option<Self> {
        let skip = "mit-rust:".len() + 4;
        let data = &data[skip..];
        let (bg, off) = Image::load(data)?;
        let data = &data[off..];
        let (overlay, off) = Image::load(data)?;
        let data = &data[off..];
        if data.len() < 8 {
            return None;
        }

        let count = u64::from_le_bytes(data[0..8].try_into().ok()?) as usize;
        let mut offset = 8;
        let mut blocks = Vec::with_capacity(count);

        for _ in 0..count {
            let (block, used) = TextBlock::load(&data[offset..])?;
            offset += used;
            blocks.push(block);
        }
        assert_eq!(data.len(), offset);

        Some(Self {
            img: bg,
            overlay,
            blocks,
        })
    }

    pub fn export(self) -> Vec<u8> {
        let mut buffer = b"mit-rust:".to_vec();
        buffer.extend(2_u32.to_le_bytes());
        buffer.extend(self.img.export());
        buffer.extend(self.overlay.export());
        buffer.extend((self.blocks.len() as u64).to_le_bytes());
        for patch in self.blocks {
            buffer.extend(patch.export())
        }
        buffer
    }
}

fn convert<P: Pixel + PixelWithColorType, Container>(
    img: &ImageBuffer<P, Container>,
    format: ImageFormat,
) -> Vec<u8>
where
    [P::Subpixel]: EncodableLayout,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
    let buf = &img.as_raw()[..(img.width() * img.height()) as usize];
    write_buffer_with_format(
        &mut cursor,
        buf.as_bytes(),
        img.width(),
        img.height(),
        <P as PixelWithColorType>::COLOR_TYPE,
        format,
    )
    .unwrap();

    cursor.into_inner()
}

impl Export {
    pub fn new(
        raw: DynamicImage,
        inpainted: DynamicImage,
        blocks: Vec<TextBlock>,
        format: Option<ImageFormat>,
    ) -> Self {
        let raw_data_fn = |data: &DynamicImage| match format {
            Some(format) => match data {
                DynamicImage::ImageLuma8(img) => convert(img, format),
                DynamicImage::ImageLumaA8(img) => convert(img, format),
                DynamicImage::ImageRgb8(img) => convert(img, format),
                DynamicImage::ImageRgba8(img) => convert(img, format),
                DynamicImage::ImageLuma16(img) => convert(img, format),
                DynamicImage::ImageLumaA16(img) => convert(img, format),
                DynamicImage::ImageRgb16(img) => convert(img, format),
                DynamicImage::ImageRgba16(img) => convert(img, format),
                DynamicImage::ImageRgb32F(img) => convert(img, format),
                DynamicImage::ImageRgba32F(img) => convert(img, format),
                _ => unimplemented!("not implemented yet"),
            },
            None => data.as_bytes().to_vec(),
        };
        let raw_data = raw_data_fn(&raw);
        let d = raw_data_fn(&inpainted);
        Self {
            img: Image {
                width: raw.width() as u16,
                height: raw.height() as u16,
                data: raw_data,
                raw: format.is_none(),
            },
            overlay: Image {
                width: inpainted.width() as u16,
                height: inpainted.height() as u16,
                data: d,
                raw: format.is_none(),
            },

            blocks,
        }
    }

    pub fn get_image(&self) -> RawImage {
        (&self.img).into()
    }

    pub fn get_overlay(&self) -> RawImage {
        (&self.overlay).into()
    }
}

pub struct Image {
    width: u16,
    height: u16,
    data: Vec<u8>,
    raw: bool,
}

impl From<&Image> for RawImage {
    fn from(value: &Image) -> Self {
        if value.raw {
            RawImage {
                data: value.data.clone(),
                width: value.width,
                height: value.height,
                channels: (value.data.len() / (value.width as usize * value.height as usize)) as u8,
            }
        } else {
            let img =
                image::load(Cursor::new(&value.data), guess_format(&value.data).unwrap()).unwrap();
            let w = img.width() as u16;
            let h = img.height() as u16;
            let (data, channels) = match img {
                DynamicImage::ImageLuma8(image_buffer) => (image_buffer.into_raw(), 1),
                DynamicImage::ImageLumaA8(image_buffer) => (image_buffer.into_raw(), 2),
                DynamicImage::ImageRgb8(image_buffer) => (image_buffer.into_raw(), 3),
                DynamicImage::ImageRgba8(image_buffer) => (image_buffer.into_raw(), 4),
                DynamicImage::ImageLuma16(image_buffer) => {
                    let img = DynamicImage::from(image_buffer).to_luma8();
                    (img.into_raw(), 1)
                }
                DynamicImage::ImageLumaA16(image_buffer) => {
                    let img = DynamicImage::from(image_buffer).to_luma_alpha8();
                    (img.into_raw(), 2)
                }
                DynamicImage::ImageRgb16(image_buffer) => {
                    let img = DynamicImage::from(image_buffer).to_rgb8();
                    (img.into_raw(), 3)
                }
                DynamicImage::ImageRgba16(image_buffer) => {
                    let img = DynamicImage::from(image_buffer).to_rgba8();
                    (img.into_raw(), 4)
                }
                DynamicImage::ImageRgb32F(image_buffer) => {
                    let img = DynamicImage::from(image_buffer).to_rgb8();
                    (img.into_raw(), 3)
                }
                DynamicImage::ImageRgba32F(image_buffer) => {
                    let img = DynamicImage::from(image_buffer).to_rgba8();
                    (img.into_raw(), 4)
                }
                _ => unreachable!(),
            };
            RawImage {
                data,
                width: w,
                height: h,
                channels,
            }
        }
    }
}

impl Image {
    pub fn new(width: u16, height: u16, data: Vec<u8>, raw: bool) -> Self {
        Image {
            width,
            height,
            data,
            raw,
        }
    }
}
