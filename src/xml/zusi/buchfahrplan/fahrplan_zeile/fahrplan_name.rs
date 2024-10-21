use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanName {
    #[serde(rename = "@FplNameText")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_name_text: String,

    #[serde(rename = "@FplWichtigkeit")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_wichtigkeit: i32,

    #[serde(rename = "@FplOriginaltext")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_original_text: String, // TODO: check if it is just not documented?
}
