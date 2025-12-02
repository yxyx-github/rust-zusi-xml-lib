mod utils;

use serde_helpers::xml::FromXML;
use std::fs;
use tempfile::tempdir;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::info::{DateiTyp, Info};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtEintrag;
use zusi_xml_lib::xml::zusi::result::ZusiResult;
use zusi_xml_lib::xml::zusi::TypedZusi;

const EXPECTED_SERIALIZED: &'static str = r#"
    <Zusi>
        <Info DateiTyp="result" Version="A.4" MinVersion="A.3"/>
        <result Zugnummer="00000" TfNummer="12345" Datum="2019-01-01 23:14:00">
            <FahrtEintrag FahrtZeit="2019-01-01 23:18:00"/>
        </result>
    </Zusi>
"#;

fn expected_deserialized() -> TypedZusi<ZusiResult> {
    TypedZusi::builder()
        .info(
            Info::builder()
                .datei_typ(DateiTyp::Result)
                .version("A.4".into())
                .min_version("A.3".into())
                .build()
        )
        .value(
            ZusiResult::builder()
                .zugnummer("00000".into())
                .tf_nummer("12345".into())
                .datum(Some(datetime!(2019-01-01 23:14)))
                .fahrt_eintraege(vec![
                    FahrtEintrag::builder()
                        .fahrt_zeit(datetime!(2019-01-01 23:18))
                        .build()
                ])
                .build()
        )
        .build()
}

#[test]
fn test_from_xml() {
    let zusi = TypedZusi::from_xml(EXPECTED_SERIALIZED).unwrap();
    assert_eq!(zusi, expected_deserialized());
}

#[test]
fn test_from_xml_file_by_path() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("some_dir/xml_input_file.xml");
    fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    fs::write(&file_path, EXPECTED_SERIALIZED).unwrap();

    let zusi = TypedZusi::from_xml_file_by_path(&file_path).unwrap();

    assert_eq!(zusi, expected_deserialized());
}