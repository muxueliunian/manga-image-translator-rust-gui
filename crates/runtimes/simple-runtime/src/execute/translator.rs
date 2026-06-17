use std::{path::Path, time::Duration};

use anyhow::{anyhow, bail};
use interface_translator::Detector;
use interface_translator::TranslationListOutput;
use log::info;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use textline_merge::TextBlock;

use crate::{
    diagnostics,
    perf::JobLogger,
    settings::{OpenAICompatibleSettings, Target, Translation, Translator, TranslatorSettings},
    setup::Models,
};

impl Models {
    pub async fn run_translators(
        &self,
        textblocks: Vec<TextBlock>,
        config: &TranslatorSettings,
        debug_path: Option<&Path>,
        logger: Option<&JobLogger>,
    ) -> anyhow::Result<Vec<TextBlock>> {
        match &config.target {
            Target::Single(items) => {
                self.run_translator_list(textblocks, items.as_slice(), config, debug_path, logger)
                    .await
            }
            Target::Selective(hash_map) => todo!("selective not implemented yet"),
        }
    }

    pub async fn run_translator_list(
        &self,
        mut textblocks: Vec<TextBlock>,
        translators: &[Translation],
        config: &TranslatorSettings,
        debug_path: Option<&Path>,
        logger: Option<&JobLogger>,
    ) -> anyhow::Result<Vec<TextBlock>> {
        assert!(!textblocks.is_empty());
        let mut textblocks_use = textblocks
            .iter_mut()
            .filter(|v| !v.skip_translate)
            .collect::<Vec<_>>();
        for tb in &textblocks_use {
            assert!(tb.translations.is_empty());
        }

        // Nothing to translate (every block was filtered out upstream).
        if textblocks_use.is_empty() {
            return Ok(textblocks);
        }

        // Per-block cache lookup. The signature pins the translator chain + settings so a
        // change to target language / model / prompt won't return stale translations.
        let signature = cache_signature(translators, config);
        let keys = textblocks_use
            .iter()
            .map(|tb| cache_key(signature, &tb.text))
            .collect::<Vec<_>>();
        let cached = self.translation_cache.get_batch(&keys).await;

        // `final_texts[i]` is the final translation for `textblocks_use[i]`, in order.
        let mut final_texts = vec![String::new(); textblocks_use.len()];
        let mut miss_indices = Vec::new();
        let mut miss_texts = Vec::new();
        for (i, hit) in cached.into_iter().enumerate() {
            match hit {
                Some(text) => final_texts[i] = text,
                None => {
                    miss_indices.push(i);
                    miss_texts.push(textblocks_use[i].text.clone());
                }
            }
        }

        if !miss_texts.is_empty() {
            let lang = self.lang_detector.detect_language(&miss_texts.join(" "));
            let mut texts = TranslationListOutput {
                text: miss_texts,
                lang,
            };
            // Run the (possibly chained) translators only on the cache misses.
            for (index, translator) in translators.iter().enumerate() {
                texts = self
                    .run_translator_item(texts, translator, config, debug_path, logger, index + 1)
                    .await?;
            }

            let mut new_entries = Vec::with_capacity(miss_indices.len());
            for (slot, &i) in miss_indices.iter().enumerate() {
                let translated = texts.text[slot].clone();
                new_entries.push((keys[i].clone(), translated.clone()));
                final_texts[i] = translated;
            }
            self.translation_cache.insert_batch(new_entries).await;
        }

        // Record the output language from the assembled translations (cached + fresh),
        // matching the previous behavior of stamping one language per block.
        let lang_str = self
            .lang_detector
            .detect_language(&final_texts.join(" "))
            .and_then(|v| v.to_name())
            .unwrap_or("unknown")
            .to_owned();
        for (i, tb) in textblocks_use.iter_mut().enumerate() {
            tb.translations
                .insert(lang_str.clone(), final_texts[i].clone());
            tb.translations
                .insert("last_trans".to_owned(), lang_str.clone());
        }
        Ok(textblocks)
    }
    pub async fn run_translator_item(
        &self,
        input: TranslationListOutput,
        translator_info: &Translation,
        config: &TranslatorSettings,
        debug_path: Option<&Path>,
        logger: Option<&JobLogger>,
        step: usize,
    ) -> anyhow::Result<TranslationListOutput> {
        info!("Run Translator: {:?}", translator_info.translator);
        let to = translator_info.target.0;
        // TODO: set fallback language in config
        let from = input.lang.ok_or(anyhow!("Failed to detect language"))?;
        let translator_name = format!("{:?}", translator_info.translator);
        let from_name = from.to_name().unwrap_or("unknown");
        let to_name = to.to_name().unwrap_or("unknown");
        diagnostics::record_translator_request(
            logger,
            debug_path,
            step,
            &translator_name,
            from_name,
            to_name,
            &input.text,
        )?;
        let started = std::time::Instant::now();

        let text_result = if translator_info.translator == Translator::OpenAICompatible {
            translate_openai_compatible(&input.text, from_name, to_name, &config.openai_compatible)
                .await
        } else {
            let translator = self.get_translator(translator_info.translator).await?;
            translator
                .translate_vec(&input.text, None, Some(from), &to)
                .await
                .map(|value| value.text)
        };
        let text = match text_result {
            Ok(text) => text,
            Err(err) => {
                diagnostics::record_translator_error(
                    logger,
                    debug_path,
                    step,
                    &translator_name,
                    from_name,
                    to_name,
                    &input.text,
                    &err.to_string(),
                    started.elapsed(),
                )?;
                return Err(err);
            }
        };

        let d_str = text.join(" ");
        let lang = self.lang_detector.detect_language(&d_str);
        diagnostics::record_translator_response(
            logger,
            debug_path,
            step,
            &translator_name,
            from_name,
            to_name,
            &input.text,
            &text,
            lang.and_then(|value| value.to_name()),
            started.elapsed(),
        )?;

        Ok(TranslationListOutput { text, lang })
    }
}

/// Stable hash of the translator chain + settings. Two requests share cached
/// translations only when this matches, so changing the target language, model,
/// prompt, or temperature transparently invalidates old entries.
fn cache_signature(translators: &[Translation], config: &TranslatorSettings) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    // serde_json is already a dependency here; serializing both gives a faithful,
    // forward-compatible signature without hand-maintaining a field list.
    if let Ok(serialized) = serde_json::to_string(&(translators, config)) {
        serialized.hash(&mut hasher);
    }
    hasher.finish()
}

/// Cache key for one source string under a given configuration signature. The unit
/// separator byte cannot appear in either part, so keys are unambiguous.
fn cache_key(signature: u64, text: &str) -> String {
    format!("{signature:016x}\u{1f}{text}")
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionChoice>,
}

#[derive(Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
}

#[derive(Deserialize)]
struct ChatCompletionMessage {
    content: String,
}

async fn translate_openai_compatible(
    texts: &[String],
    source_language: &str,
    target_language: &str,
    settings: &OpenAICompatibleSettings,
) -> anyhow::Result<Vec<String>> {
    if texts.is_empty() {
        return Ok(vec![]);
    }

    let base_url = settings
        .resolved_base_url()
        .ok_or_else(|| anyhow!("OpenAI-compatible base_url is required"))?;
    if settings.api_key.trim().is_empty() {
        bail!("OpenAI-compatible api_key is required");
    }
    if settings.model.trim().is_empty() {
        bail!("OpenAI-compatible model is required");
    }

    let numbered_texts = texts
        .iter()
        .enumerate()
        .map(|(i, text)| format!("[{}] {}", i + 1, text))
        .collect::<Vec<_>>()
        .join("\n");
    let user_prompt = settings
        .user_prompt_template
        .replace("{source_language}", source_language)
        .replace("{target_language}", target_language)
        .replace("{texts}", &numbered_texts);

    let request = ChatCompletionRequest {
        model: settings.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system",
                content: settings.system_prompt.clone(),
            },
            ChatMessage {
                role: "user",
                content: user_prompt,
            },
        ],
        temperature: settings.temperature,
        top_p: settings.top_p,
    };

    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let client = Client::builder()
        .timeout(Duration::from_secs(settings.timeout_secs.max(1)))
        .build()?;
    let response = client
        .post(url)
        .bearer_auth(settings.api_key.trim())
        .json(&request)
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        bail!("OpenAI-compatible request failed with {status}: {body}");
    }

    let response: ChatCompletionResponse = serde_json::from_str(&body)?;
    let content = response
        .choices
        .first()
        .ok_or_else(|| anyhow!("OpenAI-compatible response contained no choices"))?
        .message
        .content
        .as_str();

    parse_numbered_translations(content, texts.len())
}

fn parse_numbered_translations(content: &str, expected: usize) -> anyhow::Result<Vec<String>> {
    let re = Regex::new(r"^\s*(?:\[(\d+)\]|(\d+)[\.\):：、])\s*(.*)\s*$")?;
    let mut parsed = vec![None; expected];

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some(captures) = re.captures(line) {
            let raw_number = captures
                .get(1)
                .or_else(|| captures.get(2))
                .ok_or_else(|| anyhow!("missing translation number"))?
                .as_str();
            let number = raw_number.parse::<usize>()?;
            if number == 0 || number > expected {
                bail!("translation number {number} is outside expected range 1..={expected}");
            }
            let index = number - 1;
            if parsed[index].is_some() {
                bail!("duplicate translation number {number}");
            }
            parsed[index] = Some(
                captures
                    .get(3)
                    .map(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            );
        } else {
            bail!("unnumbered translation line: {line}");
        }
    }

    parsed
        .into_iter()
        .enumerate()
        .map(|(i, item)| item.ok_or_else(|| anyhow!("missing translation number {}", i + 1)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_numbered_translations;

    #[test]
    fn parses_bracketed_numbered_translations() {
        let parsed = parse_numbered_translations("[1] Hello\n[2] World", 2).unwrap();

        assert_eq!(parsed, vec!["Hello", "World"]);
    }

    #[test]
    fn rejects_missing_numbers() {
        let error = parse_numbered_translations("[1] Hello\n[3] Later", 3)
            .unwrap_err()
            .to_string();

        assert!(error.contains("missing translation number 2"));
    }

    #[test]
    fn rejects_duplicate_numbers() {
        let error = parse_numbered_translations("[1] Hello\n[1] Again", 1)
            .unwrap_err()
            .to_string();

        assert!(error.contains("duplicate translation number 1"));
    }

    #[test]
    fn rejects_unnumbered_lines() {
        let error = parse_numbered_translations("[1] Hello\nWorld", 1)
            .unwrap_err()
            .to_string();

        assert!(error.contains("unnumbered translation line"));
    }
}
