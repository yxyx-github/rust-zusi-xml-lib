use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct Datei {
    #[serde(rename = "@Dateiname", default, skip_serializing_if = "IsDefault::is_default")]
    pub dateiname: String,

    #[serde(rename = "@inst", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub inst: i32,

    #[serde(rename = "@NurInfo", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nur_info: bool,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
