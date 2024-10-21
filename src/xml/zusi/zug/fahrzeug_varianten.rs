mod fahrzeug_info;

use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrzeugVarianten {
    #[serde(rename = "@Bezeichnung")]
    #[serde(default)]
    #[builder(default)]
    pub bezeichnung: String,

    #[serde(rename = "@PerZufallUebernehmen")]
    #[serde(default)]
    #[builder(default)]
    pub per_zufall_uebernehmen: bool,

    #[serde(rename = "@Gattung")]
    #[serde(default)]
    #[builder(default)]
    pub gattung: String,

    #[serde(rename = "@ZugTypVorgeben")]
    #[serde(default)]
    #[builder(default)]
    pub zug_typ_vorgeben: bool,

    #[serde(rename = "@Zugtyp")]
    #[serde(default)]
    #[builder(default)]
    pub zug_typ: u8, // TODO: replace with enum

    #[serde(rename = "@MBrh")]
    #[serde(default)]
    #[builder(default)]
    pub mindest_bremshundertstel: f32,

    #[serde(rename = "@FplMasse")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_masse: f32,

    #[serde(rename = "@FplZuglaenge")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_zug_laenge: f32,

    #[serde(rename = "@BremsstellungZug")]
    #[serde(default)]
    #[builder(default)]
    pub bremsstellung_zug: Bremsstellung,

    #[serde(rename = "@FplBremsstellungTextvorgabe")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_bremsstellung_textvorgabe: String,

    #[serde(rename = "@spZugNiedriger")]
    #[serde(default)]
    #[builder(default)]
    pub speed_zug_niedriger: f32,

    #[serde(rename = "@ZufallsWert")]
    #[serde(default)]
    #[builder(default)]
    pub zufalls_wert: f32,

    #[serde(rename = "@FzgPosition")]
    #[serde(default)]
    #[builder(default)]
    pub fahrzeug_position: i32,

    #[serde(rename = "$value")]
    pub value: Vec<FahrzeugVariantenValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrzeugVariantenValue {
    #[serde(rename = "FahrzeugInfo")]
    FahrzeugInfo(),
}
