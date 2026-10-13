use crate::xml::zusi::lib::punkt_xyz::PunktXYZ;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::lib::ereignis::Ereignis;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct SignalFrame {
    #[serde(rename = "@WeichenbaugruppeIndex", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub weichen_baugruppe_index: i32,

    #[serde(rename = "@WeichenbaugruppeNr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub weichen_baugruppe_nummer: i32,

    #[serde(rename = "@WeichenbaugruppeBeschreibung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub weichen_baugruppe_beschreibung: String,

    #[serde(rename = "@WeichenbaugruppePos0", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub weichen_baugruppe_pos_0: bool,

    #[serde(rename = "@WeichenbaugruppePos1", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub weichen_baugruppe_pos_1: bool,

    #[serde(rename = "p")]
    pub position: PunktXYZ,

    #[serde(rename = "phi")]
    pub phi: PunktXYZ,

    #[serde(rename = "sk")]
    pub sk: PunktXYZ,

    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(rename = "Ereignis", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ereignisse: Vec<Ereignis>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}