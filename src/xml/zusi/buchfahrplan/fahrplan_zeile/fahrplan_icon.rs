use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanIcon {
    #[serde(rename = "@FplIconNr")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_icon_nummer: i32, // TODO: replace with enum?
}
