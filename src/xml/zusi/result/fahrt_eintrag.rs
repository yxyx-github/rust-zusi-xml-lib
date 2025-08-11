use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;
use crate::delphi_timestamp::DelphiTimestamp;
use crate::serde_helpers::{date_time_format, delphi_timestamp_option_format};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrtTyp {
    #[serde(rename = "0")]
    Standard,

    #[serde(rename = "1")]
    Erzwingen,

    #[serde(rename = "2")]
    Planhalt,

    #[serde(rename = "3")]
    FahrfehlerSchwer,

    #[serde(rename = "4")]
    FahrfehlerLeicht,

    #[serde(rename = "5")]
    Streckeninfo,

    #[serde(rename = "6")]
    SichtAnfg,

    #[serde(rename = "7")]
    SichtEnde,

    #[serde(rename = "8")]
    Bedienung,
}

impl Default for FahrtTyp {
    fn default() -> Self {
        FahrtTyp::Standard
    }
}

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrtEintrag {
    #[serde(rename = "@FahrtTyp", default)]
    #[builder(default)]
    pub fahrt_typ: FahrtTyp,

    #[serde(rename = "@FahrtWeg", default)]
    #[builder(default)]
    pub fahrt_weg: f32,

    #[serde(rename = "@FahrtZeit", with = "date_time_format")]
    pub fahrt_zeit: PrimitiveDateTime,

    #[serde(rename = "@Fahrtsp", default)]
    #[builder(default)]
    pub fahrt_speed: f32,

    #[serde(rename = "@FahrtspStrecke", default)]
    #[builder(default)]
    pub fahrt_speed_strecke: f32,

    #[serde(rename = "@FahrtspSignal", default)]
    #[builder(default)]
    pub fahrt_speed_signal: f32,

    #[serde(rename = "@FahrtspZugsicherung", default)]
    #[builder(default)]
    pub fahrt_speed_zugsicherung: f32,

    #[serde(rename = "@FahrtAutopilot", default)]
    #[builder(default)]
    pub fahrt_autopilot: bool,

    #[serde(rename = "@Fahrtkm", default)]
    #[builder(default)]
    pub fahrt_km: f32,

    #[serde(rename = "@FahrtHLDruck", default)]
    #[builder(default)]
    pub fahrt_hl_druck: f32,

    #[serde(rename = "@FahrtParameter", default)]
    #[builder(default)]
    pub fahrt_parameter: u32,

    #[serde(rename = "@FahrtText", default)]
    #[builder(default)]
    pub fahrt_text: String,

    #[serde(
        rename = "@FahrtFplAnk",
        with = "delphi_timestamp_option_format",
        default,
        skip_serializing_if = "Option::is_none",
    )]
    #[builder(default)]
    pub fahrt_fpl_ank: Option<DelphiTimestamp>,

    #[serde(
        rename = "@FahrtFplAbf",
        with = "delphi_timestamp_option_format",
        default,
        skip_serializing_if = "Option::is_none",
    )]
    #[builder(default)]
    pub fahrt_fpl_abf: Option<DelphiTimestamp>,

    #[serde(rename = "@FahrtFBSchalter", default)]
    #[builder(default)]
    pub fahrt_fb_schalter: u32,
}

impl AsRef<FahrtEintrag> for FahrtEintrag {
    fn as_ref(&self) -> &FahrtEintrag {
        &self
    }
}
