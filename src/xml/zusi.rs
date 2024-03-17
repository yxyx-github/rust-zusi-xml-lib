pub mod info;
pub mod result;

use serde::{Deserialize, Serialize};
use crate::xml::zusi::info::Info;
use crate::xml::zusi::result::ZusiResult;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Zusi {
    #[serde(rename = "$value")]
    pub value: Vec<ZusiValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ZusiValue {
    Info(Info),

    #[serde(rename = "result")]
    Result(ZusiResult),
}