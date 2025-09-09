use serde::{Deserialize, Serialize};

// order is not verified
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ETCSModus {
    #[serde(rename = "0")]
    Undefined,

    #[serde(rename = "1")]
    FS,

    #[serde(rename = "2")]
    OS,

    #[serde(rename = "3")]
    SR,

    #[serde(rename = "4")]
    SH,

    #[serde(rename = "5")]
    UN,

    #[serde(rename = "6")]
    SL,

    #[serde(rename = "7")]
    SB,

    #[serde(rename = "8")]
    TR,

    #[serde(rename = "9")]
    PT,

    #[serde(rename = "10")]
    SF,

    #[serde(rename = "11")]
    IS,

    #[serde(rename = "12")]
    NP,

    #[serde(rename = "13")]
    NL,

    #[serde(rename = "14")]
    SE,

    #[serde(rename = "15")]
    SN,

    #[serde(rename = "16")]
    RV,

    #[serde(rename = "17")]
    LS,

    #[serde(rename = "18")]
    PS,
}

impl Default for ETCSModus {
    fn default() -> Self {
        ETCSModus::Undefined
    }
}