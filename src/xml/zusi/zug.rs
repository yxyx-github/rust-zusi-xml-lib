pub mod aufgleis_referenz;
pub mod fahrplan_eintrag;
pub mod fahrzeug_varianten;

use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::zug::aufgleis_referenz::AufgleisReferenz;
use crate::xml::zusi::zug::fahrplan_eintrag::FahrplanEintrag;
use crate::xml::zusi::zug::fahrzeug_varianten::FahrzeugVarianten;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Prioritaet {
    #[serde(rename = "0")]
    Alle = 0,

    #[serde(rename = "0")]
    Viele = 1,

    #[serde(rename = "2")]
    Wichtige = 2,

    #[serde(rename = "3")]
    NurWichtigste = 3,
}

impl Default for Prioritaet {
    fn default() -> Self {
        Prioritaet::Alle
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ZugTyp {
    #[serde(rename = "0")]
    Gueterzug = 0,

    #[serde(rename = "0")]
    Reisezug = 1,
}

impl Default for ZugTyp {
    fn default() -> Self {
        ZugTyp::Gueterzug
    }
}

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Zug {
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

    #[serde(rename = "@BRAngabe")]
    #[serde(default)]
    #[builder(default)]
    pub baureihe_angabe: String,

    #[serde(rename = "@Prio")]
    #[serde(default)]
    #[builder(default)]
    pub prioritaet: Prioritaet,

    #[serde(rename = "@Standortmodus")]
    #[serde(default)]
    #[builder(default)]
    pub standort_modus: u8, // TODO: replace with enum

    #[serde(rename = "@StartVorschubweg")]
    #[serde(default)]
    #[builder(default)]
    pub start_vorschubweg: f32,

    #[serde(rename = "@BremsstellungZug")]
    #[serde(default)]
    #[builder(default)]
    pub bremsstellung_zug: Bremsstellung,

    #[serde(rename = "@FplBremsstellungTextvorgabe")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_bremsstellung_textvorgabe: String,

    #[serde(rename = "@EnergieVorgabe")]
    #[serde(default)]
    #[builder(default)]
    pub energie_vorgabe: f32,

    #[serde(rename = "@spAnfang")]
    #[serde(default)]
    #[builder(default)]
    pub speed_anfang: f32,

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

    #[serde(rename = "@Verkehrstage")]
    #[serde(default)]
    #[builder(default)]
    pub verkehrstage: String,

    #[serde(rename = "@Grenzlast")]
    #[serde(default)]
    #[builder(default)]
    pub grenzlast: bool,

    #[serde(rename = "@spZugNiedriger")]
    #[serde(default)]
    #[builder(default)]
    pub speed_zug_niedriger: f32,

    #[serde(rename = "@APBeschl")]
    #[serde(default)]
    #[builder(default)]
    pub autopilot_beschleunigung: f32,

    #[serde(rename = "@KeineVorplanKorrektur")]
    #[serde(default)]
    #[builder(default)]
    pub keine_vorplan_korrektur: bool,

    #[serde(rename = "@Dekozug")]
    #[serde(default)]
    #[builder(default)]
    pub dekozug: bool,

    #[serde(rename = "@LODzug")]
    #[serde(default)]
    #[builder(default)]
    pub lod_zug: u8, // TODO: replace with enum

    #[serde(rename = "@ReisendenDichte")]
    #[serde(default)]
    #[builder(default)]
    pub reisenden_dichte: f32,

    #[serde(rename = "@FahrplanGruppe")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_gruppe: String,

    #[serde(rename = "@Rekursionstiefe")]
    #[serde(default)]
    #[builder(default)]
    pub rekursionstiefe: i32,

    #[serde(rename = "@ZugsicherungStartmodus")]
    #[serde(default)]
    #[builder(default)]
    pub zugsicherung_startmodus: u8, // TODO: replace with enum

    #[serde(rename = "@ColdMovement")]
    #[serde(default)]
    #[builder(default)]
    pub cold_movement: bool,

    #[serde(rename = "@FahrstrName")]
    #[serde(default)]
    #[builder(default)]
    pub fahrstrassen_name: String,

    #[serde(rename = "@AufgleisenRegisterpruefen")]
    #[serde(default)]
    #[builder(default)]
    pub aufgleisen_register_pruefen: bool,

    #[serde(rename = "@Zugtyp")]
    #[serde(default)]
    #[builder(default)]
    pub zug_typ: ZugTyp,

    #[serde(rename = "@Ueberschrift")]
    #[serde(default)]
    #[builder(default)]
    pub ueberschrift: String,

    #[serde(rename = "@odtDateiAbs")]
    #[serde(default)]
    #[builder(default)]
    pub odt_datei_absolut: String,

    #[serde(rename = "@BuchfahrplanEinfach")]
    #[serde(default)]
    #[builder(default)]
    pub buchfahrplan_einfach: bool,

    #[serde(rename = "@Buchfahrplandll")]
    #[serde(default)]
    #[builder(default)]
    pub buchfahrplan_dll: String,

    #[serde(rename = "@TuerSystemBezeichner")]
    #[serde(default)]
    #[builder(default)]
    pub tuer_system_bezeichner: String,

    #[serde(rename = "$value")]
    pub value: Vec<ZugValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ZugValue {
    #[serde(rename = "Datei")]
    Datei(Datei),

    #[serde(rename = "BuchfahrplanRohDatei")]
    BuchfahrplanRohDatei(Datei),

    #[serde(rename = "BuchfahrplanBMPDatei")]
    BuchfahrplanBMPDatei(Datei),

    #[serde(rename = "Anfangsbefehl")]
    Anfangsbefehl(Datei),

    #[serde(rename = "Aufgleisreferenz")]
    AufgleisReferenz(AufgleisReferenz),

    #[serde(rename = "FahrplanEintrag")]
    FahrplanEintrag(FahrplanEintrag),

    #[serde(rename = "FahrzeugVarianten")]
    FahrzeugVarianten(FahrzeugVarianten),
}
