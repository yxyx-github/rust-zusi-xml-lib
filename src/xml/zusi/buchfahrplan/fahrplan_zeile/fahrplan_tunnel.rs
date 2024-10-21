use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanTunnel {
    #[serde(rename = "@FplNameText")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_name_text: String,

    #[serde(rename = "@FplTunnelAnfang")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_tunnel_anfang: bool,
}
