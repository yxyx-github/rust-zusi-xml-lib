use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::serde_helpers::IsDefault;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct AutorEintrag {
    #[serde(rename = "@AutorID", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_id: i32,

    #[serde(rename = "@AutorName", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_name: String,

    #[serde(rename = "@AutorEmail", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_email: String,

    #[serde(rename = "@AutorAufwand", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_aufwand: f32,

    #[serde(rename = "@AutorAufwandPunkte", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_aufwand_punkte: f32,

    #[serde(rename = "@AutorAufwandStunden", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_aufwand_stunden: f32,

    #[serde(rename = "@AutorLizenz", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_lizenz: u8, // TODO: replace with enum

    #[serde(rename = "@AutorBeschreibung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_beschreibung: String,

    #[serde(flatten)]
    #[builder(default)]
    pub unknown: HashMap<String, String>,
}