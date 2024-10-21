use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZugEintrag {
    #[serde(rename = "$value")]
    pub value: Vec<ZugEintragValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ZugEintragValue {
    #[serde(rename = "Datei")]
    Datei(Datei),
}
