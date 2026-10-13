pub mod fahrstrasse_start;
pub mod fahrstrasse_ziel;
pub mod fahrstrasse_register;
pub mod fahrstrasse_aufloesung;
pub mod fahrstrasse_teil_aufloesung;
pub mod fahrstrasse_signal_haltfall;
pub mod fahrstrasse_weiche;
pub mod fahrstrasse_signal;
pub mod fahrstrasse_vorsignal;

use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_aufloesung::FahrstrasseAufloesung;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_register::FahrstrasseRegister;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_signal::FahrstrasseSignal;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_signal_haltfall::FahrstrasseSignalHaltfall;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_start::FahrstrasseStart;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_teil_aufloesung::FahrstrasseTeilAufloesung;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_vorsignal::FahrstrasseVorsignal;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_weiche::FahrstrasseWeiche;
use crate::xml::zusi::strecke::fahrstrasse::fahrstrasse_ziel::FahrstrasseZiel;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum FahrstrassenTyp {
    #[serde(rename = "TypZug")]
    TypZug,
}

impl Default for FahrstrassenTyp {
    fn default() -> Self {
        FahrstrassenTyp::TypZug
    }
}

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Fahrstrasse {
    #[serde(rename = "@FahrstrName", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrassen_name: String,

    #[serde(rename = "@FahrstrStrecke", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrassen_strecke: String,

    #[serde(rename = "@RglGgl", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub rgl_ggl: i32,

    #[serde(rename = "@FahrstrTyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrassen_typ: FahrstrassenTyp,

    #[serde(rename = "@ZufallsWert", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zufalls_wert: f32,

    #[serde(rename = "@Laenge", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub laenge: f32,

    #[serde(rename = "FahrstrStart")]
    pub fahrstrasse_start: FahrstrasseStart,

    #[serde(rename = "FahrstrZiel")]
    pub fahrstrasse_ziel: FahrstrasseZiel,

    #[serde(rename = "FahrstrRegister", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_register: Vec<FahrstrasseRegister>,

    #[serde(rename = "FahrstrAufloesung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_aufloesung: Vec<FahrstrasseAufloesung>,

    #[serde(rename = "FahrstrTeilaufloesung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_teil_aufloesung: Vec<FahrstrasseTeilAufloesung>,

    #[serde(rename = "FahrstrSigHaltfall", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_signal_haltfall: Vec<FahrstrasseSignalHaltfall>,

    #[serde(rename = "FahrstrWeiche", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_weiche: Vec<FahrstrasseWeiche>,

    #[serde(rename = "FahrstrSignal", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_signal: Vec<FahrstrasseSignal>,

    #[serde(rename = "FahrstrVSignal", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstrasse_vorsignal: Vec<FahrstrasseVorsignal>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}