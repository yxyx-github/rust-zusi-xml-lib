use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::datei::Datei;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct NachfolgerElementInModul {
    #[serde(rename = "@Nr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nummer: i64, // TODO: check inconsistency

    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}