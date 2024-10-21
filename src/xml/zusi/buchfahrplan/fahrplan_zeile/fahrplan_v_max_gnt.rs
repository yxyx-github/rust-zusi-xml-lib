use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanVMaxGNT {
    #[serde(rename = "@vMax")]
    #[serde(default)]
    #[builder(default)]
    pub v_max: f32,
}
