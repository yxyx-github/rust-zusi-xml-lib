use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ereignis {
    #[serde(rename = "@Er")]
    #[serde(default)]
    #[builder(default)]
    pub ereignis: i32, // TODO: replace with enum?

    #[serde(rename = "@Wert")]
    #[serde(default)]
    #[builder(default)]
    pub wert: f32,

    #[serde(rename = "@Beschr")]
    #[serde(default)]
    #[builder(default)]
    pub beschreibung: String,
}
