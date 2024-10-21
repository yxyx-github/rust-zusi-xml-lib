use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Fahrzeug {
    #[serde(rename = "@FahrzeugZusatzinfo")]
    #[serde(default)]
    #[builder(default)]
    pub fahrzeug_zusatzinfo: String,
}
