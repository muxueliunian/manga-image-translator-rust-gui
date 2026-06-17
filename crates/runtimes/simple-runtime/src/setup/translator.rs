use std::{collections::HashMap, env, sync::Arc};

use interface_translator::{AsyncTranslator, ComputeType};
use tokio::sync::Mutex;

use crate::settings::Translator;
pub type TranslatorType = Arc<dyn AsyncTranslator + Send + Sync>;

/// Translators are lazily initialized and shared. Storing them behind an async mutex
/// lets the whole pipeline run with `&self`, so a single `Models` instance can serve
/// concurrent images without duplicating ONNX sessions (and the VRAM they hold).
pub struct Translators(Mutex<HashMap<Translator, TranslatorType>>);

async fn create_papago() -> Option<TranslatorType> {
    Some(Arc::new(
        interface_translator::PapagoTranslator::new(false)
            .await
            .ok()?,
    ) as TranslatorType)
}

use interface_translator::{
    BaiduTranslator, CaiyunTranslator, DeeplTranslator, GoogleTranslator, YoudaoTranslator,
};
use log::{info, warn};

pub fn create_baidu_translator() -> Option<TranslatorType> {
    let app_id = env::var("BAIDU_APP_ID").ok();
    let secret_key = env::var("BAIDU_SECRET_KEY").ok();

    match (&app_id, &secret_key) {
        (Some(app_id), Some(secret_key)) => {
            Some(Arc::new(BaiduTranslator::new(&app_id, &secret_key)) as TranslatorType)
        }
        _ => {
            if app_id.is_none() {
                warn!("BAIDU_APP_ID not set");
            }
            if secret_key.is_none() {
                warn!("BAIDU_SECRET_KEY not set");
            }
            None
        }
    }
}

pub fn create_caiyun_translator() -> Option<TranslatorType> {
    match env::var("CAIYUN_TOKEN") {
        Ok(token) => Some(Arc::new(CaiyunTranslator::new(
            token,
            "manga-image-translator".to_string(),
        ))),
        Err(_) => {
            warn!("CAIYUN_TOKEN not set");
            None
        }
    }
}

pub fn create_deepl_translator() -> Option<TranslatorType> {
    match env::var("DEEPL_AUTH_KEY") {
        Ok(key) => Some(Arc::new(DeeplTranslator::new(key))),
        Err(_) => {
            warn!("DEEPL_AUTH_KEY not set");
            None
        }
    }
}

pub fn create_google_translator() -> Option<TranslatorType> {
    match env::var("GOOGLE_API_KEY") {
        Ok(key) => Some(Arc::new(GoogleTranslator::new(key))),
        Err(_) => {
            warn!("GOOGLE_API_KEY not set");
            None
        }
    }
}

pub fn create_youdao_translator() -> Option<TranslatorType> {
    let app_key = env::var("YOUDAO_APP_KEY").ok();
    let secret_key = env::var("YOUDAO_SECRET_KEY").ok();

    match (&app_key, &secret_key) {
        (Some(app_key), Some(secret_key)) => Some(Arc::new(YoudaoTranslator::new(
            app_key.to_owned(),
            secret_key.to_owned(),
        )) as TranslatorType),
        _ => {
            if app_key.is_none() {
                warn!("YOUDAO_APP_KEY not set");
            }
            if secret_key.is_none() {
                warn!("YOUDAO_SECRET_KEY not set");
            }
            None
        }
    }
}
impl Translators {
    pub async fn get(&self, translator: Translator, cuda: bool) -> anyhow::Result<TranslatorType> {
        let mut guard = self.0.lock().await;
        if !guard.contains_key(&translator) {
            info!("Lazy initializing translator: {translator:?}");
            if let Some(item) = create_translator(translator, cuda).await {
                guard.insert(translator, item);
            }
        }

        guard.get(&translator).cloned().ok_or_else(|| {
            anyhow::anyhow!(
                "Translator {translator:?} is not available. Check environment variables or translator settings."
            )
        })
    }

    pub async fn new(_cuda: bool) -> Self {
        Translators(Mutex::new(HashMap::new()))
    }
}

async fn create_translator(key: Translator, cuda: bool) -> Option<TranslatorType> {
    match key {
        Translator::JParaCrawlSmall => {
            Some(Arc::new(interface_translator::JParaCrawlTranslator::new(
                false,
                cuda,
                ComputeType::DEFAULT,
                interface_translator::JParaCrawlSize::Small,
            )) as TranslatorType)
        }
        Translator::JParaCrawlBase => {
            Some(Arc::new(interface_translator::JParaCrawlTranslator::new(
                false,
                cuda,
                ComputeType::DEFAULT,
                interface_translator::JParaCrawlSize::Base,
            )) as TranslatorType)
        }
        Translator::JParaCrawlLarge => {
            Some(Arc::new(interface_translator::JParaCrawlTranslator::new(
                false,
                cuda,
                ComputeType::DEFAULT,
                interface_translator::JParaCrawlSize::Large,
            )) as TranslatorType)
        }
        Translator::Baidu => create_baidu_translator(),
        Translator::Caiyun => create_caiyun_translator(),
        Translator::Deepl => create_deepl_translator(),
        Translator::Google => create_google_translator(),
        Translator::M2M100Small => Some(Arc::new(interface_translator::M2M100Translator::new(
            cuda,
            ComputeType::DEFAULT,
            interface_translator::M2M100Size::Small,
        )) as TranslatorType),
        Translator::M2M100Large => Some(Arc::new(interface_translator::M2M100Translator::new(
            cuda,
            ComputeType::DEFAULT,
            interface_translator::M2M100Size::Large,
        )) as TranslatorType),
        Translator::MyMemory => {
            Some(Arc::new(interface_translator::MyMemoryTranslator::new()) as TranslatorType)
        }
        Translator::NLLBSmallDistilled => Some(Arc::new(interface_translator::NLLBTranslator::new(
            cuda,
            ComputeType::DEFAULT,
            interface_translator::NLLBSize::SmallDistilled,
        )) as TranslatorType),
        Translator::NLLBBase => Some(Arc::new(interface_translator::NLLBTranslator::new(
            cuda,
            ComputeType::DEFAULT,
            interface_translator::NLLBSize::Base,
        )) as TranslatorType),
        Translator::NLLBLarge => Some(Arc::new(interface_translator::NLLBTranslator::new(
            cuda,
            ComputeType::DEFAULT,
            interface_translator::NLLBSize::Large,
        )) as TranslatorType),
        Translator::Papago => create_papago().await,
        Translator::OpenAICompatible => None,
        Translator::Sugoi => Some(Arc::new(interface_translator::SugoiTranslator::new(
            cuda,
            ComputeType::DEFAULT,
        )) as TranslatorType),
        Translator::Youdao => create_youdao_translator(),
        Translator::MBart => Some(Arc::new(interface_translator::MBart50Translator::new(
            cuda,
            ComputeType::DEFAULT,
        )) as TranslatorType),
    }
}
