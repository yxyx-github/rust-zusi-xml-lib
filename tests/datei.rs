mod deserialized;
mod utils;

use crate::deserialized::datei;
use crate::deserialized::datei::DateiTestWrapper;
use crate::utils::TestRunner;

const XML_DATEIEN: &str = "./tests/serialized/datei.xml";

fn runner() -> TestRunner<DateiTestWrapper> {
    TestRunner::new_with_file(XML_DATEIEN, datei::dateien())
}

#[test]
fn test_serialize() {
    runner().test_serialize();
}

#[test]
fn test_deserialize() {
    runner().test_deserialize();
}