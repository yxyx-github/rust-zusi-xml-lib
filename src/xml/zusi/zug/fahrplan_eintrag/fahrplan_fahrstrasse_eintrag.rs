use serde_helpers::default::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanFahrstrasseEintrag {
    #[serde(rename = "@FahrplanFahrstrasse", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_fahrstrasse: String,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
