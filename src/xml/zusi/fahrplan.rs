pub mod zug_datei_eintrag;
pub mod strecken_modul;

use serde_helpers::with::bool_as_int::bool_as_int_format;
use serde_helpers::with::date_time::date_time_format;
use serde_helpers::default::IsDefault;
use crate::xml::zusi::fahrplan::strecken_modul::StreckenModul;
use crate::xml::zusi::fahrplan::zug_datei_eintrag::ZugDateiEintrag;
use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::lib::utm::UTM;
use crate::xml::zusi::zug::Zug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Fahrplan {
    #[serde(rename = "@AnfangsZeit", with = "date_time_format")]
    pub anfangs_zeit: PrimitiveDateTime,

    #[serde(rename = "@Zeitmodus", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zeitmodus: i32,

    #[serde(rename = "@ChaosVorschlagen", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub chaos_vorschlagen: bool,

    #[serde(rename = "@ChaosVorschlag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub chaos_vorschlag: f32,

    #[serde(rename = "@trnDateien", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub trn_dateien: bool,

    #[serde(rename = "BefehlsKonfiguration")]
    pub befehls_konfiguration: Datei,

    #[serde(rename = "LaPDF")]
    pub la_pdf: Datei,

    #[serde(rename = "StrebuPDF")]
    pub streckenbuch_pdf: Datei,

    #[serde(rename = "ErsatzfahrplaenePDF")]
    pub ersatzfahrplaene_pdf: Datei,

    #[serde(rename = "Begruessungsdatei")]
    pub begruessungs_datei: Datei,

    #[serde(rename = "trn", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_eintraege: Vec<Zug>,

    #[serde(rename = "Zug", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zug_dateien: Vec<ZugDateiEintrag>,

    #[serde(rename = "StrModul", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub strecken_module: Vec<StreckenModul>,

    #[serde(rename = "UTM")]
    pub utm: UTM,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}