use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanSaegelinien {
    #[serde(rename = "@FplAnzahl")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_anzahl: i32,
}
