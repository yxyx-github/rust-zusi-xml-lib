mod zugdaten_etcs;
mod zugdaten_zbs;
mod zugdaten_fahrsperre;
mod zugdaten_lzb80;
mod zugdaten_indusi_analog;
mod zugdaten_indusi_rechner;
mod zugdaten_pzb80;

use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::bremsstellung::Bremsstellung;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_etcs::ZugdatenETCS;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_fahrsperre::ZugdatenFahrsperre;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_indusi_analog::ZugdatenIndusiAnalog;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_indusi_rechner::ZugdatenIndusiRechner;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_lzb80::ZugdatenLZB80;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_pzb80::ZugdatenPZB80;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_zbs::ZugdatenZBS;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DoppeltraktionsModus {
    #[serde(rename = "0")]
    LokfuehrerOderKeinAntrieb = 0,

    #[serde(rename = "1")]
    Mehrfachtraktion = 1,

    #[serde(rename = "2")]
    AbgeschalteterAntrieb = 2,
}

impl Default for DoppeltraktionsModus {
    fn default() -> Self {
        DoppeltraktionsModus::LokfuehrerOderKeinAntrieb
    }
}

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrzeugVarianten {
    #[serde(rename = "@IDHaupt")]
    #[serde(default)]
    #[builder(default)]
    pub id_haupt: i32,

    #[serde(rename = "@IDNeben")]
    #[serde(default)]
    #[builder(default)]
    pub id_neben: i32,

    #[serde(rename = "@StartAntriebIndex")]
    #[serde(default)]
    #[builder(default)]
    pub start_antrieb_index: i32,

    #[serde(rename = "@EigeneBremsstellung")]
    #[serde(default)]
    #[builder(default)]
    pub eigene_bremsstellung: bool,

    #[serde(rename = "@BremsstellungFahrzeug")]
    #[serde(default)]
    #[builder(default)]
    pub bremsstellung_fahrzeug: Bremsstellung,

    #[serde(rename = "@EigeneZugart")]
    #[serde(default)]
    #[builder(default)]
    pub eigene_zugart: bool,

    #[serde(rename = "@Tuerignorieren")]
    #[serde(default)]
    #[builder(default)]
    pub tuer_ignorieren: bool,

    #[serde(rename = "@DotraModus")]
    #[serde(default)]
    #[builder(default)]
    pub doppeltraktions_modus: DoppeltraktionsModus,

    #[serde(rename = "@SASchaltung")]
    #[serde(default)]
    #[builder(default)]
    pub stromabnehmer_schaltung: i32,

    #[serde(rename = "@Gedreht")]
    #[serde(default)]
    #[builder(default)]
    pub gedreht: bool,

    #[serde(rename = "@Feststellbremse")]
    #[serde(default)]
    #[builder(default)]
    pub feststellbremse: bool,

    #[serde(rename = "@LastwechselBeladen")]
    #[serde(default)]
    #[builder(default)]
    pub lastwechsel_beladen: i32,

    #[serde(rename = "@FahrzeugZusatzinfo")]
    #[serde(default)]
    #[builder(default)]
    pub fahrzeug_zusatzinfo: String,

    #[serde(rename = "@NVRNummer")]
    #[serde(default)]
    #[builder(default)]
    pub nvr_nummer: String,

    #[serde(rename = "$value")]
    pub value: Vec<FahrzeugInfoValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrzeugInfoValue {
    #[serde(rename = "Datei")]
    Datei(Datei),

    #[serde(rename = "ZugdatenETCS")]
    ZugdatenETCS(ZugdatenETCS),

    #[serde(rename = "ZugdatenZBS")]
    ZugdatenZBS(ZugdatenZBS),

    #[serde(rename = "ZugdatenFahrsperre")]
    ZugdatenFahrsperre(ZugdatenFahrsperre),

    #[serde(rename = "ZugdatenLZB80")]
    ZugdatenLZB80(ZugdatenLZB80),

    #[serde(rename = "ZugdatenIndusiAnalog")]
    ZugdatenIndusiAnalog(ZugdatenIndusiAnalog),

    #[serde(rename = "ZugdatenIndusiRechner")]
    ZugdatenIndusiRechner(ZugdatenIndusiRechner),

    #[serde(rename = "ZugdatenPZB80")]
    ZugdatenPZB80(ZugdatenPZB80),
}
