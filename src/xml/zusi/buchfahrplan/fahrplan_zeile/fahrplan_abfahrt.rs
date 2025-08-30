use serde_helpers::with::date_time::date_time_format;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanAbfahrt {
    #[serde(rename = "@Abf", with = "date_time_format")]
    pub abfahrt: PrimitiveDateTime,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
