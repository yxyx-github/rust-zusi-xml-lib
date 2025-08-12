use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct ZugdatenETCS {
    #[serde(rename = "@BRH", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremshundertstel: i32,

    #[serde(rename = "@Zugkategorie", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugkategorie: i32,

    #[serde(rename = "@ZL", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_laenge: i32,

    #[serde(rename = "@VMZ", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub v_max_zug: i32,

    #[serde(rename = "@Achslast", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub achslast: i32,

    #[serde(rename = "@TfNummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub tf_nummer: String,

    #[serde(rename = "@Zugnummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_nummer: String,

    #[serde(rename = "@ETCSLevel", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_level: u8, // TODO: replace with enum

    #[serde(rename = "@ETCSModus", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_modus: u8, // TODO: replace with enum

    #[serde(rename = "@Startsystem", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub start_system: String,

    #[serde(rename = "@ETCSLSS", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_lss: i32,

    #[serde(rename = "@ETCSCEASchalter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_cea_schalter: i32,

    #[serde(rename = "@Lufthahn", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub lufthahn: i32,

    #[serde(rename = "@ETCSPassivschalter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_passiv_schalter: i32,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
