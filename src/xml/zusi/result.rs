use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

use crate::serde_helpers::{date_time_format, date_time_option_format};
use crate::xml::zusi::result::fahrt_eintrag::FahrtEintrag;

pub mod fahrt_eintrag;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields, from = "ZusiResultDeserialize")]
pub struct ZusiResult {
    #[serde(rename = "@Zugnummer")]
    #[builder(default)]
    pub zugnummer: String,

    #[serde(rename = "@TfNummer")]
    #[builder(default)]
    pub tf_nummer: String,

    #[serde(rename = "@AnfDatum")]
    #[serde(with = "date_time_format")]
    pub anfang_datum: PrimitiveDateTime,

    #[serde(rename = "@Datum")]
    #[serde(with = "date_time_format")]
    pub datum: PrimitiveDateTime,

    #[serde(rename = "@Verbrauch")]
    #[builder(default)]
    pub verbrauch: f32, // in Joule

    #[serde(rename = "@Bemerkung")]
    #[builder(default)]
    pub bemerkung: String,

    #[serde(rename = "@Schummel")]
    #[builder(default)]
    pub schummel: bool,

    #[serde(rename = "@Schwierigkeitsgrad")]
    #[builder(default)]
    pub schwierigkeitsgrad: u32,

    #[serde(rename = "@EnergieVorgabe")]
    #[builder(default)]
    pub energie_vorgabe: f32,

    #[serde(rename = "$value")]
    pub value: Vec<ResultValue>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ZusiResultDeserialize {
    #[serde(rename = "@Zugnummer")]
    #[serde(default)]
    pub zugnummer: String,

    #[serde(rename = "@TfNummer")]
    #[serde(default)]
    pub tf_nummer: String,

    #[serde(rename = "@AnfDatum")]
    #[serde(with = "date_time_option_format")]
    #[serde(default)]
    pub anfang_datum: Option<PrimitiveDateTime>,

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

impl From<ZusiResultDeserialize> for ZusiResult {
    fn from(value: ZusiResultDeserialize) -> Self {
        ZusiResult {
            zugnummer: value.zugnummer,
            tf_nummer: value.tf_nummer,
            anfang_datum: value.anfang_datum.unwrap_or(value.datum),
            datum: value.datum,
            verbrauch: value.verbrauch,
            bemerkung: value.bemerkung,
            schummel: value.schummel,
            schwierigkeitsgrad: value.schwierigkeitsgrad,
            energie_vorgabe: value.energie_vorgabe,
            value: value.value,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ResultValue {
    FahrtEintrag(FahrtEintrag),
    // TODO: add FahrtEventEintrag?
}

impl AsRef<ZusiResult> for ZusiResult {
    fn as_ref(&self) -> &ZusiResult {
        &self
    }
}
