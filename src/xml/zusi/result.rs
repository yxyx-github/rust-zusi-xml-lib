use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::xml::format::date_time_format;
use crate::xml::zusi::result::fahrt_eintrag::FahrtEintrag;

pub mod fahrt_eintrag;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ZusiResult {
    #[serde(rename = "@Zugnummer")]
    #[serde(default)]
    pub zugnummer: String,

    #[serde(rename = "@TfNummer")]
    #[serde(default)]
    pub tf_nummer: String,

    #[serde(rename = "@Datum")]
    #[serde(with = "date_time_format")]
    pub datum: PrimitiveDateTime,

    #[serde(rename = "@Verbrauch")]
    #[serde(default)]
    pub verbrauch: f32, // in Joule

    #[serde(rename = "@Bemerkung")]
    #[serde(default)]
    pub bemerkung: String,

    #[serde(rename = "@Schummel")]
    #[serde(default)]
    pub schummel: bool,

    #[serde(rename = "@Schwierigkeitsgrad")]
    #[serde(default)]
    pub schwierigkeitsgrad: u32,

    #[serde(rename = "@EnergieVorgabe")]
    #[serde(default)]
    pub energie_vorgabe: f32,

    #[serde(rename = "$value")]
    pub value: Vec<ResultValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ResultValue {
    FahrtEintrag(FahrtEintrag),
    // TODO: add FahrtEventEintrag?
}