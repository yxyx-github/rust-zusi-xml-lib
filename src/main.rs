use quick_xml::se;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Zusi {
    #[serde(rename = "$value")]
    value: Vec<ZusiValue>,
}

#[derive(Serialize, Deserialize, Debug)]
enum ZusiValue {
    Info(Info),
    Result(ZusiResult),
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    datei_typ: String,
    version: String,
    min_version: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ZusiResult {
    #[serde(rename = "$value")]
    value: Vec<ResultValue>,
}

#[derive(Serialize, Deserialize, Debug)]
enum ResultValue {
    FahrtEintrag(FahrtEintrag),
}

#[derive(Serialize, Deserialize, Debug)]
struct FahrtEintrag {
    fahrt_typ: u32, // TODO: check correct type
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
}
