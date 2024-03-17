use quick_xml::{de, se};
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::info::Info;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};

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
