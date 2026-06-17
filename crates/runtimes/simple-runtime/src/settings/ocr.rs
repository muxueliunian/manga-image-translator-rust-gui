use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone, JsonSchema, Debug,
)]
pub enum OCR {
    MangaOcr,
    Native,
    Tesseract,
    Ctc48px,
    #[default]
    Ocr48px,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
#[serde(default)]
pub struct OCRSettings {
    /// Optical character recognition (OCR) model to use
    pub ocr: OCR,
    /// Beam search width for autoregressive OCR models (only `Ocr48px`).
    /// 1 = greedy (fastest, slightly less accurate); higher keeps more candidate
    /// sequences (slower, more robust). Default 5. Ignored by non-autoregressive models.
    pub beam_size: usize,
    /// Use bbox merge when Manga OCR inference.
    /// todo: not used
    use_mocr_merge: bool,

    #[serde(flatten)]
    pub post_processing: PostProcessingSettings,
}

impl Default for OCRSettings {
    fn default() -> Self {
        Self {
            ocr: OCR::default(),
            beam_size: 5,
            use_mocr_merge: false,
            post_processing: PostProcessingSettings::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
#[serde(default)]
pub struct PostProcessingSettings {
    /// Minimum text length of a text region
    pub min_text_length: usize,
    /// Filter regions by their text with a regex. Example usage: '.*badtext.*'
    pub filter_text: Vec<String>,
    /// Minimum probability of a text region to be considered valid. If None, uses the model default
    pub prob: f64,
}

impl Default for PostProcessingSettings {
    fn default() -> Self {
        Self {
            min_text_length: 1,
            filter_text: Vec::new(),
            prob: 0.2,
        }
    }
}
