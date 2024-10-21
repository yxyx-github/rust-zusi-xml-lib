use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(rename = "@X")]
    #[serde(default)]
    #[builder(default)]
    pub x: f32,

    #[serde(rename = "@Y")]
    #[serde(default)]
    #[builder(default)]
    pub y: f32,

    #[serde(rename = "@Z")]
    #[serde(default)]
    #[builder(default)]
    pub z: f32,
}
