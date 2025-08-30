use quick_xml::{de, se};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_helpers::xml::test_utils::read_xml_file;
use std::fmt::Debug;
use std::path::Path;

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