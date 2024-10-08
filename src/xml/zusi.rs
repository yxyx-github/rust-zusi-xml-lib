use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

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
    pub fn from_xml(xml: &str) -> Result<Self, DeError> {
        de::from_str(xml)
    }

    pub fn from_xml_file_by_path<P: AsRef<Path>>(path: P) -> Result<Self, ZusiXMLFileError> {
        let mut file = File::open(path)
            .map_err(|err| ZusiXMLFileError::IOError(err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| ZusiXMLFileError::IOError(err))?;
        Ok(
            Zusi::from_xml(&contents)
                .map_err(|err| ZusiXMLFileError::DeError(err))?
        )
    }

    pub fn to_xml(&self) -> Result<String, DeError> {
        se::to_string(self)
    }

    pub fn to_xml_file_by_path<P: AsRef<Path>>(&self, path: P) -> Result<(), ZusiXMLFileError> {
        let xml = self.to_xml()
            .map_err(|err| ZusiXMLFileError::DeError(err))?;
        let mut file = File::create(path)
            .map_err(|err| ZusiXMLFileError::IOError(err))?;
        file.write_all(xml.as_bytes())
            .map_err(|err| ZusiXMLFileError::IOError(err))?;
        Ok(())
    }
}

impl AsRef<Zusi> for Zusi {
    fn as_ref(&self) -> &Zusi {
        &self
    }
}

#[derive(Debug)]
pub enum ZusiXMLFileError {
    IOError(io::Error),
    DeError(DeError),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ZusiValue {
    Info(Info),

    #[serde(rename = "result")]
    Result(ZusiResult),
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};

    use quick_xml::{de, se};
    use tempfile::tempdir;
    use time::macros::datetime;
    use crate::delphi_timestamp::DelphiTimestamp;
    use crate::xml::zusi::{Zusi, ZusiValue};
    use crate::xml::zusi::info::Info;
    use crate::xml::zusi::result::{ResultValue, ZusiResult};
    use crate::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};

    const EXPECTED_XML: &str = "<Zusi><Info DateiTyp=\"result\" Version=\"A.1\" MinVersion=\"A.0\"/><result Zugnummer=\"12345\" TfNummer=\"67890\" Datum=\"2019-01-01 23:14:00\" Verbrauch=\"0\" Bemerkung=\"\" Schummel=\"false\" Schwierigkeitsgrad=\"0\" EnergieVorgabe=\"0\"><FahrtEintrag FahrtTyp=\"0\" FahrtWeg=\"22.33\" FahrtZeit=\"2019-01-01 23:18:00\" Fahrtsp=\"0\" FahrtspStrecke=\"0\" FahrtspSignal=\"0\" FahrtspZugsicherung=\"0\" FahrtAutopilot=\"false\" Fahrtkm=\"0\" FahrtHLDruck=\"0\" FahrtParameter=\"0\" FahrtFplAnk=\"43410.3125\" FahrtFplAbf=\"43410.316406\" FahrtFBSchalter=\"0\"/><FahrtEintrag FahrtTyp=\"0\" FahrtWeg=\"22.43\" FahrtZeit=\"2019-01-02 01:07:00\" Fahrtsp=\"0\" FahrtspStrecke=\"0\" FahrtspSignal=\"0\" FahrtspZugsicherung=\"0\" FahrtAutopilot=\"false\" Fahrtkm=\"0\" FahrtHLDruck=\"0\" FahrtParameter=\"0\" FahrtFplAnk=\"43410.347656\" FahrtFplAbf=\"43410.351563\" FahrtFBSchalter=\"0\"/></result></Zusi>";
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
                                fahrt_fpl_ank: if include_optionals { Some(DelphiTimestamp(43410.3125)) } else { None },
                                fahrt_fpl_abf: if include_optionals { Some(DelphiTimestamp(43410.316406)) } else { None },
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
                                fahrt_fpl_ank: if include_optionals { Some(DelphiTimestamp(43410.347656)) } else { None },
                                fahrt_fpl_abf: if include_optionals { Some(DelphiTimestamp(43410.351563)) } else { None },
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
        assert_eq!(serialized, EXPECTED_XML);

        let deserialized: Zusi = de::from_str(&serialized).unwrap();
        assert_eq!(deserialized, expected_zusi(true));
    }

    #[test]
    fn test_serialize_deserialize_optionals() {
        let serialized = se::to_string(&expected_zusi(false)).unwrap();
        assert_eq!(serialized, EXPECTED_XML_WITHOUT_OPTIONALS);

        let deserialized: Zusi = de::from_str(&serialized).unwrap();
        assert_eq!(deserialized, expected_zusi(false));
    }

    #[test]
    fn test_from_xml() {
        let zusi = Zusi::from_xml(EXPECTED_XML).unwrap();
        assert_eq!(zusi, expected_zusi(true));
    }

    #[test]
    fn test_from_xml_file_by_path() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("xml_input_file.xml");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(EXPECTED_XML.as_bytes()).unwrap();

        let zusi: Zusi = Zusi::from_xml_file_by_path(&file_path).unwrap();

        assert_eq!(zusi, expected_zusi(true));
    }

    #[test]
    fn test_to_xml() {
        let xml = expected_zusi(true).to_xml().unwrap();
        assert_eq!(xml, EXPECTED_XML);
    }

    #[test]
    fn test_to_xml_file_by_path() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("xml_output_file.xml");

        expected_zusi(true).to_xml_file_by_path(&file_path).unwrap();

        let mut file = File::open(&file_path).unwrap();
        let mut xml = String::new();
        file.read_to_string(&mut xml).unwrap();
        assert_eq!(xml, EXPECTED_XML);
    }
}
