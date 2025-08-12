use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct UTM {
    #[serde(rename = "@UTM_WE", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub utm_we: i32,

    #[serde(rename = "@UTM_NS", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub utm_ns: i32,

    #[serde(rename = "@UTM_Zone", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub utm_zone: i32,

    #[serde(rename = "@UTM_Zone2", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub utm_zone_2: String,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
