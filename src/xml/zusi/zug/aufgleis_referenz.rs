use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct AufgleisReferenz {
    #[serde(rename = "@ReferenzNr")]
    #[builder(default)]
    pub referenz_nummer: i32,

    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}