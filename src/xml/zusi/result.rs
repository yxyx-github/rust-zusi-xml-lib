use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

use crate::serde_helpers::{date_time_format, date_time_option_format};
use crate::xml::zusi::result::fahrt_eintrag::FahrtEintrag;

pub mod fahrt_eintrag;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZusiResult {
    #[serde(rename = "@Zugnummer")]
    #[serde(default)]
    #[builder(default)]
    pub zugnummer: String,

    #[serde(rename = "@TfNummer")]
    #[serde(default)]
    #[builder(default)]
    pub tf_nummer: String,

    #[serde(rename = "@AnfDatum")]
    #[serde(with = "date_time_option_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub anfang_datum: Option<PrimitiveDateTime>,

    #[serde(rename = "@Datum")]
    #[serde(with = "date_time_format")]
    pub datum: PrimitiveDateTime,

    #[serde(rename = "@Verbrauch")]
    #[serde(default)]
    #[builder(default)]
    pub verbrauch: f32, // in Joule

    #[serde(rename = "@Bemerkung")]
    #[serde(default)]
    #[builder(default)]
    pub bemerkung: String,

    #[serde(rename = "@Schummel")]
    #[serde(default)]
    #[builder(default)]
    pub schummel: bool,

    #[serde(rename = "@Schwierigkeitsgrad")]
    #[serde(default)]
    #[builder(default)]
    pub schwierigkeitsgrad: u32,

    #[serde(rename = "@EnergieVorgabe")]
    #[serde(default)]
    #[builder(default)]
    pub energie_vorgabe: f32,

    #[serde(rename = "$value")]
    pub value: Vec<ResultValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ResultValue {
    FahrtEintrag(FahrtEintrag),
}

impl AsRef<ZusiResult> for ZusiResult {
    fn as_ref(&self) -> &ZusiResult {
        &self
    }
}
