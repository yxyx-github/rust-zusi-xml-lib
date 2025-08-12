mod deserialized;
mod utils;

use crate::deserialized::zug;
use crate::utils::TestRunner;
use zusi_xml_lib::xml::zusi::Zusi;

const XML_ALL: &str = "./tests/serialized/zug.xml";
const XML_WITH_DEFAULTS: &str = "./tests/serialized/zug-with-defaults.xml";

fn runner() -> TestRunner<Zusi> {
    TestRunner::new_with_file(XML_ALL, zug::all())
}

fn runner_without_optionals() -> TestRunner<Zusi> {
    TestRunner::new_with_file(XML_WITH_DEFAULTS, zug::with_defaults())
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
    runner().test_deserialize();
}

#[test]
fn test_deserialize_optionals() {
    runner_without_optionals().test_deserialize();
}