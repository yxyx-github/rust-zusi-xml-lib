mod deserialized;
mod utils;

use crate::deserialized::result;
use crate::utils::TestRunner;

const XML_PATH: &str = "./tests/serialized/result.xml";
const XML_PATH_WITHOUT_OPTIONALS: &str = "./tests/serialized/result-without-optionals.xml";

#[test]
fn test_serialize() {
    TestRunner::new_with_file(XML_PATH, result::deserialized(true)).test_serialize();
}

#[test]
fn test_serialize_optionals() {
    TestRunner::new_with_file(XML_PATH_WITHOUT_OPTIONALS, result::deserialized(false)).test_serialize();
}

#[test]
fn test_deserialize() {
    TestRunner::new_with_file(XML_PATH, result::deserialized(true)).test_serialize();
}

#[test]
fn test_deserialize_optionals() {
    TestRunner::new_with_file(XML_PATH_WITHOUT_OPTIONALS, result::deserialized(false)).test_serialize();
}