pub mod zugdaten_etcs;
pub mod zugdaten_zbs;
pub mod zugdaten_fahrsperre;
pub mod zugdaten_lzb80;
pub mod zugdaten_indusi_analog;
pub mod zugdaten_indusi_rechner;
pub mod zugdaten_pzb80;

use serde_helpers::with::bool_as_int::bool_as_int_format;
use serde_helpers::default::IsDefault;
use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_etcs::ZugdatenETCS;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_fahrsperre::ZugdatenFahrsperre;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_indusi_analog::ZugdatenIndusiAnalog;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_indusi_rechner::ZugdatenIndusiRechner;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_lzb80::ZugdatenLZB80;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_pzb80::ZugdatenPZB80;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_zbs::ZugdatenZBS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DoppeltraktionsModus {
    #[serde(rename = "0")]
    LokfuehrerOderKeinAntrieb,

    #[serde(rename = "1")]
    Mehrfachtraktion,

    #[serde(rename = "2")]
    AbgeschalteterAntrieb,
}

impl Default for DoppeltraktionsModus {
    fn default() -> Self {
        DoppeltraktionsModus::LokfuehrerOderKeinAntrieb
    }
}

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrzeugInfo {
    #[serde(rename = "@IDHaupt", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub id_haupt: i32,

    #[serde(rename = "@IDNeben", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub id_neben: i32,

    #[serde(rename = "@StartAntriebIndex", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub start_antrieb_index: i32,

    #[serde(rename = "@EigeneBremsstellung", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub eigene_bremsstellung: bool,

    #[serde(rename = "@BremsstellungFahrzeug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremsstellung_fahrzeug: Bremsstellung,

    #[serde(rename = "@EigeneZugart", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub eigene_zugart: bool,

    #[serde(rename = "@Tuerignorieren", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub tuer_ignorieren: bool,

    #[serde(rename = "@DotraModus", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub doppeltraktions_modus: DoppeltraktionsModus,

    #[serde(rename = "@SASchaltung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub stromabnehmer_schaltung: i32,

    #[serde(rename = "@Gedreht", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub gedreht: bool,

    #[serde(rename = "@Feststellbremse", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub feststellbremse: bool,

    #[serde(rename = "@LastwechselBeladen", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub lastwechsel_beladen: i32,

    #[serde(rename = "@Ueberladung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ueberladung: i32,

    #[serde(rename = "@Bremsstoerung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremsstoerung: i32,

    #[serde(rename = "@MgBremseSenken", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub mg_bremse_senken: i32,

    #[serde(rename = "@MgBremseStrom", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub mg_bremse_strom: i32,

    #[serde(rename = "@FahrzeugZusatzinfo", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrzeug_zusatzinfo: String,

    #[serde(rename = "@NVRNummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nvr_nummer: String,

    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(rename = "ZugdatenETCS", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugdaten_etcs: Option<ZugdatenETCS>,

    #[serde(rename = "ZugdatenZBS", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugdaten_zbs: Option<ZugdatenZBS>,

    #[serde(rename = "ZugdatenFahrsperre", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugdaten_fahrsperre: Option<ZugdatenFahrsperre>,

    #[serde(rename = "ZugdatenLZB80", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugdaten_lzb80: Option<ZugdatenLZB80>,

    #[serde(rename = "ZugdatenIndusiAnalog", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugdaten_indusi_analog: Option<ZugdatenIndusiAnalog>,

    #[serde(rename = "ZugdatenIndusiRechner", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugdaten_indusi_rechner: Option<ZugdatenIndusiRechner>,

    #[serde(rename = "ZugdatenPZB80", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugdaten_pzb80: Option<ZugdatenPZB80>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}