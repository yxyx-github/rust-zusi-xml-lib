use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum AbhaengigkeitOperator {
    #[serde(rename = "0")]
    UND,

    #[serde(rename = "1")]
    ODER,
}

impl Default for AbhaengigkeitOperator {
    fn default() -> Self {
        AbhaengigkeitOperator::UND
    }
}