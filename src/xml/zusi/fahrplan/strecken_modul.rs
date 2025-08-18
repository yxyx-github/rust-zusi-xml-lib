use crate::xml::zusi::lib::phi::Phi;
use crate::xml::zusi::lib::position::Position;
use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct StreckenModul {
    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(rename = "p")]
    pub position: Position,

    #[serde(rename = "phi")]
    pub phi: Phi,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}