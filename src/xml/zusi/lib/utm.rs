use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct UTM {
    #[serde(rename = "@UTM_WE")]
    #[serde(default)]
    #[builder(default)]
    pub utm_we: i32,

    #[serde(rename = "@UTM_NS")]
    #[serde(default)]
    #[builder(default)]
    pub utm_ns: i32,

    #[serde(rename = "@UTM_Zone")]
    #[serde(default)]
    #[builder(default)]
    pub utm_zone: i32,

    #[serde(rename = "@UTM_Zone2")]
    #[serde(default)]
    #[builder(default)]
    pub utm_zone_2: String,
}
