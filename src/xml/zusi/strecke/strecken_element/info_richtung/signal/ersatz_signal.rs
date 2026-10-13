use crate::xml::zusi::strecke::strecken_element::info_richtung::signal::matrix_eintrag::MatrixEintrag;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct ErsatzSignal {
    #[serde(rename = "@ErsatzsigBezeichnung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ersatz_signal_bezeichnung: String,

    #[serde(rename = "@ErsatzsigID", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_id: String, // TODO: D3DColor

    #[serde(rename = "MatrixEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub matrix_eintraege: Vec<MatrixEintrag>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}