use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum FahrzeugVerbandAktion {
    #[serde(rename = "0")]
    Keine,

    #[serde(rename = "1")]
    ZugDrehen,

    #[serde(rename = "2")]
    Fueherstandswechsel,
}

impl Default for FahrzeugVerbandAktion {
    fn default() -> Self {
        FahrzeugVerbandAktion::Keine
    }
}
