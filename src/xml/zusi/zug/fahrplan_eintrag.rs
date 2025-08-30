pub mod fahrplan_signal_eintrag;
pub mod fahrplan_fahrstrasse_eintrag;
pub mod fahrplan_vorgang;

use serde_helpers::with::date_time::date_time_option_format;
use serde_helpers::default::IsDefault;
use crate::xml::zusi::lib::ereignis::Ereignis;
use crate::xml::zusi::lib::fahrplan_eintrag::FahrplanEintragsTyp;
use crate::xml::zusi::zug::fahrplan_eintrag::fahrplan_signal_eintrag::FahrplanSignalEintrag;
use crate::xml::zusi::zug::fahrplan_eintrag::fahrplan_vorgang::FahrplanVorgang;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct FahrplanEintrag {
    #[serde(rename = "@Ank", with = "date_time_option_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ankunft: Option<PrimitiveDateTime>,

    #[serde(rename = "@Abf", with = "date_time_option_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub abfahrt: Option<PrimitiveDateTime>,

    #[serde(rename = "@Signalvorlauf", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_vorlauf: f32,

    #[serde(rename = "@SignalBleibtAufHalt", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_bleibt_auf_halt: bool,

    #[serde(rename = "@Betrst", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub betriebsstelle: String,

    #[serde(rename = "@FplEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_eintrag: FahrplanEintragsTyp,

    #[serde(rename = "@ErsatzsignalzeilePlus1", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ersatz_signal_zeile_plus_1: i32,

    #[serde(rename = "@FzgVerbandAktionWendesignal", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrzeug_verband_aktion_wende_signal: bool,

    #[serde(rename = "@KuerzungLoeschen", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub kuerzung_loeschen: bool,

    #[serde(rename = "@FzgVerbandAktionWendeSignalabstand", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrzeug_verband_aktion_wende_signal_abstand: f32,

    #[serde(rename = "FahrplanSignalEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_signal_eintraege: Vec<FahrplanSignalEintrag>,

    #[serde(rename = "FahrplanFahrstrasseEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_fahrstrassen_eintraege: Vec<FahrplanSignalEintrag>,

    #[serde(rename = "FahrplanVorgangEreignisse", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_vorgang_ereignisse: Option<FahrplanVorgang>,

    #[serde(rename = "FahrplanVorgangFahrstrasse", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrplan_vorgang_fahrstrasse: Option<FahrplanVorgang>,

    #[serde(rename = "Ereignis", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ereignisse: Vec<Ereignis>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}