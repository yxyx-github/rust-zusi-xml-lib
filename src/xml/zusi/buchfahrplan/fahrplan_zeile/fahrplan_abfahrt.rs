use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;
use crate::serde_helpers::date_time_option_format;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanAbfahrt {
    #[serde(rename = "@Abf")]
    #[serde(with = "date_time_option_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub abfahrt: Option<PrimitiveDateTime>,
}
