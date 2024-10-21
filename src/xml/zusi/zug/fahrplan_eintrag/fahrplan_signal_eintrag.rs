use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanSignalEintrag {
    #[serde(rename = "@FahrplanSignal")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_signal: String,
}
