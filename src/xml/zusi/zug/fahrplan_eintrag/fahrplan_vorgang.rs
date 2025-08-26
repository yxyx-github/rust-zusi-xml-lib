use crate::serde_helpers::IsDefault;
use crate::xml::zusi::lib::abhaengigkeit::Abhaengigkeit;
use crate::xml::zusi::lib::ereignis::Ereignis;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanVorgang {
    #[serde(rename = "@Beschreibung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub beschreibung: String,
    
    #[serde(rename = "@StartbedingungVorgang", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub startbedingung_vorgang: String,

    #[serde(rename = "Abhaengigkeit", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abhaengigkeiten: Vec<Abhaengigkeit>,

    #[serde(rename = "Ereignis", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ereignisse: Vec<Ereignis>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}