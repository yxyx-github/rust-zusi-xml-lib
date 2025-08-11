use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct Datei {
    #[serde(rename = "@Dateiname")]
    pub dateiname: String,

    #[serde(rename = "@inst", default)]
    #[builder(default)]
    pub inst: i32,

    #[serde(rename = "@NurInfo", default)]
    #[builder(default)]
    pub nur_info: bool,

    #[serde(flatten)]
    #[builder(default)]
    pub unknown: HashMap<String, String>,
}
