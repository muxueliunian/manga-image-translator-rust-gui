use log::info;
use textline_merge::TextBlock;

use crate::{dict::Dict, settings::TranslatorSettings, setup::Models};

impl Models {
    pub fn run_pre_dict(
        &self,
        mut textblocks: Vec<TextBlock>,
        config: &TranslatorSettings,
    ) -> anyhow::Result<Vec<TextBlock>> {
        if let Some(pre_dict) = &config.pre_dict {
            info!("Running pre-dictionary processing");
            //TODO: add caching
            let dict = Dict::try_load(pre_dict);
            for textblock in &mut textblocks {
                textblock.text = dict.apply(&textblock.text);
            }
        }
        Ok(textblocks)
    }

    pub fn run_post_dict(
        &self,
        mut textblocks: Vec<TextBlock>,
        config: &TranslatorSettings,
    ) -> anyhow::Result<Vec<TextBlock>> {
        if let Some(post_dict) = &config.post_dict {
            info!("Running post-dictionary processing");
            //TODO: add caching
            let dict = Dict::try_load(post_dict);
            for textblock in &mut textblocks {
                for value in textblock.translations.values_mut() {
                    *value = dict.apply(value);
                }
            }
        }
        Ok(textblocks)
    }
}
