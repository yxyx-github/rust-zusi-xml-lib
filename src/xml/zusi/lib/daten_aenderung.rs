pub mod fahrplan_anmerkung;

use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use crate::xml::zusi::lib::daten_aenderung::fahrplan_anmerkung::FahrplanAnmerkung;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct DatenAenderung {
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

    #[serde(rename = "@Grenzlast", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
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

    #[serde(rename = "@BremsstellungZug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremsstellung_zug: Bremsstellung,

    #[serde(rename = "@FplBremsstellungTextvorgabe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_bremsstellung_textvorgabe: String,

    #[serde(rename = "FplAnmerkung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_anmerkung: Option<FahrplanAnmerkung>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}