use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Bremsstellung {
    #[serde(rename = "0")]
    Unknown,

    #[serde(rename = "1")]
    G,

    #[serde(rename = "2")]
    P,

    #[serde(rename = "3")]
    PMg,

    #[serde(rename = "4")]
    R,

    #[serde(rename = "5")]
    RMg,

    #[serde(rename = "6")]
    Aus,

    #[serde(rename = "7")]
    H,

    #[serde(rename = "8")]
    E,

    #[serde(rename = "9")]
    E160,

    #[serde(rename = "10")]
    RRot,
}

impl Default for Bremsstellung {
    fn default() -> Self {
        Bremsstellung::Unknown
    }
}
