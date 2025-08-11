use std::collections::{HashMap, HashSet};

use interface_detector::{DefaultOptions, PreprocessorOptions};
use interface_translator::Language;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
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
#[derive(Serialize, Deserialize, Default)]
pub struct TranslatorSettings {
    pub translator: Translator,
}

// #[derive(Deserialize)]
// #[serde(untagged)]
pub enum Target {
    Single(SingleOrMultiple),
    Selective(HashMap<Option<Language>, SingleOrMultiple>),
}

impl Target {
    pub fn validate(&self) -> Option<&'static str> {
        match self {
            Target::Single(_) => None,
            Target::Selective(hash_map) => {
                if hash_map.get(&None).is_none() {
                    return Some("no default");
                };
                for mut key in hash_map.keys().cloned() {
                    let mut keys_used = HashSet::new();
                    loop {
                        let value = hash_map.get(&key);
                        let value = match value {
                            Some(v) => v,
                            None => return None,
                        };
                        let v = keys_used.insert(key);
                        if !v {
                            return Some("loop detected");
                        }
                        let next = match value {
                            SingleOrMultiple::Single(translation) => translation.target,
                            SingleOrMultiple::Multiple(translations) => {
                                if translations.is_empty() {
                                    return Some("empty array");
                                }
                                translations.last().unwrap().target
                            }
                        };
                        key = Some(next);
                    }
                }
                None
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
// #[serde(untagged)]
pub enum SingleOrMultiple {
    Single(Translation),
    Multiple(Vec<Translation>),
}

#[derive(Hash, Eq, PartialEq)]
pub struct Translation {
    translator: Translator,
    target: Language,
}

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
pub enum OCR {
    #[default]
    MangaOcr,
    // Native,
    // Tesseract,
}
#[derive(Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Detector {
    #[default]
    DBNet,
    // DBNetConvNext,
    Paddle,
    Ctd,
}

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
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
#[derive(Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Inpainter {
    #[default]
    LamaAot,
    LamaLarge,
    LamaMpe,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DetectorSettings {
    pub detector: Detector,
    pub preprocessor: PreprocessorOptions,
    pub options: DefaultOptions,
}
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OCRSettings {
    pub ocr: OCR,
    pub min_text_length: usize,
    pub filter_text: Vec<String>,
    pub filter_lang: Vec<String>,
}
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InpainterSettings {}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RenderSettings {}

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[serde(default)]
pub struct UpscalerSettings {
    pub upscaler: Option<Upscaler>,
    pub patch_size: Option<usize>,
    pub padding: usize,
}
