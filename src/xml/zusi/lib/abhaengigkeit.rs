use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct Abhaengigkeit {
    #[serde(rename = "@AbhOperator", default)]
    #[builder(default)]
    pub abhaengigkeit_operator: u8, // TODO: replace with enum

    #[serde(rename = "@AbhBedingung", default)]
    #[builder(default)]
    pub abhaengigkeit_bedingung: u8, // TODO: replace with enum

    #[serde(rename = "@AbhParameter", default)]
    #[builder(default)]
    pub abhaengigkeit_parameter: f32,

    #[serde(rename = "@AbhAndererZug", default)]
    #[builder(default)]
    pub abhaengigkeit_anderer_zug: String,

    #[serde(flatten)]
    #[builder(default)]
    pub unknown: HashMap<String, String>,
}
