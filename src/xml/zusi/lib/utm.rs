use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct UTM {
    #[serde(rename = "@UTM_WE", default)]
    #[builder(default)]
    pub utm_we: i32,

    #[serde(rename = "@UTM_NS", default)]
    #[builder(default)]
    pub utm_ns: i32,

    #[serde(rename = "@UTM_Zone", default)]
    #[builder(default)]
    pub utm_zone: i32,

    #[serde(rename = "@UTM_Zone2", default)]
    #[builder(default)]
    pub utm_zone_2: String,

    #[serde(flatten)]
    #[builder(default)]
    pub unknown: HashMap<String, String>,
}
