mod deserialized;
mod utils;

use zusi_xml_lib::xml::zusi::Zusi;
use crate::deserialized::result;
use crate::utils::TestRunner;

const XML_PATH: &str = "./tests/serialized/result.xml";
const XML_PATH_WITHOUT_OPTIONALS: &str = "./tests/serialized/result-without-optionals.xml";

fn runner() -> TestRunner<Zusi> {
    TestRunner::new_with_file(XML_PATH, result::deserialized(true))
}

fn runner_without_optionals() -> TestRunner<Zusi> {
    TestRunner::new_with_file(XML_PATH_WITHOUT_OPTIONALS, result::deserialized(false))
}

#[test]
fn test_serialize() {
    runner().test_serialize();
}

#[test]
fn test_serialize_optionals() {
    runner_without_optionals().test_serialize();
}

#[test]
fn test_deserialize() {
    runner().test_serialize();
}

#[test]
fn test_deserialize_optionals() {
    runner_without_optionals().test_serialize();
}