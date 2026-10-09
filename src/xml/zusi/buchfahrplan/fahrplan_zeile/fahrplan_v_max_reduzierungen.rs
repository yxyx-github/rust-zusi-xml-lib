use crate::xml::zusi::lib::ereignis::Ereignis;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanVMaxReduzierungen {
    #[serde(rename = "@FplvMax", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_vmax: i32,

    #[serde(rename = "@FplPunktuell", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_punktuell: bool,

    #[serde(rename = "Ereignis", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub v_max_reduzierungen: Vec<Ereignis>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}