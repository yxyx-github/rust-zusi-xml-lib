use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct LoeschFahrstrasse {
    #[serde(rename = "@FahrstrName", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrassen_name: String,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}