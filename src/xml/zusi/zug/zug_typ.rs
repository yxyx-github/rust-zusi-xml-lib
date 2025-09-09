use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ZugTyp {
    #[serde(rename = "0")]
    Gueterzug,

    #[serde(rename = "1")]
    Reisezug,
}

impl Default for ZugTyp {
    fn default() -> Self {
        ZugTyp::Gueterzug
    }
}