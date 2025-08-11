use quick_xml::{de, se};
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::fs;
use std::path::Path;

pub fn read_xml_file<P: AsRef<Path>>(path: P) -> String {
    let raw_xml = fs::read_to_string(path).unwrap();
    cleanup_xml(raw_xml)
}

pub fn cleanup_xml(raw_xml: String) -> String {
    let parsed_xml = Regex::new("[ \n\r\t]+").unwrap().replace_all(raw_xml.trim(), " ");
    let parsed_xml = Regex::new("> <").unwrap().replace_all(&parsed_xml, "><");
    let parsed_xml = Regex::new(" >").unwrap().replace_all(&parsed_xml, ">");
    let parsed_xml = Regex::new(" />").unwrap().replace_all(&parsed_xml, "/>");
    parsed_xml.to_string()
}

#[allow(dead_code)]
pub struct TestRunner<T: Debug + PartialEq + Serialize + DeserializeOwned> {
    expected_serialized: String,
    expected_deserialized: T,
}

#[allow(dead_code)]
impl<T: Debug + PartialEq + Serialize + DeserializeOwned> TestRunner<T> {
    pub fn new_with_file<P: AsRef<Path>>(serialized: P, deserialized: T) -> TestRunner<T> {
        Self {
            expected_serialized: read_xml_file(serialized),
            expected_deserialized: deserialized,
        }
    }

    pub fn new_with_string<P: AsRef<Path>>(serialized: String, deserialized: T) -> TestRunner<T> {
        Self {
            expected_serialized: serialized,
            expected_deserialized: deserialized,
        }
    }

    pub fn test_serialize(&self) {
        let serialized = se::to_string(&self.expected_deserialized).unwrap();
        assert_eq!(serialized, self.expected_serialized);
    }

    pub fn test_deserialize(&self) {
        let deserialized: T = de::from_str(&self.expected_serialized).unwrap();
        assert_eq!(deserialized, self.expected_deserialized);
    }
}