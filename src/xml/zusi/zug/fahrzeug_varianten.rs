pub mod fahrzeug_info;

use serde_helpers::with::bool_as_int::bool_as_int_format;
use serde_helpers::default::IsDefault;
use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::FahrzeugInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrzeugVarianten {
    #[serde(rename = "@Bezeichnung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bezeichnung: String,

    #[serde(rename = "@PerZufallUebernehmen", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub per_zufall_uebernehmen: bool,

    #[serde(rename = "@Gattung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub gattung: String,

    #[serde(rename = "@ZugTypVorgeben", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_typ_vorgeben: bool,

    #[serde(rename = "@Zugtyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_typ: u8, // TODO: replace with enum

    #[serde(rename = "@MBrh", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub mindest_bremshundertstel: f32,

    #[serde(rename = "@FplMasse", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_masse: f32,

    #[serde(rename = "@FplZuglaenge", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_zug_laenge: f32,

    #[serde(rename = "@BremsstellungZug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremsstellung_zug: Bremsstellung,

    #[serde(rename = "@FplBremsstellungTextvorgabe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_bremsstellung_textvorgabe: String,

    #[serde(rename = "@spZugNiedriger", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub speed_zug_niedriger: f32,

    #[serde(rename = "@ZufallsWert", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zufalls_wert: f32,

    #[serde(rename = "@FzgPosition", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrzeug_position: i32,

    #[serde(rename = "FahrzeugInfo", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrzeug_infos: Vec<FahrzeugInfo>,

    #[serde(rename = "FahrzeugVarianten", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrzeug_varianten: Vec<FahrzeugVarianten>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}