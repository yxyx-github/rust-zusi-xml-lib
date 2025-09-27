use std::collections::HashMap;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_abfahrt::FahrplanAbfahrt;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_ankunft::FahrplanAnkunft;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_icon::FahrplanIcon;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_km::FahrplanKm;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_name::FahrplanName;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_name_rechts::FahrplanNameRechts;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_richtungswechsel::FahrplanRichtungswechsel;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_saegelinien::FahrplanSaegelinien;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_signal_typ::FahrplanSignalTyp;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_tunnel::FahrplanTunnel;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max::FahrplanVMax;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_gnt::FahrplanVMaxGNT;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_reduzierungen::FahrplanVMaxReduzierungen;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::FahrplanZeile;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrzeug_zusatz_info::FahrzeugZusatzInfo;
use zusi_xml_lib::xml::zusi::buchfahrplan::Buchfahrplan;
use zusi_xml_lib::xml::zusi::info::{DateiTyp, Info};
use zusi_xml_lib::xml::zusi::lib::bremsstellung::Bremsstellung;
use zusi_xml_lib::xml::zusi::lib::datei::Datei;
use zusi_xml_lib::xml::zusi::lib::ereignis::Ereignis;
use zusi_xml_lib::xml::zusi::lib::fahrplan_eintrag::FahrplanEintragsTyp;
use zusi_xml_lib::xml::zusi::lib::utm::UTM;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};

#[allow(dead_code)]
pub fn all() -> Zusi {
    Zusi {
        info: Info {
            datei_typ: DateiTyp::Buchfahrplan,
            version: "A.1".into(),
            min_version: "A.0".into(),
            autor_eintrag: None,
            datei: None,
            _unknown: HashMap::new(),
        },
        value: ZusiValue::Buchfahrplan(Buchfahrplan {
            gattung: "RB".into(),
            nummer: "2027".into(),
            zuglauf: "ADorf - BDorf".into(),
            baureihe: "442".into(),
            masse: 40.0,
            speed_max: 30.0,
            bremshundertstel: 1.95,
            mindest_bremshundertstel: 1.8,
            verkehrstage: "Mo - Fr".into(),
            grenzlast: true,
            laenge: 70.0,
            laenge_loks: 10.0,
            wagenzug_laenge: 60.0,
            km_start: 3.4,
            bremsstellung_zug: Bremsstellung::RMg,
            fahrplan_bremsstellung_textvorgabe: "R+Mg".into(),
            gnt_spalte: true,
            datei_fpn: Datei {
                dateiname: "to/the/file.fpn".try_into().unwrap(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            },
            datei_trn: Datei {
                dateiname: "to/the/file.trn".try_into().unwrap(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            },
            utm: UTM {
                utm_we: 4,
                utm_ns: 9,
                utm_zone: 7,
                utm_zone_2: "Z".to_string(),
                _unknown: HashMap::new(),
            },
            fahrzeug_info: vec![FahrzeugZusatzInfo {
                fahrzeug_zusatzinfo: "Zusatz".into(),
            }],
            fahrplan_zeilen: vec![
                FahrplanZeile {
                    fahrplan_regelgleis_gegengleis: 2,
                    fahrplan_laufweg: 3.0,
                    fahrstrasse_strecke: "A -> B".into(),
                    fahrplan_v_max: Some(FahrplanVMax {
                        v_max_typ: 2,
                        v_max: 20.0,
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_v_max_gnt: Some(FahrplanVMaxGNT {
                        v_max: 3.7,
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_km: vec![FahrplanKm {
                        km: 3.8,
                        fahrplan_sprung: true,
                        fahrplan_km_neu: 2.7,
                        _unknown: HashMap::new(),
                    }],
                    fahrplan_name: Some(FahrplanName {
                        fahrplan_name_text: "Eintrag".into(),
                        fahrplan_wichtigkeit: 3,
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_signal_typ: Some(FahrplanSignalTyp {
                        fahrplan_signal_typ_nummer: 2,
                        fahrplan_hilfs_signal: true,
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_icon: vec![FahrplanIcon {
                        fahrplan_icon_nummer: 4,
                        _unknown: HashMap::new(),
                    }],
                    fahrplan_tunnel: Some(FahrplanTunnel {
                        fahrplan_name_text: "Großer Tunnel".into(),
                        fahrplan_tunnel_anfang: true,
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_name_rechts: Some(FahrplanNameRechts {
                        fahrplan_name_text: "Rechts".into(),
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_saegelinien: Some(FahrplanSaegelinien {
                        fahrplan_anzahl: 2,
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_ankunft: Some(FahrplanAnkunft {
                        ankunft: datetime!(2000-12-31 12:37:20),
                        fahrplan_eintrag: FahrplanEintragsTyp::Hilfseintrag,
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_abfahrt: Some(FahrplanAbfahrt {
                        abfahrt: datetime!(2000-11-11 11:11:11),
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_richtungswechsel: Some(FahrplanRichtungswechsel {
                        _unknown: HashMap::new(),
                    }),
                    fahrplan_v_max_reduzierungen: Some(FahrplanVMaxReduzierungen {
                        v_max_reduzierungen: vec![
                            Ereignis {
                                ereignis: 58,
                                wert: 20.0,
                                beschreibung: "Beschreibung".into(),
                            },
                        ],
                        _unknown: HashMap::new(),
                    }),
                    _unknown: HashMap::new(),
                },
            ],
            _unknown: HashMap::new(),
        }),
    }
}

#[allow(dead_code)]
pub fn with_defaults() -> Zusi {
    Zusi::builder()
        .info(
            Info::builder()
            .datei_typ(DateiTyp::Buchfahrplan)
            .version("A.1".into())
            .min_version("A.0".into())
            .build()
        )
        .value(ZusiValue::Buchfahrplan(
            Buchfahrplan::builder()
                .datei_fpn(Datei::builder().build())
                .datei_trn(Datei::builder().build())
                .utm(UTM::builder().build())
                .fahrplan_zeilen(vec![
                    FahrplanZeile::builder().fahrplan_name(Some(FahrplanName::builder().fahrplan_name_text("Name".into()).build())).build(),
                    FahrplanZeile::builder().fahrplan_name_rechts(Some(FahrplanNameRechts::builder().fahrplan_name_text("Name Rechts".into()).build())).build(),
                ])
                .build()
        ))
        .build()
}