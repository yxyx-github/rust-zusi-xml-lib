use crate::delphi_timestamp::DelphiTimestamp;
use crate::serde_helpers::delphi_timestamp_option_format;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::date_time::date_time_format;
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FahrtEintrag {
    #[serde(rename = "@FahrtTyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_typ: FahrtTyp,

    #[serde(rename = "@FahrtWeg", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_weg: f32,

    #[serde(rename = "@FahrtZeit", with = "date_time_format")]
    pub fahrt_zeit: PrimitiveDateTime,

    #[serde(rename = "@Fahrtsp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_speed: f32,

    #[serde(rename = "@FahrtspStrecke", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_speed_strecke: f32,

    #[serde(rename = "@FahrtspSignal", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_speed_signal: f32,

    #[serde(rename = "@FahrtspZugsicherung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_speed_zugsicherung: f32,

    #[serde(rename = "@FahrtAutopilot", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_autopilot: bool,

    #[serde(rename = "@Fahrtkm", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_km: f32,

    #[serde(rename = "@FahrtHLDruck", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_hl_druck: f32,

    #[serde(rename = "@FahrtParameter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_parameter: u32,

    #[serde(rename = "@FahrtText", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_text: String,

    #[serde(rename = "@FahrtFplAnk", with = "delphi_timestamp_option_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_fpl_ank: Option<DelphiTimestamp>,

    #[serde(rename = "@FahrtFplAbf", with = "delphi_timestamp_option_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_fpl_abf: Option<DelphiTimestamp>,

    #[serde(rename = "@FahrtFBSchalter", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrt_fb_schalter: u32,
}

impl AsRef<FahrtEintrag> for FahrtEintrag {
    fn as_ref(&self) -> &FahrtEintrag {
        &self
    }
}
