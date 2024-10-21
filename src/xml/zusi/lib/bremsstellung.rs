use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Bremsstellung {
    #[serde(rename = "0")]
    Unknown = 0,

    #[serde(rename = "1")]
    G = 1,

    #[serde(rename = "2")]
    P = 2,

    #[serde(rename = "3")]
    PMg = 3,

    #[serde(rename = "4")]
    R = 4,

    #[serde(rename = "5")]
    RMg = 5,

    #[serde(rename = "6")]
    Aus = 6,

    #[serde(rename = "7")]
    H = 7,

    #[serde(rename = "8")]
    E = 8,

    #[serde(rename = "9")]
    E160 = 9,

    #[serde(rename = "10")]
    RRot =10,
}

impl Default for Bremsstellung {
    fn default() -> Self {
        Bremsstellung::Unknown
    }
}
