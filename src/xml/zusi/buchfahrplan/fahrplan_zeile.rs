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

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
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

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanZeile {
    #[serde(rename = "@FplRglGgl")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_regelgleis_gegengleis: i32, // TODO: replace with enum?

    #[serde(rename = "@FplLaufweg")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_laufweg: f32,

    #[serde(rename = "@FahrstrStrecke")]
    #[serde(default)]
    #[builder(default)]
    pub fahrstrasse_strecke: String,

    #[serde(rename = "$value")]
    pub value: Vec<FahrplanZeileValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrplanZeileValue {
    #[serde(rename = "FplvMax")]
    FahrplanVMax(FahrplanVMax),

    #[serde(rename = "FplvMaxGNT")]
    FahrplanVMaxGNT(FahrplanVMaxGNT),

    #[serde(rename = "Fplkm")]
    FahrplanKm(FahrplanKm),

    #[serde(rename = "FplName")]
    FahrplanName(FahrplanName),

    #[serde(rename = "FplSignaltyp")]
    FahrplanSignalTyp(FahrplanSignalTyp),

    #[serde(rename = "FplIcon")]
    FahrplanIcon(FahrplanIcon),

    #[serde(rename = "FplTunnel")]
    FahrplanTunnel(FahrplanTunnel),

    #[serde(rename = "FplNameRechts")]
    FahrplanNameRechts(FahrplanNameRechts),

    #[serde(rename = "FplSaegelinien")]
    FahrplanSaegelinien(FahrplanSaegelinien),

    #[serde(rename = "FplAnk")]
    FahrplanAnkunft(FahrplanAnkunft),

    #[serde(rename = "FplAbf")]
    FahrplanAbfahrt(FahrplanAbfahrt),

    #[serde(rename = "FplRichtungswechsel")]
    FahrplanRichtungswechsel(FahrplanRichtungswechsel),

    #[serde(rename = "FplvMaxReduzierungen")]
    FahrplanVMaxReduzierungen(FahrplanVMaxReduzierungen),
}
