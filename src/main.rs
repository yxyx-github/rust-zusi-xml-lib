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
    #[serde(rename = "@Zugnummer")]
    #[serde(default = "")]
    zugnummer: String,

    #[serde(rename = "@TfNummer")]
    #[serde(default = "")]
    tf_nummer: String,

    #[serde(rename = "@Datum")]
    datum: f32, // TODO: check correct type

    #[serde(rename = "@Verbrauch")]
    #[serde(default = 0)]
    verbrauch: f32, // in Joule

    #[serde(rename = "@Bemerkung")]
    #[serde(default = "")]
    bemerkung: String,

    #[serde(rename = "@Schummel")]
    #[serde(default = false)]
    schummel: bool,

    #[serde(rename = "@Schwierigkeitsgrad")]
    #[serde(default = 0)]
    schwierigkeitsgrad: u32,

    #[serde(rename = "@EnergieVorgabe")]
    #[serde(default = 0)]
    energie_vorgabe: f32,

    #[serde(rename = "$value")]
    value: Vec<ResultValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum ResultValue {
    FahrtEintrag(FahrtEintrag),
    // TODO: add FahrtEventEintrag?
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct FahrtEintrag {
    #[serde(rename = "@FahrtTyp")]
    #[serde(default = 0)]
    fahrt_typ: u32,

    #[serde(rename = "@FahrtWeg")]
    #[serde(default = 0)]
    fahrt_weg: f32,

    #[serde(rename = "@FahrtZeit")]
    fahrt_zeit: f32, // TODO: check correct type

    #[serde(rename = "@Fahrtsp")]
    #[serde(default = 0)]
    fahrt_speed: f32,

    #[serde(rename = "@FahrtspStrecke")]
    #[serde(default = 0)]
    fahrt_speed_strecke: f32,

    #[serde(rename = "@FahrtspSignal")]
    #[serde(default = 0)]
    fahrt_speed_signal: f32,

    #[serde(rename = "@FahrtspZugsicherung")]
    #[serde(default = 0)]
    fahrt_speed_zugsicherung: f32,

    #[serde(rename = "@FahrtAutopilot")]
    #[serde(default = false)]
    fahrt_autopilot: bool,

    #[serde(rename = "@Fahrtkm")]
    #[serde(default = 0)]
    fahrt_km: f32,

    #[serde(rename = "@FahrtHLDruck")]
    #[serde(default = 0)]
    fahrt_hl_druck: f32,

    #[serde(rename = "@FahrtParameter")]
    #[serde(default = 0)]
    fahrt_parameter: u32,

    #[serde(rename = "@FahrtFplAnk")]
    fahrt_fpl_ank: f32, // TODO: check correct type

    #[serde(rename = "@FahrtFplAbf")]
    fahrt_fpl_abf: f32, // TODO: check correct type

    #[serde(rename = "@FahrtFBSchalter")]
    #[serde(default = 0)]
    fahrt_fb_schalter: u32,
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
