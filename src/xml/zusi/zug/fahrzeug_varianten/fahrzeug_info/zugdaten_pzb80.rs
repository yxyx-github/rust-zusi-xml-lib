use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZugdatenPZB80 {
    #[serde(rename = "@IndusiZugart")]
    #[serde(default)]
    #[builder(default)]
    pub indusi_zugart: u8, // TODO: replace with enum
}
