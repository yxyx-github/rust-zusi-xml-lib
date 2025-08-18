use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct FahrplanNameRechts {
    #[serde(rename = "@FplNameText", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_name_text: String,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
