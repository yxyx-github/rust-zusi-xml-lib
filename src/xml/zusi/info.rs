pub mod autor_eintrag;

use crate::serde_helpers::IsDefault;
use crate::xml::zusi::info::autor_eintrag::AutorEintrag;
use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use quick_xml::{de, DeError};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Info {
    #[serde(rename = "@DateiTyp")]
    pub datei_typ: DateiTyp,

    #[serde(rename = "@Version")]
    pub version: String,

    #[serde(rename = "@MinVersion")]
    pub min_version: String,

    #[serde(rename = "AutorEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub autor_eintrag: Option<AutorEintrag>,

    #[serde(rename = "Datei", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub datei: Option<Datei>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum DateiTyp {
    #[serde(rename = "result")]
    Result,

    #[serde(rename = "Fahrplan")]
    Fahrplan,

    #[serde(rename = "Zug")]
    Zug,

    #[serde(rename = "Buchfahrplan")]
    Buchfahrplan,
}

impl AsRef<Info> for Info {
    fn as_ref(&self) -> &Info {
        &self
    }
}

impl TryFrom<&str> for DateiTyp {
    type Error = DeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        de::from_str(value)
    }
}