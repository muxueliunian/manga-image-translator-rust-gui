use interface_detector::{DefaultOptions, PreprocessorOptions};
use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(Deserialize, Default)]
/// Settings for the simple runtime
pub struct Settings {
    /// Settings for the upscaler module
    pub upscaler: UpscalerSettings,
    /// Settings for the detector module
    pub detector: DetectorSettings,
    /// Settings for the OCR module
    pub ocr: OCRSettings,
    /// Settings for the inpainter module
    pub inpainter: InpainterSettings,
    /// Settings for the render module
    pub render: RenderSettings,
    /// Settings for the translator module
    pub translator: TranslatorSettings,
}
#[derive(Deserialize, Default)]
pub struct TranslatorSettings {
    pub translator: Translator,
}

#[derive(Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
pub enum OCR {
    #[default]
    MangaOcr,
    // Native,
    // Tesseract,
}
#[derive(Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Translator {
    JParaCrawlSmall,
    JParaCrawlBase,
    JParaCrawlLarge,
    Baidu,
    Caiyun,
    Deepl,
    Google,
    M2M100Small,
    M2M100Large,
    MBart,
    MyMemory,
    NLLBSmallDistilled,
    NLLBBase,
    NLLBLarge,
    Papago,
    #[default]
    Sugoi,
    Youdao,
}

#[derive(Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Detector {
    #[default]
    DBNet,
    // DBNetConvNext,
    Paddle,
    Ctd,
}

#[derive(Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Upscaler {
    Esrgan2x,
    #[default]
    Esrgan4x,
    EsrganAnime4x,
    Waifu2xCuNetArt(Option<u8>),
    Waifu2xSwinUnetArt2x(Option<u8>),
    Waifu2xSwinUnetArt4x(Option<u8>),
    Anime4k,
}
#[derive(Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Inpainter {
    #[default]
    LamaAot,
    LamaLarge,
    LamaMpe,
}

#[derive(Deserialize, Default)]
pub struct DetectorSettings {
    #[serde(default = "default_detector")]
    pub detector: Detector,
    #[serde(default = "default_preprocessor")]
    pub preprocessor: PreprocessorOptions,
    pub options: DefaultOptions,
}
#[derive(Deserialize, Default)]
pub struct OCRSettings {
    pub ocr: OCR,
    pub min_text_length: usize,
    pub filter_text: Vec<String>,
    pub filter_lang: Vec<String>,
}
#[derive(Deserialize, Default)]
pub struct InpainterSettings {}
#[derive(Deserialize, Default)]
pub struct RenderSettings {}

#[derive(Deserialize, Default, Copy, Clone)]
pub struct UpscalerSettings {
    pub upscaler: Option<Upscaler>,
    pub patch_size: Option<usize>,
    #[serde(default = "default_padding")]
    pub padding: usize,
}

fn default_padding() -> usize {
    10
}

fn default_detector() -> Detector {
    Detector::default()
}

fn default_preprocessor() -> PreprocessorOptions {
    PreprocessorOptions::default()
}
