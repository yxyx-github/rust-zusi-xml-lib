use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum LodZug {
    #[serde(rename = "0")]
    Alle,

    #[serde(rename = "1")]
    Viele,

    #[serde(rename = "2")]
    Wichtige,

    #[serde(rename = "3")]
    NurWichtigste,
}

impl Default for LodZug {
    fn default() -> Self {
        LodZug::Alle
    }
}