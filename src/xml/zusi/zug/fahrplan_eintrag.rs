pub mod fahrplan_signal_eintrag;
pub mod fahrplan_fahrstrasse_eintrag;
mod fahrplan_vorgang;

use crate::serde_helpers::date_time_option_format;
use crate::xml::zusi::zug::fahrplan_eintrag::fahrplan_fahrstrasse_eintrag::FahrplanFahrstrasseEintrag;
use crate::xml::zusi::zug::fahrplan_eintrag::fahrplan_signal_eintrag::FahrplanSignalEintrag;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_reduzierungen::ereignis::Ereignis;
use crate::xml::zusi::lib::fahrplan_eintrag::FahrplanEintragsTyp;
use crate::xml::zusi::zug::fahrplan_eintrag::fahrplan_vorgang::FahrplanVorgang;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanEintrag {
    #[serde(rename = "@Ank")]
    #[serde(with = "date_time_option_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub ankunft: Option<PrimitiveDateTime>,

    #[serde(rename = "@Abf")]
    #[serde(with = "date_time_option_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub abfahrt: Option<PrimitiveDateTime>,

    #[serde(rename = "@Betrst")]
    #[serde(default)]
    #[builder(default)]
    pub betriebsstelle: String,

    #[serde(rename = "@FplEintrag")]
    #[serde(default)]
    #[builder(default)]
    pub fahrplan_eintrag: FahrplanEintragsTyp,

    #[serde(rename = "@ErsatzsignalzeilePlus1")]
    #[serde(default)]
    #[builder(default)]
    pub ersatz_signal_zeile_plus_1: i32,

    #[serde(rename = "@FzgVerbandAktionWendesignal")]
    #[serde(default)]
    #[builder(default)]
    pub fahrzeug_verband_aktion_wende_signal: bool,

    #[serde(rename = "@FzgVerbandAktionWendeSignalabstand")]
    #[serde(default)]
    #[builder(default)]
    pub fahrzeug_verband_aktion_wende_signal_abstand: f32,

    #[serde(rename = "$value")]
    pub value: Option<Vec<FahrplanEintragValue>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrplanEintragValue {
    #[serde(rename = "FahrplanSignalEintrag")]
    FahrplanSignalEintrag(FahrplanSignalEintrag),

    #[serde(rename = "FahrplanFahrstrasseEintrag")]
    FahrplanFahrstrasseEintrag(FahrplanFahrstrasseEintrag),

    #[serde(rename = "FahrplanVorgangEreignisse")]
    FahrplanVorgangEreignisse(FahrplanVorgang),

    #[serde(rename = "FahrplanVorgangFahrstrasse")]
    FahrplanVorgangFahrstrasse(FahrplanVorgang),

    #[serde(rename = "Ereignis")]
    Ereignis(Ereignis),
}
