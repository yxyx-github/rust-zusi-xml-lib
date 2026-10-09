use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanAnmerkung {
    #[serde(rename = "@FplAnmerkungTyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_anmerkung_typ: i32, // TODO: replace with enum

    #[serde(rename = "@FplAnmerkungText", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_anmerkung_text: String,

    #[serde(rename = "@FplAnmerkungSpalte", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_anmerkung_spalte: i32,
}