use crate::xml::zusi::lib::path::zusi_environment_path::ZusiEnvironmentPath;
use serde_helpers::xml::{FromXML, ReadXMLFileError, ToXML, WriteXMLFileError};

pub trait FromZusiXMLFile: FromXML {
    fn from_zusi_xml_file(path: impl AsRef<ZusiEnvironmentPath>) -> Result<Self, ReadXMLFileError>;
}

impl<T> FromZusiXMLFile for T
where
    T: FromXML,
{
    fn from_zusi_xml_file(path: impl AsRef<ZusiEnvironmentPath>) -> Result<Self, ReadXMLFileError> {
        path.as_ref().read_xml_file()
    }
}

pub trait ToZusiXMLFile: ToXML {
    fn to_zusi_xml_file(&self, path: impl AsRef<ZusiEnvironmentPath>, declaration_header: bool) -> Result<(), WriteXMLFileError>;

    fn force_to_zusi_xml_file(&self, path: impl AsRef<ZusiEnvironmentPath>, declaration_header: bool) -> Result<(), WriteXMLFileError>;
}

impl<T> ToZusiXMLFile for T
where
    T: ToXML,
{
    fn to_zusi_xml_file(&self, path: impl AsRef<ZusiEnvironmentPath>, declaration_header: bool) -> Result<(), WriteXMLFileError> {
        path.as_ref().write_xml_file(self, declaration_header)
    }

    fn force_to_zusi_xml_file(&self, path: impl AsRef<ZusiEnvironmentPath>, declaration_header: bool) -> Result<(), WriteXMLFileError> {
        path.as_ref().force_write_xml_file(self, declaration_header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_helpers::xml::test_utils::cleanup_xml;
    use std::fs;
    use tempfile::tempdir;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        #[serde(rename = "@attr")]
        attr: String,

        #[serde(rename = "TestDataChild")]
        child: TestDataChild,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestDataChild {
        #[serde(rename = "@attr")]
        attr: String,
    }

    const EXPECTED_SERIALIZED: &'static str = r#"
        <TestData attr="value">
            <TestDataChild attr="child value"/>
        </TestData>
    "#;

    const EXPECTED_SERIALIZED_WITH_DECLARATION_HEADER: &'static str = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <TestData attr="value">
            <TestDataChild attr="child value"/>
        </TestData>
    "#;

    fn expected_deserialized() -> TestData {
        TestData {
            attr: "value".into(),
            child: TestDataChild {
                attr: "child value".into(),
            }
        }
    }

    #[test]
    fn test_from_zusi_xml_file() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_input_file.xml");
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        fs::write(&file_path, EXPECTED_SERIALIZED).unwrap();

        let env = ZusiEnvironmentPath::new(tmp_dir.path(), tmp_dir.path()).with_path("some_dir/xml_input_file.xml").unwrap();
        let test_data = TestData::from_zusi_xml_file(&env).unwrap();

        assert_eq!(test_data, expected_deserialized());
    }

    #[test]
    fn test_from_zusi_xml_file_with_declaration_header() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_input_file.xml");
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        fs::write(&file_path, EXPECTED_SERIALIZED_WITH_DECLARATION_HEADER).unwrap();

        let env = ZusiEnvironmentPath::new(tmp_dir.path(), tmp_dir.path()).with_path("some_dir/xml_input_file.xml").unwrap();
        let test_data = TestData::from_zusi_xml_file(&env).unwrap();

        assert_eq!(test_data, expected_deserialized());
    }

    #[test]
    fn test_to_zusi_xml_file() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_output_file.xml");

        let env = ZusiEnvironmentPath::new(tmp_dir.path(), tmp_dir.path()).with_path("some_dir/xml_output_file.xml").unwrap();
        expected_deserialized().to_zusi_xml_file(&env, false).unwrap();

        let xml = fs::read_to_string(&file_path).unwrap();
        assert_eq!(cleanup_xml(xml), cleanup_xml(EXPECTED_SERIALIZED.into()));
    }

    #[test]
    fn test_to_zusi_xml_file_with_declaration_header() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_output_file.xml");

        let env = ZusiEnvironmentPath::new(tmp_dir.path(), tmp_dir.path()).with_path("some_dir/xml_output_file.xml").unwrap();
        expected_deserialized().to_zusi_xml_file(&env, true).unwrap();

        let xml = fs::read_to_string(&file_path).unwrap();
        assert_eq!(cleanup_xml(xml), cleanup_xml(EXPECTED_SERIALIZED_WITH_DECLARATION_HEADER.into()));
    }

    #[test]
    fn test_force_to_zusi_xml_file() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_output_file.xml");

        let env = ZusiEnvironmentPath::new(tmp_dir.path(), tmp_dir.path()).with_path("some_dir/xml_output_file.xml").unwrap();
        expected_deserialized().force_to_zusi_xml_file(&env, false).unwrap();

        let xml = fs::read_to_string(&file_path).unwrap();
        assert_eq!(cleanup_xml(xml), cleanup_xml(EXPECTED_SERIALIZED.into()));
    }

    #[test]
    fn test_force_to_zusi_xml_file_with_declaration_header() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_output_file.xml");

        let env = ZusiEnvironmentPath::new(tmp_dir.path(), tmp_dir.path()).with_path("some_dir/xml_output_file.xml").unwrap();
        expected_deserialized().force_to_zusi_xml_file(&env, true).unwrap();

        let xml = fs::read_to_string(&file_path).unwrap();
        assert_eq!(cleanup_xml(xml), cleanup_xml(EXPECTED_SERIALIZED_WITH_DECLARATION_HEADER.into()));
    }
}