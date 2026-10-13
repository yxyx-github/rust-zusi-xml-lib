use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct HauptsignalBegriff {
    #[serde(rename = "@Beschreibung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub hauptsignal_geschwindigkeit: f32,

    #[serde(rename = "@FahrstrTyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrassen_typ: i64, // TODO: replace with enum?

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}