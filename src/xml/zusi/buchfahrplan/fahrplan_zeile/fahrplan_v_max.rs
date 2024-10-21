use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanVMax {
    #[serde(rename = "@vMaxTyp")]
    #[serde(default)]
    #[builder(default)]
    pub v_max_typ: i32, // TODO: replace with enum?

    #[serde(rename = "@vMax")]
    #[serde(default)]
    #[builder(default)]
    pub v_max: f32,
}
