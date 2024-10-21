pub mod fahrzeug;
pub mod fahrplan_zeile;

use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::FahrplanZeile;
use crate::xml::zusi::buchfahrplan::fahrzeug::Fahrzeug;
use crate::xml::zusi::lib::utm::UTM;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Buchfahrplan {
    #[serde(rename = "@Gattung")]
    #[serde(default)]
    #[builder(default)]
    pub gattung: String,

    #[serde(rename = "@Nummer")]
    #[serde(default)]
    #[builder(default)]
    pub nummer: String,

    #[serde(rename = "@Zuglauf")]
    #[serde(default)]
    #[builder(default)]
    pub zuglauf: String,

    #[serde(rename = "@BR")]
    #[serde(default)]
    #[builder(default)]
    pub baureihe: String,

    #[serde(rename = "@Masse")]
    #[serde(default)]
    #[builder(default)]
    pub masse: f32,

    #[serde(rename = "@spMax")]
    #[serde(default)]
    #[builder(default)]
    pub speed_max: f32,

    #[serde(rename = "@Bremsh")]
    #[serde(default)]
    #[builder(default)]
    pub bremshundertstel: f32,

    #[serde(rename = "@MBrh")]
    #[serde(default)]
    #[builder(default)]
    pub mindest_bremshundertstel: f32,

    #[serde(rename = "@Verkehrstage")]
    #[serde(default)]
    #[builder(default)]
    pub verkehrstage: String,

    #[serde(rename = "@Grenzlast")]
    #[serde(default)]
    #[builder(default)]
    pub grenzlast: bool,

    #[serde(rename = "@Laenge")]
    #[serde(default)]
    #[builder(default)]
    pub laenge: f32,

    #[serde(rename = "@LaengeLoks")]
    #[serde(default)]
    #[builder(default)]
    pub laenge_loks: f32,

    #[serde(rename = "@WagenzugLaenge")]
    #[serde(default)]
    #[builder(default)]
    pub wagenzug_laenge: f32,

    #[serde(rename = "@kmStart")]
    #[serde(default)]
    #[builder(default)]
    pub km_start: f32,

    #[serde(rename = "@BremsstellungZug")]
    #[serde(default)]
    #[builder(default)]
    pub bremsstellung_zug: Bremsstellung,

    #[serde(rename = "@FplBremsstellungTextvorgabe")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_bremsstellung_textvorgabe: String,

    #[serde(rename = "@GNTSpalte")]
    #[serde(default)]
    #[builder(default)]
    pub gnt_spalte: bool,

    #[serde(rename = "$value")]
    pub value: Vec<BuchfahrplanValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum BuchfahrplanValue {
    #[serde(rename = "Datei_fpn")]
    DateiFpn(Datei),

    #[serde(rename = "Datei_trn")]
    DateiTrn(Datei),

    #[serde(rename = "UTM")]
    UTM(UTM),

    #[serde(rename = "Fzg")]
    Fahrzeug(Fahrzeug),

    #[serde(rename = "FplZeile")]
    FahrplanZeile(FahrplanZeile),
}
