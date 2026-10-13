pub mod nachfolger_element_in_modul;
pub mod nachfolger_element;
pub mod info_richtung;

use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use std::collections::HashMap;
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::punkt_xyz::PunktXYZ;
use crate::xml::zusi::strecke::strecken_element::info_richtung::InfoRichtung;
use crate::xml::zusi::strecke::strecken_element::nachfolger_element::NachfolgerElement;
use crate::xml::zusi::strecke::strecken_element::nachfolger_element_in_modul::NachfolgerElementInModul;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct StreckenElement {
    #[serde(rename = "@Nr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nummer: i32,

    #[serde(rename = "@Ueberh", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ueberhoehung: f32,

    #[serde(rename = "@kr", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub kreis_radius: f32,

    #[serde(rename = "@spTrass", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub speed_trasse: f32,

    #[serde(rename = "@Anschluss", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub anschluss: i32,

    #[serde(rename = "@Fkt", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub funktion: i64,

    #[serde(rename = "@Oberbau", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub oberbau: String,

    #[serde(rename = "@Volt", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub volt: u32, // TODO: replace with enum

    #[serde(rename = "@Drahthoehe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub draht_hoehe: f32,

    #[serde(rename = "@Zwangshelligkeit", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zwangs_helligkeit: f32,

    #[serde(rename = "g")]
    pub gruen: PunktXYZ,

    #[serde(rename = "b")]
    pub blau: PunktXYZ,

    #[serde(rename = "InfoNormRichtung")]
    pub info_norm_richtung: InfoRichtung,

    #[serde(rename = "InfoGegenRichtung")]
    pub info_gegen_richtung: InfoRichtung,

    #[serde(rename = "NachNorm", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nachfolger_element_norm: Vec<NachfolgerElement>,

    #[serde(rename = "NachGegen", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nachfolger_element_gegen: Vec<NachfolgerElement>,

    #[serde(rename = "NachNormModul", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nachfolger_element_in_modul_norm: Vec<NachfolgerElementInModul>,

    #[serde(rename = "NachGegenModul", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nachfolger_element_in_modul_gegen: Vec<NachfolgerElementInModul>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}