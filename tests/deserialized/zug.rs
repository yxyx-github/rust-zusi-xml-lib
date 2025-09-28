use std::collections::HashMap;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::info::{DateiTyp, Info};
use zusi_xml_lib::xml::zusi::lib::bremsstellung::Bremsstellung;
use zusi_xml_lib::xml::zusi::lib::datei::Datei;
use zusi_xml_lib::xml::zusi::lib::fahrplan_eintrag::FahrplanEintragsTyp;
use zusi_xml_lib::xml::zusi::zug::aufgleis_referenz::AufgleisReferenz;
use zusi_xml_lib::xml::zusi::zug::fahrplan_eintrag::FahrplanEintrag;
use zusi_xml_lib::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_pzb80::ZugdatenPZB80;
use zusi_xml_lib::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::zugdaten_zbs::ZugdatenZBS;
use zusi_xml_lib::xml::zusi::zug::fahrzeug_varianten::fahrzeug_info::{DoppeltraktionsModus, FahrzeugInfo};
use zusi_xml_lib::xml::zusi::zug::fahrzeug_varianten::FahrzeugVarianten;
use zusi_xml_lib::xml::zusi::zug::lod_zug::LodZug;
use zusi_xml_lib::xml::zusi::zug::standort_modus::StandortModus;
use zusi_xml_lib::xml::zusi::zug::zug_typ::ZugTyp;
use zusi_xml_lib::xml::zusi::zug::zugsicherung_start_modus::ZugsicherungStartModus;
use zusi_xml_lib::xml::zusi::zug::Zug;
use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::zug::fahrplan_eintrag::fahrzeug_verband_aktion::FahrzeugVerbandAktion;

#[allow(dead_code)]
pub fn all() -> Zusi {
    Zusi {
        info: Info {
            datei_typ: DateiTyp::Zug,
            version: "A.1".into(),
            min_version: "A.0".into(),
            autor_eintrag: None,
            datei: None,
            _unknown: HashMap::new(),
        },
        value: ZusiValue::Zug(Zug {
            gattung: "RE".into(),
            nummer: "33333".into(),
            zuglauf: "AStadt - BStadt".into(),
            baureihe_angabe: "442".into(),
            prioritaet: 1,
            standort_modus: StandortModus::Fahrstrassenanfang,
            start_vorschubweg: 10.0,
            bremsstellung_zug: Default::default(),
            fahrplan_bremsstellung_textvorgabe: "R+Mg".into(),
            energie_vorgabe: 20.0,
            speed_anfang: 10.0,
            mindest_bremshundertstel: 1.9,
            fahrplan_masse: 300.0,
            fahrplan_zug_laenge: 100.0,
            verkehrstage: "Mo-Fr".into(),
            grenzlast: true,
            speed_zug_niedriger: 30.0,
            autopilot_beschleunigung: 0.7,
            keine_vorplan_korrektur: true,
            dekozug: true,
            lod_zug: LodZug::Wichtige,
            reisenden_dichte: 0.4,
            fahrplan_gruppe: "1a".into(),
            rekursionstiefe: 3,
            zugsicherung_startmodus: ZugsicherungStartModus::ZugdatenMuessenEingegebenWerden,
            cold_movement: true,
            fahrstrassen_name: "A -> B".into(),
            aufgleisen_register_pruefen: true,
            zug_typ: ZugTyp::Reisezug,
            ueberschrift: "keine".into(),
            odt_datei_absolut: "some/where/located".into(),
            buchfahrplan_einfach: true,
            buchfahrplan_dll: "my.dll".into(),
            tuer_system_bezeichner: "TAV".into(),
            fahrplan_datei: Datei {
                dateiname: "this/is/it.fpn".try_into().unwrap(),
                inst: 1,
                nur_info: true,
                _unknown: HashMap::new(),
            },
            buchfahrplan_roh_datei: Some(Datei {
                dateiname: "to/the/file.ext".try_into().unwrap(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            }),
            buchfahrplan_bmp_datei: Some(Datei {
                dateiname: "to/the/file.ext".try_into().unwrap(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            }),
            anfangsbefehl: Some(Datei {
                dateiname: "to/the/file.ext".try_into().unwrap(),
                inst: 0,
                nur_info: false,
                _unknown: HashMap::new(),
            }),
            aufgleis_referenz: Some(AufgleisReferenz {
                referenz_nummer: 3,
                datei: Datei {
                    dateiname: "to/the/file.ext".try_into().unwrap(),
                    inst: 0,
                    nur_info: false,
                    _unknown: HashMap::new(),
                },
                _unknown: HashMap::new(),
            }),
            fahrplan_eintraege: vec![
                FahrplanEintrag {
                    ankunft: Some(datetime!(2020-10-01 03:43:21)),
                    abfahrt: Some(datetime!(2020-10-01 03:46:21)),
                    signal_vorlauf: 180.0,
                    signal_bleibt_auf_halt: true,
                    betriebsstelle: "Blersdorf".into(),
                    fahrplan_eintrag: FahrplanEintragsTyp::Betriebshalt,
                    ersatz_signal_zeile_plus_1: 1,
                    fahrzeug_verband_aktion: FahrzeugVerbandAktion::Fueherstandswechsel,
                    fahrzeug_verband_aktion_wende_signal: true,
                    kuerzung_loeschen: true,
                    fahrzeug_verband_aktion_wende_signal_abstand: 30.0,
                    fahrplan_signal_eintraege: vec![], // TODO: add
                    fahrplan_fahrstrassen_eintraege: vec![], // TODO: add
                    fahrplan_vorgang_ereignisse: None,
                    fahrplan_vorgang_fahrstrasse: None,
                    ereignisse: vec![], // TODO: add
                    _unknown: HashMap::new(),
                },
            ],
            fahrzeug_varianten: FahrzeugVarianten {
                bezeichnung: "Triebzug".into(),
                per_zufall_uebernehmen: true,
                gattung: "Gattung".into(),
                zug_typ_vorgeben: true,
                zug_typ: ZugTyp::Reisezug,
                mindest_bremshundertstel: 1.2,
                fahrplan_masse: 10.0,
                fahrplan_zug_laenge: 200.0,
                bremsstellung_zug: Bremsstellung::RMg,
                fahrplan_bremsstellung_textvorgabe: "R+Mg".into(),
                speed_zug_niedriger: 20.0,
                zufalls_wert: 0.3,
                fahrzeug_position: 2,
                fahrzeug_infos: vec![
                    FahrzeugInfo {
                        id_haupt: 2,
                        id_neben: 3,
                        start_antrieb_index: 2,
                        eigene_bremsstellung: true,
                        bremsstellung_fahrzeug: Bremsstellung::G,
                        eigene_zugart: true,
                        tuer_ignorieren: true,
                        doppeltraktions_modus: DoppeltraktionsModus::Mehrfachtraktion,
                        stromabnehmer_schaltung: 1,
                        gedreht: true,
                        feststellbremse: true,
                        lastwechsel_beladen: 1,
                        ueberladung: 20,
                        bremsstoerung: 1,
                        mg_bremse_senken: 1,
                        mg_bremse_strom: 20,
                        fahrzeug_zusatzinfo: "Info".into(),
                        nvr_nummer: "NVR".into(),
                        datei: Datei {
                            dateiname: "to/the/file.ext".try_into().unwrap(),
                            inst: 0,
                            nur_info: false,
                            _unknown: HashMap::new(),
                        },

                        zugdaten_etcs: None,
                        zugdaten_zbs: Some(ZugdatenZBS {
                            bremshundertstel: 90,
                            zug_laenge: 200,
                            v_max_zug: 80,
                            tf_nummer: "123".into(),
                            zug_nummer: "456".into(),
                            zbs_betriebszustand: 1,
                            start_system: "A".into(),
                            zugsicherung_hauptschalter: 1,
                            lufthahn: 1,
                            zbs_stoerschalter: 1,
                            _unknown: HashMap::new(),
                        }),
                        // TODO: add missing variants for zugdaten
                        zugdaten_fahrsperre: None,
                        zugdaten_lzb80: None,
                        zugdaten_indusi_analog: None,
                        zugdaten_indusi_rechner: None,
                        zugdaten_pzb80: Some(ZugdatenPZB80 {
                            indusi_zugart: 2,
                            _unknown: HashMap::new(),
                        }),
                        _unknown: HashMap::new(),
                    },
                    FahrzeugInfo {
                        id_haupt: 1,
                        id_neben: 4,
                        start_antrieb_index: 2,
                        eigene_bremsstellung: true,
                        bremsstellung_fahrzeug: Bremsstellung::G,
                        eigene_zugart: true,
                        tuer_ignorieren: true,
                        doppeltraktions_modus: DoppeltraktionsModus::Mehrfachtraktion,
                        stromabnehmer_schaltung: 1,
                        gedreht: true,
                        feststellbremse: true,
                        lastwechsel_beladen: 1,
                        ueberladung: 20,
                        bremsstoerung: 1,
                        mg_bremse_senken: 1,
                        mg_bremse_strom: 20,
                        fahrzeug_zusatzinfo: "Info".into(),
                        nvr_nummer: "NVR".into(),
                        datei: Datei {
                            dateiname: "to/the/file.ext".try_into().unwrap(),
                            inst: 0,
                            nur_info: false,
                            _unknown: HashMap::new(),
                        },
                        zugdaten_etcs: None,
                        zugdaten_zbs: None,
                        zugdaten_fahrsperre: None,
                        zugdaten_lzb80: None,
                        zugdaten_indusi_analog: None,
                        zugdaten_indusi_rechner: None,
                        zugdaten_pzb80: None,
                        _unknown: HashMap::new(),
                    },
                ],
                fahrzeug_varianten: vec![
                    FahrzeugVarianten {
                        bezeichnung: "Triebwagen".into(),
                        per_zufall_uebernehmen: true,
                        gattung: "Die Gattung".into(),
                        zug_typ_vorgeben: false,
                        zug_typ: ZugTyp::Gueterzug,
                        mindest_bremshundertstel: 1.2,
                        fahrplan_masse: 10.0,
                        fahrplan_zug_laenge: 300.0,
                        bremsstellung_zug: Bremsstellung::RMg,
                        fahrplan_bremsstellung_textvorgabe: "R+Mg".into(),
                        speed_zug_niedriger: 20.0,
                        zufalls_wert: 0.3,
                        fahrzeug_position: 2,
                        fahrzeug_infos: vec![],
                        fahrzeug_varianten: vec![],
                        _unknown: HashMap::new(),
                    },
                ],
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
            .datei_typ(DateiTyp::Zug)
            .version("A.1".into())
            .min_version("A.0".into())
            .build()
        )
        .value(ZusiValue::Zug(
            Zug::builder()
                .fahrplan_datei(Datei::builder().dateiname("path/to/file".try_into().unwrap()).build())
                .fahrplan_eintraege(vec![
                    FahrplanEintrag::builder().build(),
                    FahrplanEintrag::builder().fahrplan_eintrag(FahrplanEintragsTyp::Hilfseintrag).build(),
                ])
                .fahrzeug_varianten(FahrzeugVarianten::builder().build())
                .build()
        ))
        .build()
}