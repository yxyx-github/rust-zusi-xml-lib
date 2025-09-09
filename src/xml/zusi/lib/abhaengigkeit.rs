pub mod abhaengigkeit_operator;
pub mod abhaengigkeit_bedingung;

use crate::xml::zusi::lib::abhaengigkeit::abhaengigkeit_bedingung::AbhaengigkeitBedingung;
use crate::xml::zusi::lib::abhaengigkeit::abhaengigkeit_operator::AbhaengigkeitOperator;
use crate::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_etcs::etcs_modus::ETCSModus;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Abhaengigkeit {
    #[serde(rename = "@AbhOperator", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeit_operator: AbhaengigkeitOperator,

    #[serde(rename = "@AbhBedingung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeit_bedingung: AbhaengigkeitBedingung,

    #[serde(rename = "@AbhParameter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeit_parameter: f32,

    #[serde(rename = "@AbhAndererZug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeit_anderer_zug: String,

    #[serde(rename = "@ETCSModus", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_modus: ETCSModus,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
