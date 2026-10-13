pub mod signal_frame;
pub mod hauptsignal_begriff;
pub mod vorsignal_begriff;
pub mod matrix_eintrag;
pub mod ersatz_signal;
pub mod koppel_signal;

use serde::{Deserialize, Serialize};
use serde_helpers::default::IsDefault;
use serde_helpers::with::bool_as_int::bool_as_int_format;
use std::collections::HashMap;
use typed_builder::TypedBuilder;
use crate::xml::zusi::lib::punkt_xyz::PunktXYZ;
use crate::xml::zusi::strecke::strecken_element::info_richtung::signal::ersatz_signal::ErsatzSignal;
use crate::xml::zusi::strecke::strecken_element::info_richtung::signal::hauptsignal_begriff::HauptsignalBegriff;
use crate::xml::zusi::strecke::strecken_element::info_richtung::signal::koppel_signal::KoppelSignal;
use crate::xml::zusi::strecke::strecken_element::info_richtung::signal::matrix_eintrag::MatrixEintrag;
use crate::xml::zusi::strecke::strecken_element::info_richtung::signal::signal_frame::SignalFrame;
use crate::xml::zusi::strecke::strecken_element::info_richtung::signal::vorsignal_begriff::VorsignalBegriff;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
pub struct Signal {
    #[serde(rename = "@NameBetriebsstelle", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub name_betriebsstelle: String,

    #[serde(rename = "@Stellwerk", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub stellwerk: String,

    #[serde(rename = "@Signalname", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_name: String,

    #[serde(rename = "@SignalnameEinblendung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_name_einblendung: String,

    #[serde(rename = "@ZufallsWert", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zufalls_wert: f32,

    #[serde(rename = "@AusbilderStandortLinks", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ausbilder_standort_links: bool,

    #[serde(rename = "@AusbilderSchriftVerschiebungX", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ausbilder_schrift_verschiebung_x: f32,

    #[serde(rename = "@AusbilderSchriftVerschiebungY", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ausbilder_schrift_verschiebung_y: f32,

    #[serde(rename = "@AusbilderSignalVerschiebungX", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ausbilder_signal_verschiebung_x: f32,

    #[serde(rename = "@AusbilderSignalVerschiebungY", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ausbilder_signal_verschiebung_y: f32,

    #[serde(rename = "@SignalFlags", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_flags: i32,

    #[serde(rename = "@SignalTyp", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_typ: u8, // TODO: replace with enum

    #[serde(rename = "@Weichenbauart", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub weichen_bauart: u8, // TODO: replace with enum

    #[serde(rename = "@WeichenGrundstellung", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub weichen_grundstellung: u8, // TODO: replace with enum

    #[serde(rename = "@WeicheStumpfIgnorieren", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub weiche_stumpf_ignorieren: bool,

    #[serde(rename = "@KeinHaltfallBeiFahrstrAufloesen", with = "bool_as_int_format", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub kein_haltfall_bei_fahrstrasse_aufloesen: bool,

    #[serde(rename = "@BoundingR", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub bounding_radius: i32,

    #[serde(rename = "@Zwangshelligkeit", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub zwangs_helligkeit: f32,

    #[serde(rename = "p")]
    pub position: PunktXYZ,

    #[serde(rename = "phi")]
    pub phi: PunktXYZ,

    #[serde(rename = "SignalFrame", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub signal_frame: Vec<SignalFrame>,

    #[serde(rename = "HsigBegriff", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub hauptsignal_begriff: Vec<HauptsignalBegriff>,

    #[serde(rename = "VsigBegriff", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub vorsignal_begriff: Vec<VorsignalBegriff>,

    #[serde(rename = "MatrixEintrag", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub matrix_eintraege: Vec<MatrixEintrag>,

    #[serde(rename = "Ersatzsignal", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub ersatz_signale: Vec<ErsatzSignal>,

    #[serde(rename = "KoppelSignal", default, skip_serializing_if = "IsDefault::is_default")]
    #[builder(default)]
    pub koppel_signal: Vec<KoppelSignal>,

    #[serde(flatten)]
    #[builder(default)]
    pub _unknown: HashMap<String, String>,
}