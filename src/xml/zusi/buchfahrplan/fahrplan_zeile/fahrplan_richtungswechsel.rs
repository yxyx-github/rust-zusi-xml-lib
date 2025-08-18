use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
pub struct FahrplanRichtungswechsel {
    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
