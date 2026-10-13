use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrstrasseVorsignal {
    #[serde(rename = "@Ref", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub referenz: i64,

    #[serde(rename = "@FahrstrSignalSpalte", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_signal_spalte: i32,

    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}