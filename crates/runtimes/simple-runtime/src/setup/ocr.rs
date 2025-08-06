use std::collections::HashMap;

use base_util::onnx::all_providers;
use strum::IntoEnumIterator;

use crate::settings::OCR;

pub type OcrType = Box<dyn interface_ocr::Ocr + Send + Sync>;

pub struct OCRs(HashMap<OCR, OcrType>);
impl OCRs {
    pub fn get(&mut self, ocr: OCR) -> &mut OcrType {
        self.0.get_mut(&ocr).expect("Upscaler not registered")
    }
    pub fn new() -> Self {
        let mut items = HashMap::new();
        let providers = all_providers();
        for key in OCR::iter() {
            let ocr = match key {
                OCR::MangaOcr => {
                    Box::new(manga_ocr::MangaOCR::new(providers.clone(), 256)) as OcrType
                } // OCR::Native => todo!(),
                  // OCR::Tesseract => todo!(),
            };
            items.insert(key, ocr);
        }
        OCRs(items)
    }
}
