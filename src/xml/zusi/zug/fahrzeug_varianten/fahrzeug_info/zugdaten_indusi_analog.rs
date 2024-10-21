use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZugdatenIndusiAnalog {
    #[serde(rename = "@IndusiZugart")]
    #[serde(default)]
    #[builder(default)]
    pub indusi_zugart: u8, // TODO: replace with enum

    #[serde(rename = "@ZugsicherungHS")]
    #[serde(default)]
    #[builder(default)]
    pub zugsicherung_hauptschalter: i32,

    #[serde(rename = "@Lufthahn")]
    #[serde(default)]
    #[builder(default)]
    pub lufthahn: i32,

    #[serde(rename = "@PZBStoerschalter")]
    #[serde(default)]
    #[builder(default)]
    pub pzb_stoerschalter: i32,
}
