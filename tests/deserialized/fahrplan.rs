use std::collections::HashMap;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::fahrplan::strecken_modul::StreckenModul;
use zusi_xml_lib::xml::zusi::fahrplan::zug_datei_eintrag::ZugDateiEintrag;
use zusi_xml_lib::xml::zusi::fahrplan::Fahrplan;
use zusi_xml_lib::xml::zusi::info::Info;
use zusi_xml_lib::xml::zusi::lib::datei::Datei;
use zusi_xml_lib::xml::zusi::lib::fahrplan_eintrag::FahrplanEintragsTyp;
use zusi_xml_lib::xml::zusi::lib::phi::Phi;
use zusi_xml_lib::xml::zusi::lib::position::Position;
use zusi_xml_lib::xml::zusi::lib::utm::UTM;
use zusi_xml_lib::xml::zusi::zug::fahrplan_eintrag::FahrplanEintrag;
use zusi_xml_lib::xml::zusi::zug::fahrzeug_varianten::FahrzeugVarianten;
use zusi_xml_lib::xml::zusi::zug::Zug;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};

#[allow(dead_code)]
pub fn all() -> Zusi {
    Zusi {
        info: Info {
            datei_typ: "result".into(),
            version: "A.1".into(),
            min_version: "A.0".into(),
            autor_eintrag: None,
            datei: None,
            _unknown: HashMap::new(),
        },
        value: ZusiValue::Fahrplan(Fahrplan {
            anfangs_zeit: datetime!(2023-04-07 03:23:07),
            zeitmodus: 1,
            chaos_vorschlagen: true,
            chaos_vorschlag: 0.03,
            trn_dateien: true,
            befehls_konfiguration: Datei {
                dateiname: "to/the/file.ext".into(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            },
            la_pdf: Datei {
                dateiname: "to/the/file.ext".into(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            },
            streckenbuch_pdf: Datei {
                dateiname: "to/the/file.ext".into(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            },
            ersatzfahrplaene_pdf: Datei {
                dateiname: "to/the/file.ext".into(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            },
            begruessungs_datei: Datei {
                dateiname: "to/the/file.ext".into(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            },
            zug_eintraege: vec![
                Zug::builder()
                    .fahrplan_datei(Datei::builder().dateiname("path/to/file".into()).build())
                    .fahrplan_eintraege(vec![
                        FahrplanEintrag::builder().build(),
                        FahrplanEintrag::builder().fahrplan_eintrag(FahrplanEintragsTyp::Hilfseintrag).build(),
                    ])
                    .fahrzeug_varianten(FahrzeugVarianten::builder().build())
                    .build(),
                Zug::builder()
                    .fahrplan_datei(Datei::builder().dateiname("path/to/file".into()).build())
                    .fahrplan_eintraege(vec![
                        FahrplanEintrag::builder().build(),
                        FahrplanEintrag::builder().fahrplan_eintrag(FahrplanEintragsTyp::Standard).build(),
                    ])
                    .fahrzeug_varianten(FahrzeugVarianten::builder().build())
                    .build(),
            ],
            zug_dateien: vec![
                ZugDateiEintrag {
                    datei: Datei {
                        dateiname: "to/the/zug1.trn".into(),
                        inst: 0,
                        nur_info: false,
                        _unknown: HashMap::new(),
                    },
                    _unknown: HashMap::new(),
                },
                ZugDateiEintrag {
                    datei: Datei {
                        dateiname: "to/the/zug2.trn".into(),
                        inst: 0,
                        nur_info: false,
                        _unknown: HashMap::new(),
                    },
                    _unknown: HashMap::new(),
                },
            ],
            strecken_module: vec![
                StreckenModul {
                    datei: Datei {
                        dateiname: "to/the/mod1.st3".into(),
                        inst: 0,
                        nur_info: false,
                        _unknown: HashMap::new(),
                    },
                    position: Position {
                        x: 0.1,
                        y: 0.2,
                        z: 0.3,
                        _unknown: HashMap::new(),
                    },
                    phi: Phi {
                        x: 0.4,
                        y: 0.5,
                        z: 0.6,
                        _unknown: HashMap::new(),
                    },
                    _unknown: HashMap::new(),
                },
            ],
            utm: UTM {
                utm_we: 8,
                utm_ns: 7,
                utm_zone: 33,
                utm_zone_2: "Zone2".into(),
                _unknown: HashMap::new(),
            },
            _unknown: HashMap::new(),
        }),
    }
}

#[allow(dead_code)]
pub fn with_defaults() -> Zusi {
    Zusi::builder()
        .info(
            Info::builder()
            .datei_typ("result".into())
            .version("A.1".into())
            .min_version("A.0".into())
            .build()
        )
        .value(ZusiValue::Fahrplan(
            Fahrplan::builder()
                .anfangs_zeit(datetime!(2022-09-12 08:32:29))
                .befehls_konfiguration(Datei::builder().dateiname("path/to/file".into()).build())
                .la_pdf(Datei::builder().build())
                .streckenbuch_pdf(Datei::builder().build())
                .ersatzfahrplaene_pdf(Datei::builder().build())
                .begruessungs_datei(Datei::builder().build())
                .zug_dateien(vec![
                    ZugDateiEintrag::builder().datei(Datei::builder().dateiname("path/to/first.trn".into()).build()).build(),
                    ZugDateiEintrag::builder().datei(Datei::builder().dateiname("path/to/second.trn".into()).build()).build(),
                ])
                .strecken_module(vec![
                    StreckenModul::builder().datei(Datei::builder().dateiname("path/to/strmod1.st3".into()).build()).position(Position::builder().build()).phi(Phi::builder().build()).build(),
                    StreckenModul::builder().datei(Datei::builder().dateiname("path/to/strmod2.st3".into()).build()).position(Position::builder().build()).phi(Phi::builder().build()).build(),
                ])
                .utm(UTM::builder().build())
                .build(),
        ))
        .build()
}