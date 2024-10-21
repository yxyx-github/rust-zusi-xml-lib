use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Datei {
    #[serde(rename = "@Dateiname")]
    #[serde(default)]
    #[builder(default)]
    pub dateiname: String,

    #[serde(rename = "@inst")]
    #[serde(default)]
    #[builder(default)]
    pub inst: i32,

    #[serde(rename = "@NurInfo")]
    #[serde(default)]
    #[builder(default)]
    pub nur_info: bool,
}
