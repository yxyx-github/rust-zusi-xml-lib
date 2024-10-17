use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

pub fn read_xml_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();
    let mut raw_xml = String::new();
    file.read_to_string(&mut raw_xml).unwrap();
    let parsed_xml = Regex::new("[ \n\r\t]+").unwrap().replace_all(raw_xml.trim(), " ");
    let parsed_xml = Regex::new("> <").unwrap().replace_all(&parsed_xml, "><");
    let parsed_xml = Regex::new(" >").unwrap().replace_all(&parsed_xml, ">");
    let parsed_xml = Regex::new(" />").unwrap().replace_all(&parsed_xml, "/>");
    parsed_xml.to_string()
}
