use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum StandortModus {
    #[serde(rename = "0")]
    Automatisch,

    #[serde(rename = "1")]
    Fahrstrassenanfang,

    #[serde(rename = "2")]
    VorziehenBisSignal,

    #[serde(rename = "3")]
    Vorgabewert,
}

impl Default for StandortModus {
    fn default() -> Self {
        StandortModus::Automatisch
    }
}
