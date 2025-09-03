use serde_helpers::with::bool_as_int::bool_as_int_format;
use serde_helpers::default::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanSignalTyp {
    #[serde(rename = "@FplSignaltypNr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_signal_typ_nummer: u8, // TODO: replace with enum

    #[serde(rename = "@FplHilfssignal", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_hilfs_signal: bool,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
