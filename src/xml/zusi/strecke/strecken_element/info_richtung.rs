pub mod signal;

use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::ereignis::Ereignis;
use crate::xml::zusi::strecke::strecken_element::info_richtung::signal::Signal;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct InfoRichtung {
    #[serde(rename = "@vMax", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub v_max: f32,

    #[serde(rename = "@km", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub km: f32,

    #[serde(rename = "@pos", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub positiv: bool,

    #[serde(rename = "@Reg", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub register: i32,

    #[serde(rename = "@KoppelWeicheNr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub koppel_weiche_nummer: i32,

    #[serde(rename = "@KoppelWeicheNorm", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub koppel_weiche_norm: bool,

    #[serde(rename = "Ereignis", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ereignisse: Vec<Ereignis>,

    #[serde(rename = "Signal", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal: Option<Signal>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}