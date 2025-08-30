use serde_helpers::default::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct ZugdatenIndusiAnalog {
    #[serde(rename = "@IndusiZugart", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub indusi_zugart: u8, // TODO: replace with enum

    #[serde(rename = "@ZugsicherungHS", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugsicherung_hauptschalter: i32,

    #[serde(rename = "@Lufthahn", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub lufthahn: i32,

    #[serde(rename = "@PZBStoerschalter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub pzb_stoerschalter: i32,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
