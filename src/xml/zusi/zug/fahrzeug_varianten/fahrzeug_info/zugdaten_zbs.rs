use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZugdatenZBS {
    #[serde(rename = "@BRH")]
    #[serde(default)]
    #[builder(default)]
    pub bremshundertstel: i32,

    #[serde(rename = "@ZL")]
    #[serde(default)]
    #[builder(default)]
    pub zug_laenge: i32,

    #[serde(rename = "@VMZ")]
    #[serde(default)]
    #[builder(default)]
    pub v_max_zug: i32,

    #[serde(rename = "@TfNummer")]
    #[serde(default)]
    #[builder(default)]
    pub tf_nummer: String,

    #[serde(rename = "@Zugnummer")]
    #[serde(default)]
    #[builder(default)]
    pub zug_nummer: String,

    #[serde(rename = "@ZBSBetriebszustand")]
    #[serde(default)]
    #[builder(default)]
    pub zbs_betriebszustand: u8, // TODO: replace with enum

    #[serde(rename = "@Startsystem")]
    #[serde(default)]
    #[builder(default)]
    pub start_system: String,

    #[serde(rename = "@ZugsicherungHS")]
    #[serde(default)]
    #[builder(default)]
    pub zugsicherung_hauptschalter: i32,

    #[serde(rename = "@Lufthahn")]
    #[serde(default)]
    #[builder(default)]
    pub lufthahn: i32,

    #[serde(rename = "@ZBSStoerschalter")]
    #[serde(default)]
    #[builder(default)]
    pub zbs_stoerschalter: i32,
}
