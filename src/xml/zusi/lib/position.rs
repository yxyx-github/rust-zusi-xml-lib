use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Position {
    #[serde(rename = "@X", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub x: f32,

    #[serde(rename = "@Y", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub y: f32,

    #[serde(rename = "@Z", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub z: f32,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
