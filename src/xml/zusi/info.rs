pub mod autor_eintrag;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::info::autor_eintrag::AutorEintrag;
use crate::xml::zusi::lib::datei::Datei;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct Info {
    #[serde(rename = "@DateiTyp")]
    pub datei_typ: String,

    #[serde(rename = "@Version")]
    pub version: String,

    #[serde(rename = "@MinVersion")]
    pub min_version: String,

    #[serde(rename = "AutorEintrag", default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub autor_eintrag: Option<AutorEintrag>,

    #[serde(rename = "Datei", default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub datei: Option<Datei>,

    #[serde(flatten)]
    #[builder(default)]
    pub unknown: HashMap<String, String>,
}

impl AsRef<Info> for Info {
    fn as_ref(&self) -> &Info {
        &self
    }
}