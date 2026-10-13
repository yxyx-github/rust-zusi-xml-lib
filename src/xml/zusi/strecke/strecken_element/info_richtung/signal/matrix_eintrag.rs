use crate::xml::zusi::lib::ereignis::Ereignis;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct MatrixEintrag {
    #[serde(rename = "@Signalbild", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_bild: i64,

    #[serde(rename = "@MatrixGesch", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub matrix_geschwindigkeit: f32,

    #[serde(rename = "@SignalID", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_id: i32,

    #[serde(rename = "Ereignis", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ereignisse: Vec<Ereignis>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}