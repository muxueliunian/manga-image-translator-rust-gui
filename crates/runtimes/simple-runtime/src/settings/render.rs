use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, JsonSchema)]
#[serde(default)]
pub struct RenderSettings {}
