use std::collections::HashMap;

use base_util::onnx::all_providers;
use strum::IntoEnumIterator;

use crate::settings::Detector;
pub type DetectorType = Box<dyn interface_detector::Detector + Send + Sync>;

pub struct Detectors(HashMap<Detector, DetectorType>);
impl Detectors {
    pub fn get(&mut self, detector: Detector) -> &mut DetectorType {
        self.0.get_mut(&detector).expect("Detector not registered")
    }
    pub fn new() -> Self {
        let mut items = HashMap::new();
        for detector_key in Detector::iter() {
            let detector = match detector_key {
                Detector::DBNet => {
                    Box::new(dbnet::DbNetDetector::new(all_providers(), false)) as DetectorType
                }
                // Detector::DBNetConvNext => todo!(),
                Detector::Paddle => {
                    Box::new(paddle::PaddleDetector::new(all_providers())) as DetectorType
                }
                Detector::Ctd => Box::new(ctd::CtdDetector::new(all_providers())) as DetectorType,
            };
            items.insert(detector_key, detector);
        }
        Detectors(items)
    }
}
