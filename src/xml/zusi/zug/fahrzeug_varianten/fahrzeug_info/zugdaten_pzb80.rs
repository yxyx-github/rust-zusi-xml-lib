use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct ZugdatenPZB80 {
    #[serde(rename = "@IndusiZugart", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub indusi_zugart: u8, // TODO: replace with enum

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
