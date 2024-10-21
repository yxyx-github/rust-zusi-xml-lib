mod utils;

use crate::utils::read_xml_file;
use quick_xml::{de, se};
use std::fs::File;
use std::io::{Read, Write};
use tempfile::tempdir;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::buchfahrplan::{Buchfahrplan, BuchfahrplanValue};
use zusi_xml_lib::xml::zusi::info::Info;
use zusi_xml_lib::xml::zusi::lib::bremsstellung::Bremsstellung;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::{FahrplanZeile, FahrplanZeileValue};
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
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_reduzierungen::{FahrplanVMaxReduzierungen, FahrplanVMaxReduzierungenValue};
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_reduzierungen::ereignis::Ereignis;
use zusi_xml_lib::xml::zusi::buchfahrplan::fahrzeug::Fahrzeug;
use zusi_xml_lib::xml::zusi::lib::datei::Datei;
use zusi_xml_lib::xml::zusi::lib::fahrplan_eintrag::FahrplanEintragsTyp;
use zusi_xml_lib::xml::zusi::lib::utm::UTM;

fn expected_serialized(include_optionals: bool) -> String {
    read_xml_file(format!("./tests/xml/timetable{}.xml", if include_optionals { "" } else {"-without-optionals" }))
}

fn expected_deserialized(include_optionals: bool) -> Zusi {
    Zusi {
        value: vec![
            ZusiValue::Info(
                Info {
                    datei_typ: "result".into(),
                    version: "A.1".into(),
                    min_version: "A.0".into(),
                    value: vec![],
                }
            ),
            ZusiValue::Buchfahrplan(Buchfahrplan {
                gattung: "RB".to_string(),
                nummer: "1234".to_string(),
                zuglauf: "ADorf - BDorf".to_string(),
                baureihe: "442".to_string(),
                masse: 300.0,
                speed_max: 44.4,
                bremshundertstel: 179.0,
                mindest_bremshundertstel: 178.0,
                verkehrstage: "Mo-Fr".to_string(),
                grenzlast: false,
                laenge: 88.4,
                laenge_loks: 0.0,
                wagenzug_laenge: 88.4,
                km_start: 0.0,
                bremsstellung_zug: Bremsstellung::RMg,
                fahrplan_bremsstellung_textvorgabe: "R+Mg".to_string(),
                gnt_spalte: false,
                value: vec![
                    BuchfahrplanValue::DateiFpn(Datei {
                        dateiname: "path/to/fpn.fpn".to_string(),
                        inst: 0,
                        nur_info: true,
                    }),
                    BuchfahrplanValue::DateiTrn(Datei {
                        dateiname: "path/to/trn.trn".to_string(),
                        inst: 0,
                        nur_info: true,
                    }),
                    BuchfahrplanValue::UTM(UTM {
                        utm_we: 1,
                        utm_ns: 2,
                        utm_zone: 3,
                        utm_zone_2: "zone".to_string(),
                    }),
                    BuchfahrplanValue::Fahrzeug(Fahrzeug {
                        fahrzeug_zusatzinfo: "zusatz".to_string(),
                    }),
                    BuchfahrplanValue::FahrplanZeile(FahrplanZeile {
                        fahrplan_regelgleis_gegengleis: 0,
                        fahrplan_laufweg: 3.7,
                        fahrstrasse_strecke: "fstr".to_string(),
                        value: vec![
                            FahrplanZeileValue::FahrplanVMax(FahrplanVMax {
                                v_max_typ: 0,
                                v_max: 38.8,
                            }),
                            FahrplanZeileValue::FahrplanVMaxGNT(FahrplanVMaxGNT {
                                v_max: 38.8,
                            }),
                            FahrplanZeileValue::FahrplanKm(FahrplanKm {
                                km: 172.6,
                                fahrplan_sprung: false,
                                fahrplan_km_neu: 0.0,
                            }),
                            FahrplanZeileValue::FahrplanName(FahrplanName {
                                fahrplan_name_text: "CDorf".to_string(),
                                fahrplan_wichtigkeit: 1,
                                fahrplan_original_text: "orig txt".to_string(),
                            }),
                            FahrplanZeileValue::FahrplanSignalTyp(FahrplanSignalTyp {
                                fahrplan_signal_typ_nummer: 0,
                                fahrplan_hilfs_signal: false,
                            }),
                            FahrplanZeileValue::FahrplanIcon(FahrplanIcon {
                                fahrplan_icon_nummer: 3,
                            }),
                            FahrplanZeileValue::FahrplanTunnel(FahrplanTunnel {
                                fahrplan_name_text: "Dunneler Tunnel".to_string(),
                                fahrplan_tunnel_anfang: true,
                            }),
                            FahrplanZeileValue::FahrplanNameRechts(FahrplanNameRechts {
                                fahrplan_name_text: "E60".to_string(),
                            }),
                            FahrplanZeileValue::FahrplanSaegelinien(FahrplanSaegelinien {
                                fahrplan_anzahl: 40,
                            }),
                            FahrplanZeileValue::FahrplanAnkunft(FahrplanAnkunft {
                                ankunft: if include_optionals { Some(datetime!(2001-01-01 01:01:01)) } else { None },
                                fahrplan_eintrag: FahrplanEintragsTyp::Standard,
                            }),
                            FahrplanZeileValue::FahrplanAbfahrt(FahrplanAbfahrt {
                                abfahrt: if include_optionals { Some(datetime!(2001-01-01 01:03:30)) } else { None },
                            }),
                            FahrplanZeileValue::FahrplanRichtungswechsel(FahrplanRichtungswechsel {}),
                            FahrplanZeileValue::FahrplanVMaxReduzierungen(FahrplanVMaxReduzierungen {
                                value: vec![
                                    FahrplanVMaxReduzierungenValue::Ereignis(Ereignis {
                                        ereignis: 0,
                                        wert: 7.0,
                                        beschreibung: "ctx".to_string(),
                                    })
                                ],
                            }),
                        ],
                    }),
                ],
            }),
        ],
    }
}

#[test]
fn test_serialize_deserialize() {
    let serialized = se::to_string(&expected_deserialized(true)).unwrap();
    assert_eq!(serialized, expected_serialized(true));

    let deserialized: Zusi = de::from_str(&serialized).unwrap();
    assert_eq!(deserialized, expected_deserialized(true));
}

#[test]
fn test_serialize_deserialize_optionals() {
    let serialized = se::to_string(&expected_deserialized(false)).unwrap();
    assert_eq!(serialized, expected_serialized(false));

    let deserialized: Zusi = de::from_str(&serialized).unwrap();
    assert_eq!(deserialized, expected_deserialized(false));
}

#[test]
fn test_from_xml() {
    let zusi = Zusi::from_xml(&expected_serialized(true)).unwrap();
    assert_eq!(zusi, expected_deserialized(true));
}

#[test]
fn test_from_xml_file_by_path() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("xml_input_file.xml");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(expected_serialized(true).as_bytes()).unwrap();

    let zusi: Zusi = Zusi::from_xml_file_by_path(&file_path).unwrap();

    assert_eq!(zusi, expected_deserialized(true));
}

#[test]
fn test_to_xml() {
    let xml = expected_deserialized(true).to_xml().unwrap();
    assert_eq!(xml, expected_serialized(true));
}

#[test]
fn test_to_xml_file_by_path() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("xml_output_file.xml");

    expected_deserialized(true).to_xml_file_by_path(&file_path).unwrap();

    let mut file = File::open(&file_path).unwrap();
    let mut xml = String::new();
    file.read_to_string(&mut xml).unwrap();
    assert_eq!(xml, expected_serialized(true));
}
