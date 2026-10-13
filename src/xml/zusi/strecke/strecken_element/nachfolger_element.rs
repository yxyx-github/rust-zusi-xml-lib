use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct NachfolgerElement {
    #[serde(rename = "@Nr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nummer: i32, // TODO: check inconsistency

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}