#[cfg(feature = "builder")]
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::xml::format::date_time_format;
use crate::xml::format::date_time_format_option;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum FahrtTyp {
    #[serde(rename = "0")]
    Standard = 0,

    #[serde(rename = "1")]
    Erzwingen = 1,

    #[serde(rename = "2")]
    Planhalt = 2,

    #[serde(rename = "3")]
    FahrfehlerSchwer = 3,

    #[serde(rename = "4")]
    FahrfehlerLeicht = 4,

    #[serde(rename = "5")]
    Streckeninfo = 5,

    #[serde(rename = "6")]
    SichtAnfg = 6,

    #[serde(rename = "7")]
    SichtEnde = 7,

    #[serde(rename = "8")]
    Bedienung = 8,
}

impl Default for FahrtTyp {
    fn default() -> Self {
        FahrtTyp::Standard
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(Builder))]
pub struct FahrtEintrag {
    #[serde(rename = "@FahrtTyp")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_typ: FahrtTyp,

    #[serde(rename = "@FahrtWeg")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_weg: f32,

    #[serde(rename = "@FahrtZeit")]
    #[serde(with = "date_time_format")]
    pub fahrt_zeit: PrimitiveDateTime,

    #[serde(rename = "@Fahrtsp")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_speed: f32,

    #[serde(rename = "@FahrtspStrecke")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_speed_strecke: f32,

    #[serde(rename = "@FahrtspSignal")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_speed_signal: f32,

    #[serde(rename = "@FahrtspZugsicherung")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_speed_zugsicherung: f32,

    #[serde(rename = "@FahrtAutopilot")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_autopilot: bool,

    #[serde(rename = "@Fahrtkm")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_km: f32,

    #[serde(rename = "@FahrtHLDruck")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_hl_druck: f32,

    #[serde(rename = "@FahrtParameter")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_parameter: u32,

    #[serde(rename = "@FahrtFplAnk")]
    #[serde(with = "date_time_format_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_fpl_ank: Option<PrimitiveDateTime>,

    #[serde(rename = "@FahrtFplAbf")]
    #[serde(with = "date_time_format_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_fpl_abf: Option<PrimitiveDateTime>,

    #[serde(rename = "@FahrtFBSchalter")]
    #[serde(default)]
    #[builder(default)]
    pub fahrt_fb_schalter: u32,
}