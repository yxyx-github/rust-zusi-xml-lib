use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use crate::xml::zusi::info::Info;
use crate::xml::zusi::result::ZusiResult;
use crate::xml::zusi::zug::Zug;
pub use quick_xml::DeError;
use quick_xml::{de, se, SeError};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::buchfahrplan::Buchfahrplan;
use crate::xml::zusi::fahrplan::Fahrplan;

pub mod lib;
pub mod info;
pub mod result;
pub mod zug;
pub mod fahrplan;
pub mod buchfahrplan;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Zusi {
    #[serde(rename = "Info")]
    pub info: Info,

    #[serde(rename = "$value")]
    pub value: ZusiValue,
}

impl Zusi {
    pub fn from_xml(xml: &str) -> Result<Self, DeError> {
        de::from_str(xml)
    }

    pub fn from_xml_file_by_path<P: AsRef<Path>>(path: P) -> Result<Self, ReadZusiXMLFileError> {
        let mut file = File::open(path)
            .map_err(|err| ReadZusiXMLFileError::IOError(err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| ReadZusiXMLFileError::IOError(err))?;
        Ok(
            Zusi::from_xml(&contents)
                .map_err(|err| ReadZusiXMLFileError::DeError(err))?
        )
    }

    pub fn to_xml(&self) -> Result<String, SeError> {
        se::to_string(self)
    }

    pub fn to_xml_file_by_path<P: AsRef<Path>>(&self, path: P) -> Result<(), WriteZusiXMLFileError> {
        let xml = self.to_xml()
            .map_err(|err| WriteZusiXMLFileError::SeError(err))?;
        let mut file = File::create(path)
            .map_err(|err| WriteZusiXMLFileError::IOError(err))?;
        file.write_all(xml.as_bytes())
            .map_err(|err| WriteZusiXMLFileError::IOError(err))?;
        Ok(())
    }
}

impl AsRef<Zusi> for Zusi {
    fn as_ref(&self) -> &Zusi {
        &self
    }
}

#[derive(Debug)]
pub enum ReadZusiXMLFileError {
    IOError(io::Error),
    DeError(DeError),
}

#[derive(Debug)]
pub enum WriteZusiXMLFileError {
    IOError(io::Error),
    SeError(SeError),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ZusiValue {
    /// Version A.1
    #[serde(rename = "result")]
    Result(ZusiResult),

    /// Version A.6
    #[serde(rename = "Fahrplan")]
    Fahrplan(Fahrplan),

    /// Version A.6
    #[serde(rename = "Zug")]
    Zug(Zug),

    /// Version A.4
    #[serde(rename = "Buchfahrplan")]
    Buchfahrplan(Buchfahrplan),
}