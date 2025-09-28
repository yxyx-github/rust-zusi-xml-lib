pub mod fahrplan_v_max;
pub mod fahrplan_v_max_gnt;
pub mod fahrplan_km;
pub mod fahrplan_name;
pub mod fahrplan_signal_typ;
pub mod fahrplan_icon;
pub mod fahrplan_tunnel;
pub mod fahrplan_name_rechts;
pub mod fahrplan_saegelinien;
pub mod fahrplan_ankunft;
pub mod fahrplan_abfahrt;
pub mod fahrplan_richtungswechsel;
pub mod fahrplan_v_max_reduzierungen;

use serde_helpers::default::IsDefault;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_abfahrt::FahrplanAbfahrt;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_ankunft::FahrplanAnkunft;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_icon::FahrplanIcon;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_km::FahrplanKm;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_name::FahrplanName;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_name_rechts::FahrplanNameRechts;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_richtungswechsel::FahrplanRichtungswechsel;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_saegelinien::FahrplanSaegelinien;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_signal_typ::FahrplanSignalTyp;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_tunnel::FahrplanTunnel;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max::FahrplanVMax;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_gnt::FahrplanVMaxGNT;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_reduzierungen::FahrplanVMaxReduzierungen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanZeile {
    #[serde(rename = "@FplRglGgl", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_regelgleis_gegengleis: i32,

    #[serde(rename = "@FplLaufweg", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_laufweg: f32,

    #[serde(rename = "@FahrstrStrecke", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_strecke: String,

    /// not documented
    #[serde(rename = "@FahrstrStreckeLa", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_strecke_la: String,

    /// not documented
    #[serde(rename = "@FahrstrStreckeStrukturnummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_strecke_struktur_nummer: String,

    #[serde(rename = "FplvMax", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_v_max: Option<FahrplanVMax>,

    #[serde(rename = "FplvMaxGNT", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_v_max_gnt: Option<FahrplanVMaxGNT>,

    #[serde(rename = "Fplkm", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_km: Vec<FahrplanKm>,

    #[serde(rename = "FplName", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_name: Option<FahrplanName>,

    #[serde(rename = "FplSignaltyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_signal_typ: Option<FahrplanSignalTyp>,

    #[serde(rename = "FplIcon", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_icon: Vec<FahrplanIcon>,

    #[serde(rename = "FplTunnel", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_tunnel: Option<FahrplanTunnel>,

    #[serde(rename = "FplNameRechts", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_name_rechts: Option<FahrplanNameRechts>,

    #[serde(rename = "FplSaegelinien", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_saegelinien: Option<FahrplanSaegelinien>,

    #[serde(rename = "FplAnk", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_ankunft: Option<FahrplanAnkunft>,

    #[serde(rename = "FplAbf", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_abfahrt: Option<FahrplanAbfahrt>,

    #[serde(rename = "FplRichtungswechsel", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_richtungswechsel: Option<FahrplanRichtungswechsel>,

    #[serde(rename = "FplvMaxReduzierungen", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_v_max_reduzierungen: Option<FahrplanVMaxReduzierungen>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}