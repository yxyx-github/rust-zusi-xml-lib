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
use crate::utils::read_xml_file;

fn expected_serialized(include_optionals: bool) -> String {
    read_xml_file(format!("./tests/xml/result{}.xml", if include_optionals { "" } else {"-without-optionals" }))
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
            ZusiValue::Result(
                ZusiResult {
                    zugnummer: "12345".into(),
                    tf_nummer: "67890".into(),
                    datum: datetime!(2019-01-01 23:14),
                    verbrauch: 0.0,
                    bemerkung: "".to_string(),
                    schummel: false,
                    schwierigkeitsgrad: 0,
                    energie_vorgabe: 0.0,
                    value: vec![
                        ResultValue::FahrtEintrag(FahrtEintrag {
                            fahrt_typ: FahrtTyp::Standard,
                            fahrt_weg: 22.33,
                            fahrt_zeit: datetime!(2019-01-01 23:18),
                            fahrt_speed: 0.0,
                            fahrt_speed_strecke: 0.0,
                            fahrt_speed_signal: 0.0,
                            fahrt_speed_zugsicherung: 0.0,
                            fahrt_autopilot: false,
                            fahrt_km: 0.0,
                            fahrt_hl_druck: 0.0,
                            fahrt_parameter: 0,
                            fahrt_fpl_ank: if include_optionals { Some(DelphiTimestamp::from(43410.3125)) } else { None },
                            fahrt_fpl_abf: if include_optionals { Some(DelphiTimestamp::from(43410.316406)) } else { None },
                            fahrt_fb_schalter: 0,
                        }),
                        ResultValue::FahrtEintrag(FahrtEintrag {
                            fahrt_typ: FahrtTyp::Standard,
                            fahrt_weg: 22.43,
                            fahrt_zeit: datetime!(2019-01-02 1:07),
                            fahrt_speed: 0.0,
                            fahrt_speed_strecke: 0.0,
                            fahrt_speed_signal: 0.0,
                            fahrt_speed_zugsicherung: 0.0,
                            fahrt_autopilot: false,
                            fahrt_km: 0.0,
                            fahrt_hl_druck: 0.0,
                            fahrt_parameter: 0,
                            fahrt_fpl_ank: if include_optionals { Some(DelphiTimestamp::from(43410.347656)) } else { None },
                            fahrt_fpl_abf: if include_optionals { Some(DelphiTimestamp::from(43410.351563)) } else { None },
                            fahrt_fb_schalter: 0,
                        }),
                    ],
                }
            ),
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
