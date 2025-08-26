use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zusi_xml_lib::xml::zusi::lib::datei::Datei;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields, rename = "Dateien")]
pub struct DateiTestWrapper {
    #[serde(rename = "Datei")]
    pub value: Vec<Datei>,
}

#[allow(dead_code)]
pub fn dateien() -> DateiTestWrapper {
    DateiTestWrapper {
        value: vec![
            Datei {
                dateiname: "./a/b/c.d".into(),
                inst: 1,
                nur_info: true,
                _unknown: HashMap::new(),
            },
            Datei {
                dateiname: "a/b/c.d".into(),
                inst: 1,
                nur_info: true,
                _unknown: HashMap::new(),
            },
            Datei {
                dateiname: "/a/b/c.d".into(),
                inst: 1,
                nur_info: true,
                _unknown: HashMap::new(),
            },
            Datei::builder().build(),
        ],
    }
}