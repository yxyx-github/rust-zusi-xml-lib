use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::punkt_xyz::PunktXYZ;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct HuellKurve {
    #[serde(rename = "PunktXYZ", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub punkte: Vec<PunktXYZ>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}