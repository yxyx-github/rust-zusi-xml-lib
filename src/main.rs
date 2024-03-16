use quick_xml::{de, se};
use serde::{Deserialize, Serialize};
use time::macros::datetime;
use time::PrimitiveDateTime;

mod date_time_format {
    use serde::{Deserialize, Deserializer, ser, Serializer};
    use time::{format_description, PrimitiveDateTime};

    const FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

    pub fn serialize<S>(pdt: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let format = format_description::parse(FORMAT).map_err(ser::Error::custom)?;
        let formatted = pdt.format(&format).map_err(ser::Error::custom)?;
        serializer.serialize_str(&formatted).map_err(ser::Error::custom)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error> where D: Deserializer<'de> {
        let format = format_description::parse(FORMAT).map_err(serde::de::Error::custom)?;
        let s = String::deserialize(deserializer)?;
        PrimitiveDateTime::parse(&s, &format).map_err(serde::de::Error::custom)
    }
}

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
    #[serde(default)]
    zugnummer: String,

    #[serde(rename = "@TfNummer")]
    #[serde(default)]
    tf_nummer: String,

    #[serde(rename = "@Datum")]
    #[serde(with = "date_time_format")]
    datum: PrimitiveDateTime,

    #[serde(rename = "@Verbrauch")]
    #[serde(default)]
    verbrauch: f32, // in Joule

    #[serde(rename = "@Bemerkung")]
    #[serde(default)]
    bemerkung: String,

    #[serde(rename = "@Schummel")]
    #[serde(default)]
    schummel: bool,

    #[serde(rename = "@Schwierigkeitsgrad")]
    #[serde(default)]
    schwierigkeitsgrad: u32,

    #[serde(rename = "@EnergieVorgabe")]
    #[serde(default)]
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
enum FahrtTyp {
    #[serde(rename = "0")]
    Standard = 0,

    #[serde(rename = "1")]
    Erzwingen = 1,

    #[serde(rename = "2")]
    Planhalt = 2,

    #[serde(rename = "3")]
    FahrfehlerSchwer = 3,

    #[serde(rename = "4")]
    FahrfehlerLeicht = 4,

    #[serde(rename = "5")]
    Streckeninfo = 5,

    #[serde(rename = "6")]
    SichtAnfg = 6,

    #[serde(rename = "7")]
    SichtEnde = 7,

    #[serde(rename = "8")]
    Bedienung = 8,
}

impl Default for FahrtTyp {
    fn default() -> Self {
        FahrtTyp::Standard
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct FahrtEintrag {
    #[serde(rename = "@FahrtTyp")]
    #[serde(default)]
    fahrt_typ: FahrtTyp,

    #[serde(rename = "@FahrtWeg")]
    #[serde(default)]
    fahrt_weg: f32,

    #[serde(rename = "@FahrtZeit")]
    #[serde(with = "date_time_format")]
    fahrt_zeit: PrimitiveDateTime,

    #[serde(rename = "@Fahrtsp")]
    #[serde(default)]
    fahrt_speed: f32,

    #[serde(rename = "@FahrtspStrecke")]
    #[serde(default)]
    fahrt_speed_strecke: f32,

    #[serde(rename = "@FahrtspSignal")]
    #[serde(default)]
    fahrt_speed_signal: f32,

    #[serde(rename = "@FahrtspZugsicherung")]
    #[serde(default)]
    fahrt_speed_zugsicherung: f32,

    #[serde(rename = "@FahrtAutopilot")]
    #[serde(default)]
    fahrt_autopilot: bool,

    #[serde(rename = "@Fahrtkm")]
    #[serde(default)]
    fahrt_km: f32,

    #[serde(rename = "@FahrtHLDruck")]
    #[serde(default)]
    fahrt_hl_druck: f32,

    #[serde(rename = "@FahrtParameter")]
    #[serde(default)]
    fahrt_parameter: u32,

    #[serde(rename = "@FahrtFplAnk")]
    #[serde(with = "date_time_format")]
    fahrt_fpl_ank: PrimitiveDateTime,

    #[serde(rename = "@FahrtFplAbf")]
    #[serde(with = "date_time_format")]
    fahrt_fpl_abf: PrimitiveDateTime,

    #[serde(rename = "@FahrtFBSchalter")]
    #[serde(default)]
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
                    zugnummer: "12345".into(),
                    tf_nummer: "67890".into(),
                    datum: datetime!(2019-01-01 0:00),
                    verbrauch: 0.0,
                    bemerkung: "".to_string(),
                    schummel: false,
                    schwierigkeitsgrad: 0,
                    energie_vorgabe: 0.0,
                    value: vec![
                        ResultValue::FahrtEintrag(FahrtEintrag {
                            fahrt_typ: FahrtTyp::Standard,
                            fahrt_weg: 22.33,
                            fahrt_zeit: datetime!(2019-01-01 0:00),
                            fahrt_speed: 0.0,
                            fahrt_speed_strecke: 0.0,
                            fahrt_speed_signal: 0.0,
                            fahrt_speed_zugsicherung: 0.0,
                            fahrt_autopilot: false,
                            fahrt_km: 0.0,
                            fahrt_hl_druck: 0.0,
                            fahrt_parameter: 0,
                            fahrt_fpl_ank: datetime!(2019-01-01 0:00),
                            fahrt_fpl_abf: datetime!(2019-01-01 0:00),
                            fahrt_fb_schalter: 0,
                        }),
                        ResultValue::FahrtEintrag(FahrtEintrag {
                            fahrt_typ: FahrtTyp::Standard,
                            fahrt_weg: 22.43,
                            fahrt_zeit: datetime!(2019-01-01 0:00),
                            fahrt_speed: 0.0,
                            fahrt_speed_strecke: 0.0,
                            fahrt_speed_signal: 0.0,
                            fahrt_speed_zugsicherung: 0.0,
                            fahrt_autopilot: false,
                            fahrt_km: 0.0,
                            fahrt_hl_druck: 0.0,
                            fahrt_parameter: 0,
                            fahrt_fpl_ank: datetime!(2019-01-01 0:00),
                            fahrt_fpl_abf: datetime!(2019-01-01 0:00),
                            fahrt_fb_schalter: 0,
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
