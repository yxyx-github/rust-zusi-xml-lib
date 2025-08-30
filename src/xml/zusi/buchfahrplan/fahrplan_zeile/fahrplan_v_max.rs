use serde_helpers::default::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanVMax {
    #[serde(rename = "@vMaxTyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub v_max_typ: i32, // TODO: replace with enum?

    #[serde(rename = "@vMax", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub v_max: f32,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
