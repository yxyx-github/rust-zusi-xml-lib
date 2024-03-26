use quick_xml::{de, se};
pub use quick_xml::DeError;
use serde::{Deserialize, Serialize};

use crate::xml::zusi::info::Info;
use crate::xml::zusi::result::ZusiResult;

pub mod info;
pub mod result;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Zusi {
    #[serde(rename = "$value")]
    pub value: Vec<ZusiValue>,
}

impl Zusi {
    pub fn from_xml(xml: &str) -> Result<Zusi, DeError> {
        de::from_str(xml)
    }

    pub fn to_xml(&self) -> Result<String, DeError> {
        se::to_string(self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ZusiValue {
    Info(Info),

    #[serde(rename = "result")]
    Result(ZusiResult),
}

#[cfg(test)]
mod tests {
    use quick_xml::{de, se};
    use time::macros::datetime;

    use crate::xml::zusi::{Zusi, ZusiValue};
    use crate::xml::zusi::info::Info;
    use crate::xml::zusi::result::{ResultValue, ZusiResult};
    use crate::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};

    const EXPECTED_XML: &str = "<Zusi><Info DateiTyp=\"result\" Version=\"A.1\" MinVersion=\"A.0\"/><result Zugnummer=\"12345\" TfNummer=\"67890\" Datum=\"2019-01-01 23:14:00\" Verbrauch=\"0\" Bemerkung=\"\" Schummel=\"false\" Schwierigkeitsgrad=\"0\" EnergieVorgabe=\"0\"><FahrtEintrag FahrtTyp=\"0\" FahrtWeg=\"22.33\" FahrtZeit=\"2019-01-01 23:18:00\" Fahrtsp=\"0\" FahrtspStrecke=\"0\" FahrtspSignal=\"0\" FahrtspZugsicherung=\"0\" FahrtAutopilot=\"false\" Fahrtkm=\"0\" FahrtHLDruck=\"0\" FahrtParameter=\"0\" FahrtFplAnk=\"2019-01-01 23:16:00\" FahrtFplAbf=\"2019-01-01 23:17:00\" FahrtFBSchalter=\"0\"/><FahrtEintrag FahrtTyp=\"0\" FahrtWeg=\"22.43\" FahrtZeit=\"2019-01-02 01:07:00\" Fahrtsp=\"0\" FahrtspStrecke=\"0\" FahrtspSignal=\"0\" FahrtspZugsicherung=\"0\" FahrtAutopilot=\"false\" Fahrtkm=\"0\" FahrtHLDruck=\"0\" FahrtParameter=\"0\" FahrtFplAnk=\"2019-01-02 01:01:00\" FahrtFplAbf=\"2019-01-03 08:46:00\" FahrtFBSchalter=\"0\"/></result></Zusi>";
    const EXPECTED_XML_WITHOUT_OPTIONALS: &str = "<Zusi><Info DateiTyp=\"result\" Version=\"A.1\" MinVersion=\"A.0\"/><result Zugnummer=\"12345\" TfNummer=\"67890\" Datum=\"2019-01-01 23:14:00\" Verbrauch=\"0\" Bemerkung=\"\" Schummel=\"false\" Schwierigkeitsgrad=\"0\" EnergieVorgabe=\"0\"><FahrtEintrag FahrtTyp=\"0\" FahrtWeg=\"22.33\" FahrtZeit=\"2019-01-01 23:18:00\" Fahrtsp=\"0\" FahrtspStrecke=\"0\" FahrtspSignal=\"0\" FahrtspZugsicherung=\"0\" FahrtAutopilot=\"false\" Fahrtkm=\"0\" FahrtHLDruck=\"0\" FahrtParameter=\"0\" FahrtFBSchalter=\"0\"/><FahrtEintrag FahrtTyp=\"0\" FahrtWeg=\"22.43\" FahrtZeit=\"2019-01-02 01:07:00\" Fahrtsp=\"0\" FahrtspStrecke=\"0\" FahrtspSignal=\"0\" FahrtspZugsicherung=\"0\" FahrtAutopilot=\"false\" Fahrtkm=\"0\" FahrtHLDruck=\"0\" FahrtParameter=\"0\" FahrtFBSchalter=\"0\"/></result></Zusi>";

    fn expected_zusi(include_optionals: bool) -> Zusi {
        Zusi {
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
                        datum: datetime!(2019-01-01 23:14),
                        verbrauch: 0.0,
                        bemerkung: "".to_string(),
                        schummel: false,
                        schwierigkeitsgrad: 0,
                        energie_vorgabe: 0.0,
                        value: vec![
                            ResultValue::FahrtEintrag(FahrtEintrag {
                                fahrt_typ: FahrtTyp::Standard,
                                fahrt_weg: 22.33,
                                fahrt_zeit: datetime!(2019-01-01 23:18),
                                fahrt_speed: 0.0,
                                fahrt_speed_strecke: 0.0,
                                fahrt_speed_signal: 0.0,
                                fahrt_speed_zugsicherung: 0.0,
                                fahrt_autopilot: false,
                                fahrt_km: 0.0,
                                fahrt_hl_druck: 0.0,
                                fahrt_parameter: 0,
                                fahrt_fpl_ank: if include_optionals { Some(datetime!(2019-01-01 23:16)) } else { None },
                                fahrt_fpl_abf: if include_optionals { Some(datetime!(2019-01-01 23:17)) } else { None },
                                fahrt_fb_schalter: 0,
                            }),
                            ResultValue::FahrtEintrag(FahrtEintrag {
                                fahrt_typ: FahrtTyp::Standard,
                                fahrt_weg: 22.43,
                                fahrt_zeit: datetime!(2019-01-02 1:07),
                                fahrt_speed: 0.0,
                                fahrt_speed_strecke: 0.0,
                                fahrt_speed_signal: 0.0,
                                fahrt_speed_zugsicherung: 0.0,
                                fahrt_autopilot: false,
                                fahrt_km: 0.0,
                                fahrt_hl_druck: 0.0,
                                fahrt_parameter: 0,
                                fahrt_fpl_ank: if include_optionals { Some(datetime!(2019-01-02 1:01)) } else { None },
                                fahrt_fpl_abf: if include_optionals { Some(datetime!(2019-01-03 8:46)) } else { None },
                                fahrt_fb_schalter: 0,
                            }),
                        ],
                    }
                ),
            ],
        }
    }

    #[test]
    fn test_serialize_deserialize() {
        let serialized = se::to_string(&expected_zusi(true)).unwrap();
        assert_eq!(EXPECTED_XML, serialized);

        let deserialized: Zusi = de::from_str(&serialized).unwrap();
        assert_eq!(expected_zusi(true), deserialized);
    }

    #[test]
    fn test_serialize_deserialize_optionals() {
        let serialized = se::to_string(&expected_zusi(false)).unwrap();
        assert_eq!(EXPECTED_XML_WITHOUT_OPTIONALS, serialized);

        let deserialized: Zusi = de::from_str(&serialized).unwrap();
        assert_eq!(expected_zusi(false), deserialized);
    }

    #[test]
    fn test_from_xml() {
        let zusi: Zusi = de::from_str(EXPECTED_XML).unwrap();
        assert_eq!(expected_zusi(true), zusi);
    }

    #[test]
    fn test_to_xml() {
        let xml = se::to_string(&expected_zusi(true)).unwrap();
        assert_eq!(xml, EXPECTED_XML);
    }
}