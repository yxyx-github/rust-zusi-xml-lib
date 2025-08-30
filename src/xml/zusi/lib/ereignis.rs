use serde_helpers::default::IsDefault;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Ereignis {
    #[serde(rename = "@Er", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ereignis: i32, // TODO: replace with enum?

    #[serde(rename = "@Wert", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub wert: f32,

    #[serde(rename = "@Beschr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub beschreibung: String,
}
