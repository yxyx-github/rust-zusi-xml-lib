use serde_helpers::default::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Phi {
    #[serde(rename = "@X", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub x: f64,

    #[serde(rename = "@Y", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub y: f64,

    #[serde(rename = "@Z", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub z: f64,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
