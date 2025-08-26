use crate::serde_helpers::IsDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanTunnel {
    #[serde(rename = "@FplNameText", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_name_text: String,

    #[serde(rename = "@FplTunnelAnfang", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_tunnel_anfang: bool,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
