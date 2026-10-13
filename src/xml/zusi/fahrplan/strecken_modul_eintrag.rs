use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::lib::punkt_xyz::PunktXYZ;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct StreckenModulEintrag {
    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(rename = "p")]
    pub position: PunktXYZ,

    #[serde(rename = "phi")]
    pub phi: PunktXYZ,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}