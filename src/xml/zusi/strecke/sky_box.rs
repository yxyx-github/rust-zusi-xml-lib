use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct SkyBox {
    #[serde(rename = "Skybox0")]
    pub sky_box_0: Datei,

    #[serde(rename = "Skybox1")]
    pub sky_box_1: Datei,

    #[serde(rename = "Skybox2")]
    pub sky_box_2: Datei,

    #[serde(rename = "Skybox3")]
    pub sky_box_3: Datei,

    #[serde(rename = "Skybox4")]
    pub sky_box_4: Datei,

    #[serde(rename = "Skybox5")]
    pub sky_box_5: Datei,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}