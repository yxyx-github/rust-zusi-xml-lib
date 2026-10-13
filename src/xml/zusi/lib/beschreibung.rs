use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Beschreibung {
    #[serde(rename = "@Beschreibung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub beschreibung: String,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}