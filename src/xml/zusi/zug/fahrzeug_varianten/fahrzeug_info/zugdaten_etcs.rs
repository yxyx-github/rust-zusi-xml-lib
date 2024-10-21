use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZugdatenETCS {
    #[serde(rename = "@BRH")]
    #[serde(default)]
    #[builder(default)]
    pub bremshundertstel: i32,

    #[serde(rename = "@Zugkategorie")]
    #[serde(default)]
    #[builder(default)]
    pub zugkategorie: i32,

    #[serde(rename = "@ZL")]
    #[serde(default)]
    #[builder(default)]
    pub zug_laenge: i32,

    #[serde(rename = "@VMZ")]
    #[serde(default)]
    #[builder(default)]
    pub v_max_zug: i32,

    #[serde(rename = "@Achslast")]
    #[serde(default)]
    #[builder(default)]
    pub achslast: i32,

    #[serde(rename = "@TfNummer")]
    #[serde(default)]
    #[builder(default)]
    pub tf_nummer: String,

    #[serde(rename = "@Zugnummer")]
    #[serde(default)]
    #[builder(default)]
    pub zug_nummer: String,

    #[serde(rename = "@ETCSLevel")]
    #[serde(default)]
    #[builder(default)]
    pub etcs_level: u8, // TODO: replace with enum

    #[serde(rename = "@ETCSModus")]
    #[serde(default)]
    #[builder(default)]
    pub etcs_modus: u8, // TODO: replace with enum

    #[serde(rename = "@Startsystem")]
    #[serde(default)]
    #[builder(default)]
    pub start_system: String,

    #[serde(rename = "@ETCSLSS")]
    #[serde(default)]
    #[builder(default)]
    pub etcs_lss: i32,

    #[serde(rename = "@ETCSCEASchalter")]
    #[serde(default)]
    #[builder(default)]
    pub etcs_cea_schalter: i32,

    #[serde(rename = "@Lufthahn")]
    #[serde(default)]
    #[builder(default)]
    pub lufthahn: i32,

    #[serde(rename = "@ETCSPassivschalter")]
    #[serde(default)]
    #[builder(default)]
    pub etcs_passiv_schalter: i32,
}
