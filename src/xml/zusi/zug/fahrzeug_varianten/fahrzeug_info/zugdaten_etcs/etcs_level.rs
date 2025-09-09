use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ETCSLevel {
    #[serde(rename = "0")]
    Undefined,

    #[serde(rename = "1")]
    STM,

    #[serde(rename = "2")]
    Level0,

    #[serde(rename = "3")]
    Level1,

    #[serde(rename = "4")]
    Level2,

    #[serde(rename = "5")]
    Level3,
}

impl Default for ETCSLevel {
    fn default() -> Self {
        ETCSLevel::Undefined
    }
}