use crate::serde_helpers::date_time_option_format;
use crate::xml::zusi::lib::fahrplan_eintrag::FahrplanEintragsTyp;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanAnkunft {
    #[serde(rename = "@Ank")]
    #[serde(with = "date_time_option_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub ankunft: Option<PrimitiveDateTime>,

    #[serde(rename = "@FplEintrag")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_eintrag: FahrplanEintragsTyp,
}
