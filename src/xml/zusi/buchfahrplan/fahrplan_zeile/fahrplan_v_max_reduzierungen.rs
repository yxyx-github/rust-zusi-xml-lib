use crate::serde_helpers::IsDefault;
use crate::xml::zusi::lib::ereignis::Ereignis;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanVMaxReduzierungen {
    #[serde(rename = "Ereignis", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub v_max_reduzierungen: Vec<Ereignis>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}