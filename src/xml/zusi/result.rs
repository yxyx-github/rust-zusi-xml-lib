use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

use crate::serde_helpers::{date_time_format, date_time_option_format};
use crate::xml::zusi::result::fahrt_eintrag::FahrtEintrag;

pub mod fahrt_eintrag;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct ZusiResult {
    #[serde(rename = "@Zugnummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugnummer: String,

    #[serde(rename = "@TfNummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub tf_nummer: String,

    #[serde(rename = "@AnfDatum", with = "date_time_option_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub anfang_datum: Option<PrimitiveDateTime>,

    #[serde(rename = "@Datum", with = "date_time_format")]
    pub datum: PrimitiveDateTime,

    #[serde(rename = "@Verbrauch", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub verbrauch: f32, // in Joule

    #[serde(rename = "@Bemerkung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bemerkung: String,

    #[serde(rename = "@Schummel", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub schummel: bool,

    #[serde(rename = "@Schwierigkeitsgrad", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub schwierigkeitsgrad: u32,

    #[serde(rename = "@EnergieVorgabe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub energie_vorgabe: f32,

    #[serde(rename = "FahrtEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    pub fahrt_eintraege: Vec<FahrtEintrag>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}

impl AsRef<ZusiResult> for ZusiResult {
    fn as_ref(&self) -> &ZusiResult {
        &self
    }
}
