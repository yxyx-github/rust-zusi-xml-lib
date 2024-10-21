use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanKm {
    #[serde(rename = "@km")]
    #[serde(default)]
    #[builder(default)]
    pub km: f32,

    #[serde(rename = "@FplSprung")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_sprung: bool,

    #[serde(rename = "@FplkmNeu")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_km_neu: f32,
}
