pub mod loesch_fahrstrasse;
pub mod huell_kurve;
pub mod etcs_funkmast;
pub mod sky_box;
pub mod sky_dome;
pub mod strecken_standort;
pub mod nachbar_modul;
pub mod referenz_element;
pub mod fahrstrasse;
pub mod strecken_element;

use crate::xml::zusi::lib::beschreibung::Beschreibung;
use crate::xml::zusi::lib::datei::Datei;
use crate::xml::zusi::lib::utm::UTM;
use crate::xml::zusi::strecke::etcs_funkmast::ETCSFunkmast;
use crate::xml::zusi::strecke::fahrstrasse::Fahrstrasse;
use crate::xml::zusi::strecke::huell_kurve::HuellKurve;
use crate::xml::zusi::strecke::loesch_fahrstrasse::LoeschFahrstrasse;
use crate::xml::zusi::strecke::nachbar_modul::NachbarModul;
use crate::xml::zusi::strecke::referenz_element::ReferenzElement;
use crate::xml::zusi::strecke::sky_box::SkyBox;
use crate::xml::zusi::strecke::sky_dome::SkyDome;
use crate::xml::zusi::strecke::strecken_element::StreckenElement;
use crate::xml::zusi::strecke::strecken_standort::StreckenStandort;
use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Strecke {
    #[serde(rename = "@RekTiefe", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub rekursions_tiefe: i32,

    #[serde(rename = "@Himmelsmodell", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub himmels_modell: i32,

    #[serde(rename = "@SaegelinienEreignisse", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub saegelinien_ereignisse: bool,

    #[serde(rename = "Datei")]
    pub datei: Datei,

    #[serde(rename = "HintergrundDatei", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub hintergrund_datei: Option<Datei>,

    #[serde(rename = "BefehlsKonfiguration")]
    pub befehls_konfiguration: Datei,

    #[serde(rename = "Kachelpfad")]
    pub kachel_pfad: Datei,

    #[serde(rename = "Beschreibung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub beschreibung: Option<Beschreibung>,

    #[serde(rename = "LoeschFahrstrasse", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub loesch_fahrstrasse: Option<LoeschFahrstrasse>,

    #[serde(rename = "UTM")]
    pub utm: UTM,

    #[serde(rename = "Huellkurve")]
    pub huell_kurve: HuellKurve,

    #[serde(rename = "ETCSFunkmast", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub etcs_funkmast: Option<ETCSFunkmast>,

    #[serde(rename = "Skybox", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub sky_box: Option<SkyBox>,

    #[serde(rename = "SkyDome", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub sky_dome: Option<SkyDome>,

    #[serde(rename = "StreckenStandort", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub strecken_standorte: Vec<StreckenStandort>,

    #[serde(rename = "ModulDateien", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub nachbar_module: Vec<NachbarModul>,

    #[serde(rename = "ReferenzElemente", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub referenz_elemente: Vec<ReferenzElement>,

    #[serde(rename = "StrElement", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub strecken_elemente: Vec<StreckenElement>,

    #[serde(rename = "Fahrstrasse", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub fahrstassen: Vec<Fahrstrasse>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}