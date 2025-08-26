use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Abhaengigkeit {
    #[serde(rename = "@AbhOperator", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeit_operator: u8, // TODO: replace with enum

    #[serde(rename = "@AbhBedingung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeit_bedingung: u8, // TODO: replace with enum

    #[serde(rename = "@AbhParameter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeit_parameter: f32,

    #[serde(rename = "@AbhAndererZug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeit_anderer_zug: String,

    #[serde(rename = "@ETCSModus", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_modus: u8, // TODO: replace with enum

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
