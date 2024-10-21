use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanSignalTyp {
    #[serde(rename = "@FplSignaltypNr")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_signal_typ_nummer: u8, // TODO: replace with enum

    #[serde(rename = "@FplHilfssignal")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_hilfs_signal: bool,
}
