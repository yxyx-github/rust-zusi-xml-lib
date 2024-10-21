use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrplanEintragsTyp {
    #[serde(rename = "0")]
    Standard = 0,

    #[serde(rename = "0")]
    Hilfseintrag = 1,

    #[serde(rename = "2")]
    Bedarfshalt = 2,

    #[serde(rename = "3")]
    Betriebshalt = 3,
}

impl Default for FahrplanEintragsTyp {
    fn default() -> Self {
        FahrplanEintragsTyp::Standard
    }
}
