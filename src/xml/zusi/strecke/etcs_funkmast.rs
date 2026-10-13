use crate::xml::zusi::lib::punkt_xyz::PunktXYZ;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct ETCSFunkmast {
    #[serde(rename = "@ETCSSenderadius", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_sende_radius: i32,

    #[serde(rename = "@ETCSRBCNummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_rbc_nummer: i64,

    #[serde(rename = "@ETCSRBCTelNummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_rbc_telefon_nummer: String,

    #[serde(rename = "@ETCSRBCID", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_rbc_id: i64,

    #[serde(rename = "@ETCSRBCLand", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_rbc_land: i64,

    #[serde(rename = "@ETCSGSMRNetz", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_gsmr_netz: i64,

    #[serde(rename = "@ETCSUebergangTAF", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_uebergang_taf: bool,

    #[serde(rename = "@ETCSUebergangOS", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_uebergang_os: bool,

    #[serde(rename = "@ETCSTransitionErsatzsignalSR", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_transition_ersatz_signal_sr: bool,

    #[serde(rename = "@ETCSReleaseSpeedDefault", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_release_speed_default: f32,

    #[serde(rename = "@ETCSManuellSH", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_manuell_sh: bool,

    #[serde(rename = "@ETCSKeinTextReleaseSpeed", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_kein_text_release_speed: bool,

    #[serde(rename = "@ETCSFunkausfallToleranz", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_funk_ausfall_toleranz: f32,

    #[serde(rename = "@ETCSBaseline", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_baseline: String,

    #[serde(rename = "p", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub positionen: Vec<PunktXYZ>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}