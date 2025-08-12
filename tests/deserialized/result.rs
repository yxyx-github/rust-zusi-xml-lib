use std::collections::HashMap;
use time::macros::datetime;
use zusi_xml_lib::delphi_timestamp::DelphiTimestamp;
use zusi_xml_lib::xml::zusi::info::Info;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};
use zusi_xml_lib::xml::zusi::result::ZusiResult;

pub fn all() -> Zusi {
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
            anfang_datum: Some(datetime!(2019-01-01 23:16)),
            datum: datetime!(2019-01-01 23:14),
            verbrauch: 3.7,
            bemerkung: "".to_string(),
            schummel: false,
            schwierigkeitsgrad: 1,
            energie_vorgabe: 3000.0,
            fahrt_eintraege: vec![
                FahrtEintrag {
                    fahrt_typ: FahrtTyp::Standard,
                    fahrt_weg: 22.33,
                    fahrt_zeit: datetime!(2019-01-01 23:18),
                    fahrt_speed: 20.0,
                    fahrt_speed_strecke: 30.0,
                    fahrt_speed_signal: 20.0,
                    fahrt_speed_zugsicherung: 25.0,
                    fahrt_autopilot: false,
                    fahrt_km: 77.2,
                    fahrt_hl_druck: 4.2,
                    fahrt_parameter: 1,
                    fahrt_text: "ADorf".into(),
                    fahrt_fpl_ank: Some(DelphiTimestamp::from(43410.3125)),
                    fahrt_fpl_abf: Some(DelphiTimestamp::from(43410.316406)),
                    fahrt_fb_schalter: 2,
                },
                FahrtEintrag {
                    fahrt_typ: FahrtTyp::Standard,
                    fahrt_weg: 22.43,
                    fahrt_zeit: datetime!(2019-01-02 1:07),
                    fahrt_speed: 10.0,
                    fahrt_speed_strecke: 40.0,
                    fahrt_speed_signal: 13.0,
                    fahrt_speed_zugsicherung: 15.0,
                    fahrt_autopilot: false,
                    fahrt_km: 32.0,
                    fahrt_hl_druck: 4.0,
                    fahrt_parameter: 2,
                    fahrt_text: "BDorf".into(),
                    fahrt_fpl_ank: Some(DelphiTimestamp::from(43410.347656)),
                    fahrt_fpl_abf: Some(DelphiTimestamp::from(43410.351563)),
                    fahrt_fb_schalter: 3,
                },
            ],
            unknown: HashMap::new(),
        }),
    }
}

pub fn with_defaults() -> Zusi {
    Zusi::builder()
        .info(
            Info::builder()
            .datei_typ("result".into())
            .version("A.1".into())
            .min_version("A.0".into())
            .build()
        )
        .value(ZusiValue::Result(
            ZusiResult::builder()
                .datum(datetime!(2019-01-01 23:14))
                .fahrt_eintraege(vec![
                    FahrtEintrag::builder()
                        .fahrt_zeit(datetime!(2019-01-01 23:18))
                        .build()
                ])
                .build()
        ))
        .build()
}