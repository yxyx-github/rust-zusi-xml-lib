use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZugdatenLZB80 {
    #[serde(rename = "@VMZ")]
    #[serde(default)]
    #[builder(default)]
    pub v_max_zug: i32,

    #[serde(rename = "@ZL")]
    #[serde(default)]
    #[builder(default)]
    pub zug_laenge: i32,

    #[serde(rename = "@LZBStoerschalter")]
    #[serde(default)]
    #[builder(default)]
    pub lzb_stoerschalter: i32,

    #[serde(rename = "@LZBGefuehrt")]
    #[serde(default)]
    #[builder(default)]
    pub lzb_gefuehrt: i32,

    #[serde(rename = "@BRH")]
    #[serde(default)]
    #[builder(default)]
    pub bremshundertstel: i32,

    #[serde(rename = "@TfNummer")]
    #[serde(default)]
    #[builder(default)]
    pub tf_nummer: String,

    #[serde(rename = "@Zugnummer")]
    #[serde(default)]
    #[builder(default)]
    pub zug_nummer: String,

    #[serde(rename = "@BRA")]
    #[serde(default)]
    #[builder(default)]
    pub bremsart: i32,

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
