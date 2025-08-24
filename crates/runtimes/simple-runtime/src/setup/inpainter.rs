use std::collections::HashMap;

use base_util::onnx::gpu_providers;
use strum::IntoEnumIterator;

use crate::settings::Inpainter;
pub type InpainterType = Box<dyn interface_inpainter::Inpainter + Send + Sync>;

pub struct Inpainters(HashMap<Inpainter, InpainterType>);
impl Inpainters {
    pub fn get(&mut self, inpainter: Inpainter) -> &mut InpainterType {
        self.0
            .get_mut(&inpainter)
            .expect("Inpainter not registered")
    }
    pub fn new() -> Self {
        let mut items = HashMap::new();
        let providers = gpu_providers();
        for key in Inpainter::iter() {
            let inpainter = match key {
                Inpainter::LamaAot => {
                    Box::new(lama_aot::LamaLargeInpainter::new(providers.clone())) as InpainterType
                }
                Inpainter::LamaLarge => {
                    Box::new(lama_large::LamaLargeInpainter::new(providers.clone()))
                        as InpainterType
                }
                Inpainter::LamaMpe => {
                    Box::new(lama_mpe::LamaLargeInpainter::new(providers.clone())) as InpainterType
                }
            };
            items.insert(key, inpainter);
        }
        Inpainters(items)
    }
}
