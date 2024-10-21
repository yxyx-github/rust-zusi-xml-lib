mod position;
mod phi;

use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::fahrplan::strecken_modul::phi::Phi;
use crate::xml::zusi::fahrplan::strecken_modul::position::Position;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct StreckenModul {
    #[serde(rename = "$value")]
    pub value: Vec<StreckenModulValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum StreckenModulValue {
    #[serde(rename = "Datei")]
    Datei(Datei),

    #[serde(rename = "Position")]
    Position(Position),

    #[serde(rename = "Phi")]
    Phi(Phi),
}
