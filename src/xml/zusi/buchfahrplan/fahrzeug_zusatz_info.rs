use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrzeugZusatzInfo {
    #[serde(rename = "@FahrzeugZusatzinfo", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrzeug_zusatzinfo: String,
}
