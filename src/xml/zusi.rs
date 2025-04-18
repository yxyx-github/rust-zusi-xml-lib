use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use quick_xml::{de, se, SeError};
pub use quick_xml::DeError;
use serde::{Deserialize, Serialize};
use crate::xml::zusi::buchfahrplan::Buchfahrplan;
use crate::xml::zusi::fahrplan::Fahrplan;
use crate::xml::zusi::info::Info;
use crate::xml::zusi::result::ZusiResult;
use crate::xml::zusi::zug::Zug;

pub mod info;
pub mod result;
pub mod buchfahrplan;
pub mod lib;
pub mod zug;
mod fahrplan;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
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

    pub fn to_xml(&self) -> Result<String, SeError> {
        se::to_string(self)
    }

    pub fn to_xml_file_by_path<P: AsRef<Path>>(&self, path: P) -> Result<(), ZusiXMLFileError> {
        let xml = self.to_xml()
            .map_err(|err| ZusiXMLFileError::SeError(err))?;
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
    SeError(SeError),
    DeError(DeError),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ZusiValue {
    #[serde(rename = "Info")]
    Info(Info),

    #[serde(rename = "result")]
    Result(ZusiResult),

    #[serde(rename = "Fahrplan")]
    Fahrplan(Fahrplan),

    #[serde(rename = "Zug")]
    Zug(Zug),

    #[serde(rename = "Buchfahrplan")]
    Buchfahrplan(Buchfahrplan),
}
