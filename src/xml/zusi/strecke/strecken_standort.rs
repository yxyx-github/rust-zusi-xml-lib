use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::punkt_xyz::PunktXYZ;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct StreckenStandort {
    #[serde(rename = "@StrInfo", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub string_info: String,

    #[serde(rename = "p", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub position: Vec<PunktXYZ>,

    #[serde(rename = "lookat", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub look_at: Vec<PunktXYZ>,

    #[serde(rename = "up", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub up: Vec<PunktXYZ>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}