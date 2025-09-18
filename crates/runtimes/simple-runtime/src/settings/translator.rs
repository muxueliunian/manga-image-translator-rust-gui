use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use interface_translator::Language;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use strum_macros::EnumIter;

#[derive(
    Serialize, Deserialize, Default, EnumIter, Hash, PartialEq, Eq, Copy, Clone, JsonSchema,
)]
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

#[derive(Serialize, Deserialize, Default, JsonSchema)]
pub struct TranslatorSettings {
    pub translator: Translator,
    pub target: Target,
    /// Filters out languages that should not be translated
    pub filter_lang: Vec<String>,
    pub pre_dict: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Target {
    Single(SingleOrMultiple),
    Selective(HashMap<Option<LanguageWrapper>, SingleOrMultiple>),
}

impl Default for Target {
    fn default() -> Self {
        Target::Single(SingleOrMultiple::Single(Translation {
            translator: Translator::default(),
            target: LanguageWrapper(Language::English),
        }))
    }
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
                                translations
                                    .last()
                                    .expect("translations should not be empty")
                                    .target
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

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SingleOrMultiple {
    Single(Translation),
    Multiple(Vec<Translation>),
}

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Translation {
    translator: Translator,
    target: LanguageWrapper,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct LanguageWrapper(Language);

impl JsonSchema for LanguageWrapper {
    fn schema_name() -> Cow<'static, str> {
        "LanguageWrapper".into()
    }

    fn schema_id() -> Cow<'static, str> {
        concat!(module_path!(), "::", "LanguageWrapper").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut map = serde_json::Map::new();
            map.insert("type".into(), "string".into());
            map.insert(
                "enum".into(),
                serde_json::Value::Array({
                    let mut enum_values = Vec::new();
                    enum_values.push(to_enum_schema("cht", "Chinese Traditional"));
                    enum_values.push(to_enum_schema("chs", "Chinese Simplified"));
                    for lang in Language::all() {
                        let name = lang.to_name().unwrap();
                        if let Some(code) = lang.to_639_1() {
                            enum_values.push(to_enum_schema(code, name));
                        }
                        if let Some(code) = lang.to_639_3() {
                            enum_values.push(to_enum_schema(code, name));
                        }
                    }
                    enum_values
                }),
            );
            schemars::Schema::from(map)
        }
    }
}

fn to_enum_schema(name: &str, desc: &str) -> Value {
    use schemars::_private::{
        get_title_and_description, insert_metadata_property_if_nonempty, new_unit_enum_variant,
    };
    let mut schema = new_unit_enum_variant(name);
    let (title, desc): (&str, &str) = get_title_and_description(desc);

    insert_metadata_property_if_nonempty(&mut schema, "title", title);
    insert_metadata_property_if_nonempty(&mut schema, "description", desc);
    schema.to_value()
}

impl<'de> Deserialize<'de> for LanguageWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EnumVisitor;

        impl<'de> serde::de::Visitor<'de> for EnumVisitor {
            type Value = LanguageWrapper;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a string matching one of the enum variants")
            }

            fn visit_str<E>(self, value: &str) -> Result<LanguageWrapper, E>
            where
                E: serde::de::Error,
            {
                let lang = value.trim().to_lowercase();
                if lang == "cht" {
                    return Ok(LanguageWrapper(Language::ChineseTraditional));
                } else if lang == "chs" {
                    return Ok(LanguageWrapper(Language::Chinese));
                }
                (if lang.len() == 2 {
                    Language::from_639_1(&lang)
                } else {
                    Language::from_639_3(&lang)
                })
                .map(LanguageWrapper)
                .ok_or_else(|| E::custom(format!("invalid lang code: \"{}\"", value)))
            }
        }

        deserializer.deserialize_str(EnumVisitor)
    }
}

impl Serialize for LanguageWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str = self
            .0
            .to_639_1()
            .or(self.0.to_639_3())
            .unwrap_or_else(|| self.0.to_name().unwrap());
        serializer.serialize_str(str)
    }
}
