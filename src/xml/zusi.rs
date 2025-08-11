use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use crate::xml::zusi::info::Info;
use crate::xml::zusi::result::ZusiResult;
pub use quick_xml::DeError;
use quick_xml::{de, se, SeError};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub mod info;
pub mod result;
pub mod lib;

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
    #[serde(rename = "result")]
    Result(ZusiResult),
}