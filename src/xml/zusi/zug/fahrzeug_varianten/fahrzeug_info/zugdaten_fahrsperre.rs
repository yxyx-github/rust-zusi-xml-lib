use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct ZugdatenFahrsperre {
    #[serde(rename = "@ZugsicherungHS", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zugsicherung_hauptschalter: i32,

    #[serde(rename = "@Lufthahn", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub lufthahn: i32,

    #[serde(rename = "@PZBStoerschalter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub pzb_stoerschalter: i32,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
