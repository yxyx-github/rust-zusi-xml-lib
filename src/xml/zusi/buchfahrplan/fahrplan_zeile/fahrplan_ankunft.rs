use crate::serde_helpers::IsDefault;
use crate::serde_helpers::date_time_format;
use crate::xml::zusi::lib::fahrplan_eintrag::FahrplanEintragsTyp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanAnkunft {
    #[serde(rename = "@Ank", with = "date_time_format")]
    pub ankunft: PrimitiveDateTime,

    #[serde(rename = "@FplEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_eintrag: FahrplanEintragsTyp,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}
