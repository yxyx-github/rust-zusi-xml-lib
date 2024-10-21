use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanNameRechts {
    #[serde(rename = "@FplNameText")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_name_text: String,
}
