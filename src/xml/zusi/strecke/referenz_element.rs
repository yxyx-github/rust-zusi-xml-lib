use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct ReferenzElement {
    #[serde(rename = "@ReferenzNr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub referenz_nummer: i32,

    #[serde(rename = "@StrElement", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub strecken_element: i32,

    #[serde(rename = "@StrNorm", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub strecken_norm: bool,

    #[serde(rename = "@RefTyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub referenz_typ: u8, // TODO: replace with enum

    #[serde(rename = "@Info", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub info: String,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}