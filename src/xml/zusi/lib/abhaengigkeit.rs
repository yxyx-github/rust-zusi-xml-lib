use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Abhaengigkeit {
    #[serde(rename = "@AbhOperator")]
    #[serde(default)]
    #[builder(default)]
    pub abhaengigkeit_operator: u8, // TODO: replace with enum

    #[serde(rename = "@AbhBedingung")]
    #[serde(default)]
    #[builder(default)]
    pub abhaengigkeit_bedingung: u8, // TODO: replace with enum

    #[serde(rename = "@AbhParameter")]
    #[serde(default)]
    #[builder(default)]
    pub abhaengigkeit_parameter: f32,

    #[serde(rename = "@AbhAndererZug")]
    #[serde(default)]
    #[builder(default)]
    pub abhaengigkeit_anderer_zug: String,
}
