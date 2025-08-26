use crate::serde_helpers::IsDefault;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanIcon {
    #[serde(rename = "@FplIconNr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_icon_nummer: i32, // TODO: replace with enum?

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
