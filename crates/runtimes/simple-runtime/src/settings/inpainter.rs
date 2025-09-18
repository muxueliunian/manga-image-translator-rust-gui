use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone, JsonSchema,
)]
pub enum Inpainter {
    #[default]
    LamaAot,
    LamaLarge,
    LamaMpe,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct InpainterSettings {
    /// Inpainting model to use
    pub inpainter: Inpainter,
    /// Size of image used for inpainting (too large will result in OOM)
    pub inpainting_size: u16,
    /// The threshold for ignoring text in non bubble areas, with valid values ranging from 1 to 50, does not ignore others. Recommendation 5 to 10. If it is too low, normal bubble areas may be ignored, and if it is too large, non bubble areas may be considered normal bubbles
    pub ignore_bubble: Option<u8>,
    /// By how much to extend the text mask to remove left-over text pixels of the original image.
    mask_dilation_offset: u32,
    /// Set the convolution kernel size of the text erasure area to completely clean up text residues"
    kernel_size: u8,
    pub furi: bool,
    /// If no ai is used for inpainting than use this color
    pub inpaint_color: [u8; 3],
}

impl Default for InpainterSettings {
    fn default() -> Self {
        Self {
            inpainter: Default::default(),
            inpainting_size: 2048,
            ignore_bubble: None,
            kernel_size: 3,
            mask_dilation_offset: 20,
            inpaint_color: [255; 3],
            furi: false,
        }
    }
}
