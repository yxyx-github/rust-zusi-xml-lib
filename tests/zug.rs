mod utils;

use std::fs::File;
use std::io::{Read, Write};
use quick_xml::{de, se};
use tempfile::tempdir;
use time::macros::datetime;
use zusi_xml_lib::delphi_timestamp::DelphiTimestamp;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::info::Info;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};
use zusi_xml_lib::xml::zusi::zug::{Zug, ZugValue};
use zusi_xml_lib::xml::zusi::zug::fahrplan_eintrag::FahrplanEintrag;
use crate::utils::read_xml_file;

fn expected_serialized(include_optionals: bool) -> String {
    read_xml_file(format!("./tests/xml/zug{}.xml", if include_optionals { "" } else {"-without-optionals" }))
}

fn expected_deserialized(include_optionals: bool) -> Zusi {
    Zusi {
        value: vec![
            ZusiValue::Info(
                Info {
                    datei_typ: "result".into(),
                    version: "A.1".into(),
                    min_version: "A.0".into(),
                    value: vec![],
                }
            ),
            ZusiValue::Zug(Zug {
                gattung: "RB".to_string(),
                nummer: "".to_string(),
                zuglauf: "".to_string(),
                baureihe_angabe: "".to_string(),
                prioritaet: 0,
                standort_modus: 0,
                start_vorschubweg: 0.0,
                bremsstellung_zug: Default::default(),
                fahrplan_bremsstellung_textvorgabe: "".to_string(),
                energie_vorgabe: 0.0,
                speed_anfang: 0.0,
                mindest_bremshundertstel: 0.0,
                fahrplan_masse: 0.0,
                fahrplan_zug_laenge: 0.0,
                verkehrstage: "".to_string(),
                grenzlast: false,
                speed_zug_niedriger: 0.0,
                autopilot_beschleunigung: 0.0,
                keine_vorplan_korrektur: false,
                dekozug: false,
                lod_zug: Default::default(),
                reisenden_dichte: 0.0,
                fahrplan_gruppe: "".to_string(),
                rekursionstiefe: 0,
                zugsicherung_startmodus: 0,
                cold_movement: false,
                fahrstrassen_name: "".to_string(),
                aufgleisen_register_pruefen: false,
                zug_typ: Default::default(),
                ueberschrift: "".to_string(),
                odt_datei_absolut: "".to_string(),
                buchfahrplan_einfach: false,
                buchfahrplan_dll: "".to_string(),
                tuer_system_bezeichner: "".to_string(),
                value: vec![
                    ZugValue::FahrplanEintrag(FahrplanEintrag {
                        ankunft: None,
                        abfahrt: None,
                        betriebsstelle: "".to_string(),
                        fahrplan_eintrag: Default::default(),
                        ersatz_signal_zeile_plus_1: 0,
                        fahrzeug_verband_aktion_wende_signal: false,
                        fahrzeug_verband_aktion_wende_signal_abstand: 0.0,
                        value: None,
                    })
                ],
            }),
        ],
    }
}

#[test]
fn test_serialize_deserialize() {
    let serialized = se::to_string(&expected_deserialized(true)).unwrap();
    assert_eq!(serialized, expected_serialized(true));

    let deserialized: Zusi = de::from_str(&serialized).unwrap();
    assert_eq!(deserialized, expected_deserialized(true));
}

#[test]
fn test_serialize_deserialize_optionals() {
    let serialized = se::to_string(&expected_deserialized(false)).unwrap();
    assert_eq!(serialized, expected_serialized(false));

    let deserialized: Zusi = de::from_str(&serialized).unwrap();
    assert_eq!(deserialized, expected_deserialized(false));
}

#[test]
fn test_from_xml() {
    let zusi = Zusi::from_xml(&expected_serialized(true)).unwrap();
    assert_eq!(zusi, expected_deserialized(true));
}

#[test]
fn test_from_xml_file_by_path() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("xml_input_file.xml");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(expected_serialized(true).as_bytes()).unwrap();

    let zusi: Zusi = Zusi::from_xml_file_by_path(&file_path).unwrap();

    assert_eq!(zusi, expected_deserialized(true));
}

#[test]
fn test_to_xml() {
    let xml = expected_deserialized(true).to_xml().unwrap();
    assert_eq!(xml, expected_serialized(true));
}

#[test]
fn test_to_xml_file_by_path() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("xml_output_file.xml");

    expected_deserialized(true).to_xml_file_by_path(&file_path).unwrap();

    let mut file = File::open(&file_path).unwrap();
    let mut xml = String::new();
    file.read_to_string(&mut xml).unwrap();
    assert_eq!(xml, expected_serialized(true));
}
