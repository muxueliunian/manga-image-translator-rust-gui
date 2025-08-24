use std::fs::read_to_string;

use log::{error, info};
use regex::Regex;

pub struct Dict {
    entries: Vec<(Regex, String, usize)>,
}
impl Dict {
    pub fn apply(&self, original_text: &str) -> String {
        let mut text = original_text.to_owned();
        for (pattern, value, line_number) in &self.entries {
            let t = pattern.replace_all(value, original_text);
            if original_text != t {
                info!("Line {line_number}: Replaced \"{original_text}\" with \"{text}\" using value \"{value}\"");
                text = t.to_string();
            }
        }
        text
    }

    pub fn try_load(content: &str) -> Self {
        if content.contains("\n") {
            Self::new(content)
        } else {
            Self::new(&match read_to_string(content) {
                Ok(v) => v,
                Err(_) => {
                    error!("Failed to read dict from: {}", content);
                    "".to_owned()
                }
            })
        }
    }
    pub fn new(content: &str) -> Self {
        let entries = content
            .lines()
            .enumerate()
            .map(|v| (v.0 + 1, v.1.trim()))
            .filter(|v| v.1.is_empty() || v.1.starts_with("#") || v.1.starts_with("//"))
            .map(|v| {
                (
                    v.0,
                    v.1.split("#")
                        .next()
                        .unwrap_or_default()
                        .split("//")
                        .next()
                        .unwrap_or_default()
                        .split_whitespace(),
                )
            })
            .filter_map(|(line_number, mut line)| {
                let f = line.next().unwrap();
                let s = line.next();
                let t = line.next();
                let pattern = Regex::new(f).unwrap();
                match (s, t) {
                    (None, None) => {
                        // If there is only the left part, the right part defaults to an empty string, meaning delete the left part
                        Some((pattern, "".to_string(), line_number))
                    }
                    (Some(s), None) => Some((pattern, s.to_string(), line_number)),
                    (_, Some(_)) => {
                        error!("Invalid dictionary entry at line {line_number}");
                        None
                    }
                }
            })
            .collect();
        Self { entries }
    }
}
