use crate::xml::zusi::lib::datei::Datei;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct SkyDome {
    #[serde(rename = "HimmelTex")]
    pub himmel_textur: Datei,

    #[serde(rename = "SonneTex")]
    pub sonne_textur: Datei,

    #[serde(rename = "SonneHorizontTex")]
    pub sonne_horizont_textur: Datei,

    #[serde(rename = "MondTex")]
    pub mond_textur: Datei,

    #[serde(rename = "SternTex")]
    pub stern_textur: Datei,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}