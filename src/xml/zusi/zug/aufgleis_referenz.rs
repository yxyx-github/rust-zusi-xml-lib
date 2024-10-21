use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct AufgleisReferenz {
    #[serde(rename = "@ReferenzNr")]
    #[serde(default)]
    #[builder(default)]
    pub referenz_nummer: i32,

    #[serde(rename = "$value")]
    pub value: Vec<AufgleisReferenzValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AufgleisReferenzValue {
    #[serde(rename = "Datei")]
    Datei(Datei),
}
