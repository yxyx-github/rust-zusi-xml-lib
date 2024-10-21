pub mod zug_eintrag;
mod strecken_modul;

use crate::serde_helpers::date_time_option_format;
use crate::xml::zusi::fahrplan::strecken_modul::StreckenModul;
use crate::xml::zusi::fahrplan::zug_eintrag::ZugEintrag;
use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::lib::utm::UTM;
use crate::xml::zusi::zug::Zug;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Fahrplan {
    #[serde(rename = "@AnfangsZeit")]
    #[serde(with = "date_time_option_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub anfangs_zeit: Option<PrimitiveDateTime>,

    #[serde(rename = "@Zeitmodus")]
    #[serde(default)]
    #[builder(default)]
    pub zeitmodus: i32,

    #[serde(rename = "@ChaosVorschlagen")]
    #[serde(default)]
    #[builder(default)]
    pub chaos_vorschlagen: bool,

    #[serde(rename = "@ChaosVorschlag")]
    #[serde(default)]
    #[builder(default)]
    pub chaos_vorschlag: f32,

    #[serde(rename = "@trnDateien")]
    #[serde(default)]
    #[builder(default)]
    pub trn_dateien: bool,

    #[serde(rename = "$value")]
    pub value: Vec<FahrplanValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrplanValue {
    #[serde(rename = "BefehlsKonfiguration")]
    BefehlsKonfiguration(Datei),

    #[serde(rename = "LaPDF")]
    LaPDF(Datei),

    #[serde(rename = "StrebuPDF")]
    StreckenbuchPDF(Datei),

    #[serde(rename = "ErsatzfahrplaenePDF")]
    ErsatzfahrplaenePDF(Datei),

    #[serde(rename = "Begruessungsdatei")]
    Begruessungsdatei(Datei),

    #[serde(rename = "Zug")]
    ZugEintrag(ZugEintrag),

    #[serde(rename = "trn")]
    Zug(Zug),

    #[serde(rename = "StrModul")]
    StreckenModul(StreckenModul),

    #[serde(rename = "UTM")]
    UTM(UTM),
}
