pub mod aufgleis_referenz;
pub mod fahrplan_eintrag;
pub mod fahrzeug_varianten;
pub mod zugsicherung_start_modus;
pub mod standort_modus;
pub mod zug_typ;
pub mod lod_zug;

use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::zug::aufgleis_referenz::AufgleisReferenz;
use crate::xml::zusi::zug::fahrplan_eintrag::FahrplanEintrag;
use crate::xml::zusi::zug::fahrzeug_varianten::FahrzeugVarianten;
use crate::xml::zusi::zug::lod_zug::LodZug;
use crate::xml::zusi::zug::standort_modus::StandortModus;
use crate::xml::zusi::zug::zug_typ::ZugTyp;
use crate::xml::zusi::zug::zugsicherung_start_modus::ZugsicherungStartModus;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Zug {
    #[serde(rename = "@Gattung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub gattung: String,

    #[serde(rename = "@Nummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nummer: String,

    #[serde(rename = "@Zuglauf", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zuglauf: String,

    #[serde(rename = "@BRAngabe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub baureihe_angabe: String,

    #[serde(rename = "@Prio", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub prioritaet: i32,

    #[serde(rename = "@Standortmodus", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub standort_modus: StandortModus,

    #[serde(rename = "@StartVorschubweg", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub start_vorschubweg: f32,

    #[serde(rename = "@BremsstellungZug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremsstellung_zug: Bremsstellung,

    #[serde(rename = "@FplBremsstellungTextvorgabe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_bremsstellung_textvorgabe: String,

    #[serde(rename = "@EnergieVorgabe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub energie_vorgabe: f32,

    #[serde(rename = "@spAnfang", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub speed_anfang: f32,

    #[serde(rename = "@MBrh", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub mindest_bremshundertstel: f32,

    #[serde(rename = "@FplMasse", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_masse: f32,

    #[serde(rename = "@FplZuglaenge", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_zug_laenge: f32,

    #[serde(rename = "@Verkehrstage", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub verkehrstage: String,

    #[serde(rename = "@Grenzlast", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub grenzlast: bool,

    #[serde(rename = "@spZugNiedriger", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub speed_zug_niedriger: f32,

    #[serde(rename = "@APBeschl", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autopilot_beschleunigung: f32,

    #[serde(rename = "@KeineVorplanKorrektur", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub keine_vorplan_korrektur: bool,

    #[serde(rename = "@Dekozug", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub dekozug: bool,

    #[serde(rename = "@LODzug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub lod_zug: LodZug,

    #[serde(rename = "@ReisendenDichte", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub reisenden_dichte: f32,

    #[serde(rename = "@FahrplanGruppe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_gruppe: String,

    #[serde(rename = "@Rekursionstiefe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub rekursionstiefe: i32,

    #[serde(rename = "@ZugsicherungStartmodus", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugsicherung_startmodus: ZugsicherungStartModus,

    #[serde(rename = "@ColdMovement", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub cold_movement: bool,

    #[serde(rename = "@FahrstrName", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrassen_name: String,

    #[serde(rename = "@AufgleisenRegisterpruefen", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub aufgleisen_register_pruefen: bool,

    #[serde(rename = "@Zugtyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_typ: ZugTyp,

    #[serde(rename = "@Ueberschrift", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ueberschrift: String,

    #[serde(rename = "@odtDateiAbs", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub odt_datei_absolut: String,

    #[serde(rename = "@BuchfahrplanEinfach", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub buchfahrplan_einfach: bool,

    #[serde(rename = "@Buchfahrplandll", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub buchfahrplan_dll: String,

    #[serde(rename = "@TuerSystemBezeichner", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub tuer_system_bezeichner: String,

    #[serde(rename = "Datei")]
    pub fahrplan_datei: Datei,

    #[serde(rename = "BuchfahrplanRohDatei", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub buchfahrplan_roh_datei: Option<Datei>,

    #[serde(rename = "BuchfahrplanBMPDatei", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub buchfahrplan_bmp_datei: Option<Datei>,

    #[serde(rename = "Anfangsbefehl", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub anfangsbefehl: Option<Datei>,

    #[serde(rename = "Aufgleisreferenz", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub aufgleis_referenz: Option<AufgleisReferenz>,

    #[serde(rename = "FahrplanEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_eintraege: Vec<FahrplanEintrag>,

    #[serde(rename = "FahrzeugVarianten")]
    pub fahrzeug_varianten: FahrzeugVarianten,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}