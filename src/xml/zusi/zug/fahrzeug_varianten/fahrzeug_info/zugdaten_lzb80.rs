use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct ZugdatenLZB80 {
    #[serde(rename = "@VMZ", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub v_max_zug: i32,

    #[serde(rename = "@ZL", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_laenge: i32,

    #[serde(rename = "@LZBStoerschalter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub lzb_stoerschalter: i32,

    #[serde(rename = "@LZBGefuehrt", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub lzb_gefuehrt: i32,

    #[serde(rename = "@BRH", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremshundertstel: i32,

    #[serde(rename = "@TfNummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub tf_nummer: String,

    #[serde(rename = "@Zugnummer", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_nummer: String,

    #[serde(rename = "@BRA", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bremsart: i32,

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
