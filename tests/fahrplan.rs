mod deserialized;
mod utils;

use crate::deserialized::fahrplan;
use crate::utils::TestRunner;
use zusi_xml_lib::xml::zusi::Zusi;

const XML_ALL: &str = "./tests/serialized/fahrplan.xml";
const XML_WITH_DEFAULTS: &str = "./tests/serialized/fahrplan_with-defaults.xml";

fn runner() -> TestRunner<Zusi> {
    TestRunner::new_with_file(XML_ALL, fahrplan::all())
}

fn runner_without_optionals() -> TestRunner<Zusi> {
    TestRunner::new_with_file(XML_WITH_DEFAULTS, fahrplan::with_defaults())
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