use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrstrasseSignal {
    #[serde(rename = "@Ref", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub referenz: i64,

    #[serde(rename = "@FahrstrSignalZeile", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_signal_zeile: i32,

    #[serde(rename = "@FahrstrSignalErsatzsignal", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_signal_ersatz_signal: bool,

    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}