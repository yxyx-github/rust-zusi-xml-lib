use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct AutorEintrag {
    #[serde(rename = "@AutorID")]
    #[serde(default)]
    #[builder(default)]
    pub autor_id: i32,

    #[serde(rename = "@AutorName")]
    #[serde(default)]
    #[builder(default)]
    pub autor_name: String,

    #[serde(rename = "@AutorEmail")]
    #[serde(default)]
    #[builder(default)]
    pub autor_email: String,

    #[serde(rename = "@AutorAufwand")]
    #[serde(default)]
    #[builder(default)]
    pub autor_aufwand: f32,

    #[serde(rename = "@AutorAufwandPunkte")]
    #[serde(default)]
    #[builder(default)]
    pub autor_aufwand_punkte: f32,

    #[serde(rename = "@AutorAufwandStunden")]
    #[serde(default)]
    #[builder(default)]
    pub autor_aufwand_stunden: f32,

    #[serde(rename = "@AutorLizenz")]
    #[serde(default)]
    #[builder(default)]
    pub autor_lizenz: u8, // TODO: replace with enum

    #[serde(rename = "@AutorBeschreibung")]
    #[serde(default)]
    #[builder(default)]
    pub autor_beschreibung: String,
}
