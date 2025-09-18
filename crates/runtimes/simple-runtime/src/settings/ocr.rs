use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone, JsonSchema,
)]
pub enum OCR {
    MangaOcr,
    Native,
    Tesseract,
    Ctc48px,
    #[default]
    Ocr48px,
}

#[derive(Serialize, Deserialize, Default, JsonSchema)]
#[serde(default)]
pub struct OCRSettings {
    /// Optical character recognition (OCR) model to use
    pub ocr: OCR,
    /// Minimum text length of a text region
    pub min_text_length: usize,
    /// Filter regions by their text with a regex. Example usage: '.*badtext.*'
    /// todo: regex
    pub filter_text: Vec<String>,
    /// Minimum probability of a text region to be considered valid. If None, uses the model default
    /// todo: not used
    prob: Option<f32>,
    /// Use bbox merge when Manga OCR inference.
    /// todo: not used
    use_mocr_merge: bool,
}
