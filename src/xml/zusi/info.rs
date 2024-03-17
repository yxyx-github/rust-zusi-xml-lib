use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Info {
    #[serde(rename = "@DateiTyp")]
    pub datei_typ: String,

    #[serde(rename = "@Version")]
    pub version: String,

    #[serde(rename = "@MinVersion")]
    pub min_version: String,
}