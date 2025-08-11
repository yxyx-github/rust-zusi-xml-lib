use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrplanEintragsTyp {
    #[serde(rename = "0")]
    Standard,

    #[serde(rename = "1")]
    Hilfseintrag,

    #[serde(rename = "2")]
    Bedarfshalt,

    #[serde(rename = "3")]
    Betriebshalt,
}

impl Default for FahrplanEintragsTyp {
    fn default() -> Self {
        FahrplanEintragsTyp::Standard
    }
}
