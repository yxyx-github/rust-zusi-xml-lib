use serde_helpers::default::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanKm {
    #[serde(rename = "@km", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub km: f32,

    #[serde(rename = "@FplSprung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_sprung: bool,

    #[serde(rename = "@FplkmNeu", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_km_neu: f32,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
