pub mod fahrzeug_zusatz_info;
pub mod fahrplan_zeile;

use crate::serde_helpers::IsDefault;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::FahrplanZeile;
use crate::xml::zusi::buchfahrplan::fahrzeug_zusatz_info::FahrzeugZusatzInfo;
use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::lib::utm::UTM;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct Buchfahrplan {
    #[serde(rename = "@Gattung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub gattung: String,

    #[serde(rename = "@Nummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nummer: String,

    #[serde(rename = "@Zuglauf", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zuglauf: String,

    #[serde(rename = "@BR", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub baureihe: String,

    #[serde(rename = "@Masse", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub masse: f32,

    #[serde(rename = "@spMax", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub speed_max: f32,

    #[serde(rename = "@Bremsh", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremshundertstel: f32,

    #[serde(rename = "@MBrh", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub mindest_bremshundertstel: f32,

    #[serde(rename = "@Verkehrstage", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub verkehrstage: String,

    #[serde(rename = "@Grenzlast", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub grenzlast: bool,

    #[serde(rename = "@Laenge", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub laenge: f32,

    #[serde(rename = "@LaengeLoks", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub laenge_loks: f32,

    #[serde(rename = "@WagenzugLaenge", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub wagenzug_laenge: f32,

    #[serde(rename = "@kmStart", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub km_start: f32,

    #[serde(rename = "@BremsstellungZug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremsstellung_zug: Bremsstellung,

    #[serde(rename = "@FplBremsstellungTextvorgabe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_bremsstellung_textvorgabe: String,

    #[serde(rename = "@GNTSpalte", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub gnt_spalte: bool,

    #[serde(rename = "Datei_fpn")]
    pub datei_fpn: Datei,

    #[serde(rename = "Datei_trn")]
    pub datei_trn: Datei,

    #[serde(rename = "UTM")]
    pub utm: UTM,

    #[serde(rename = "Fzg", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrzeug_info: Option<FahrzeugZusatzInfo>,

    #[serde(rename = "FplZeile", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_zeilen: Vec<FahrplanZeile>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}