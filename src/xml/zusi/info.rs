pub mod autor_eintrag;
pub mod datei;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::info::autor_eintrag::AutorEintrag;
use crate::xml::zusi::info::datei::Datei;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct Info {
    #[serde(rename = "@DateiTyp")]
    pub datei_typ: String,

    #[serde(rename = "@Version")]
    pub version: String,

    #[serde(rename = "@MinVersion")]
    pub min_version: String,

    #[serde(rename = "$value")]
    #[serde(default)]
    #[builder(default)]
    pub value: Vec<InfoValue>,

    // TODO: add missing attributes
}

impl AsRef<Info> for Info {
    fn as_ref(&self) -> &Info {
        &self
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum InfoValue {
    AutorEintrag(AutorEintrag),
    Datei(Datei),
}
