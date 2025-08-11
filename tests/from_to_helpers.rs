mod utils;

use crate::utils::cleanup_xml;
use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::info::Info;
use zusi_xml_lib::xml::zusi::result::ZusiResult;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};

const EXPECTED_SERIALIZED: &'static str = r#"
    <Zusi>
        <Info DateiTyp="" Version="" MinVersion=""/>
        <result Zugnummer="" TfNummer="" Datum="2023-04-05 06:07:08" Verbrauch="0" Bemerkung="" Schummel="false"
                Schwierigkeitsgrad="0" EnergieVorgabe="0"/>
    </Zusi>
"#;

fn expected_deserialized() -> Zusi {
    Zusi {
        info: Info {
            datei_typ: "".into(),
            version: "".into(),
            min_version: "".into(),
            autor_eintrag: None,
            datei: None,
            unknown: HashMap::new(),
        },
        value: ZusiValue::Result(ZusiResult {
            zugnummer: "".into(),
            tf_nummer: "".into(),
            anfang_datum: None,
            datum: datetime!(2023-04-05 06:07:08),
            verbrauch: 0.0,
            bemerkung: "".into(),
            schummel: false,
            schwierigkeitsgrad: 0,
            energie_vorgabe: 0.0,
            fahrt_eintraege: vec![],
            unknown: HashMap::new(),
        }),
    }
}

#[test]
fn test_from_xml() {
    let zusi = Zusi::from_xml(EXPECTED_SERIALIZED).unwrap();
    assert_eq!(zusi, expected_deserialized());
}

#[test]
fn test_from_xml_file_by_path() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("xml_input_file.xml");    
    fs::write(&file_path, EXPECTED_SERIALIZED).unwrap();

    let zusi: Zusi = Zusi::from_xml_file_by_path(&file_path).unwrap();

    assert_eq!(zusi, expected_deserialized());
}

#[test]
fn test_to_xml() {
    let xml = expected_deserialized().to_xml().unwrap();
    assert_eq!(xml, cleanup_xml(EXPECTED_SERIALIZED.into()));
}

#[test]
fn test_to_xml_file_by_path() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("xml_output_file.xml");

    expected_deserialized().to_xml_file_by_path(&file_path).unwrap();

    let xml = fs::read_to_string(&file_path).unwrap();
    assert_eq!(xml, cleanup_xml(EXPECTED_SERIALIZED.into()));
}