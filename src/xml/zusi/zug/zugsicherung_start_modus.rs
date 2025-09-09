use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ZugsicherungStartModus {
    #[serde(rename = "0")]
    GemaessSimulatorEinstellung,

    #[serde(rename = "1")]
    ZugdatenMuessenEingegebenWerden,

    #[serde(rename = "2")]
    ZugdatenSindBereitsEingegeben,
}

impl Default for ZugsicherungStartModus {
    fn default() -> Self {
        ZugsicherungStartModus::GemaessSimulatorEinstellung
    }
}
