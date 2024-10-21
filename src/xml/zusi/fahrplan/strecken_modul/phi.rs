use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Phi {
    #[serde(rename = "@X")]
    #[serde(default)]
    #[builder(default)]
    pub x: f64,

    #[serde(rename = "@Y")]
    #[serde(default)]
    #[builder(default)]
    pub y: f64,

    #[serde(rename = "@Z")]
    #[serde(default)]
    #[builder(default)]
    pub z: f64,
}
