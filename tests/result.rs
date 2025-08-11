mod utils;

use crate::utils::TestRunner;
use std::collections::HashMap;
use time::macros::datetime;
use zusi_xml_lib::delphi_timestamp::DelphiTimestamp;
use zusi_xml_lib::xml::zusi::info::Info;
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};
use zusi_xml_lib::xml::zusi::result::ZusiResult;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};

const XML_PATH: &str = "./tests/xml/result.xml";
const XML_PATH_WITHOUT_OPTIONALS: &str = "./tests/xml/result-without-optionals.xml";

fn expected_deserialized(include_optionals: bool) -> Zusi {
    Zusi {
        info: Info {
            datei_typ: "result".into(),
            version: "A.1".into(),
            min_version: "A.0".into(),
            autor_eintrag: None,
            datei: None,
            unknown: HashMap::new(),
        },
        value: ZusiValue::Result(ZusiResult {
            zugnummer: "12345".into(),
            tf_nummer: "67890".into(),
            anfang_datum: if include_optionals { Some(datetime!(2019-01-01 23:16)) } else { None },
            datum: datetime!(2019-01-01 23:14),
            verbrauch: 0.0,
            bemerkung: "".to_string(),
            schummel: false,
            schwierigkeitsgrad: 0,
            energie_vorgabe: 0.0,
            fahrt_eintraege: vec![
                FahrtEintrag {
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
                    fahrt_text: "ADorf".into(),
                    fahrt_fpl_ank: if include_optionals { Some(DelphiTimestamp::from(43410.3125)) } else { None },
                    fahrt_fpl_abf: if include_optionals { Some(DelphiTimestamp::from(43410.316406)) } else { None },
                    fahrt_fb_schalter: 0,
                },
                FahrtEintrag {
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
                    fahrt_text: "BDorf".into(),
                    fahrt_fpl_ank: if include_optionals { Some(DelphiTimestamp::from(43410.347656)) } else { None },
                    fahrt_fpl_abf: if include_optionals { Some(DelphiTimestamp::from(43410.351563)) } else { None },
                    fahrt_fb_schalter: 0,
                },
            ],
            unknown: HashMap::new(),
        }),
    }
}

#[test]
fn test_serialize() {
    TestRunner::new_with_file(XML_PATH, expected_deserialized(true)).test_serialize();
}

#[test]
fn test_serialize_optionals() {
    TestRunner::new_with_file(XML_PATH_WITHOUT_OPTIONALS, expected_deserialized(false)).test_serialize();
}

#[test]
fn test_deserialize() {
    TestRunner::new_with_file(XML_PATH, expected_deserialized(true)).test_serialize();
}

#[test]
fn test_deserialize_optionals() {
    TestRunner::new_with_file(XML_PATH_WITHOUT_OPTIONALS, expected_deserialized(false)).test_serialize();
}