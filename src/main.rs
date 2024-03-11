use quick_xml::{de, se};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Zusi {
    #[serde(rename = "$value")]
    value: Vec<ZusiValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum ZusiValue {
    Info(Info),
    #[serde(rename = "result")]
    Result(ZusiResult),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Info {
    #[serde(rename = "@DateiTyp")]
    datei_typ: String,
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "@MinVersion")]
    min_version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ZusiResult {
    #[serde(rename = "$value")]
    value: Vec<ResultValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum ResultValue {
    FahrtEintrag(FahrtEintrag),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct FahrtEintrag {
    #[serde(rename = "@FahrtTyp")]
    fahrt_typ: u32, // TODO: check correct type
    #[serde(rename = "@FahrtWeg")]
    fahrt_weg: f32, // TODO: check correct type
    // TODO: add missing attributes
}

fn main() {
    println!("Hello, world!");

    let zusi = Zusi {
        value: vec![
            ZusiValue::Info(
                Info {
                    datei_typ: "result".into(),
                    version: "A.1".into(),
                    min_version: "A.0".into(),
                }
            ),
            ZusiValue::Result(
                ZusiResult {
                    value: vec![
                        ResultValue::FahrtEintrag(FahrtEintrag {
                            fahrt_typ: 0,
                            fahrt_weg: 22.33,
                        }),
                        ResultValue::FahrtEintrag(FahrtEintrag {
                            fahrt_typ: 0,
                            fahrt_weg: 22.43,
                        }),
                    ],
                }
            ),
        ],
    };

    let serialized = se::to_string(&zusi).unwrap();
    println!("{serialized}");
    let deserialized: Zusi = de::from_str(&serialized).unwrap();
    println!("{deserialized:?}");
    assert_eq!(zusi, deserialized);
}
