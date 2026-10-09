use crate::xml::zusi::lib::daten_aenderung::DatenAenderung;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanZugParameter {
    #[serde(rename = "DatenAenderung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub daten_aenderung: Option<DatenAenderung>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}