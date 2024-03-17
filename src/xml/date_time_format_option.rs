use serde::{Deserialize, Deserializer, ser, Serializer};
use time::{format_description, PrimitiveDateTime};

const FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

pub fn serialize<S>(pdt: &Option<PrimitiveDateTime>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let format = format_description::parse(FORMAT).map_err(ser::Error::custom)?;
    match pdt {
        None => serializer.serialize_str("").map_err(ser::Error::custom),
        Some(pdt) => {
            let formatted = pdt.format(&format).map_err(ser::Error::custom)?;
            serializer.serialize_str(&formatted).map_err(ser::Error::custom)
        }
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error> where D: Deserializer<'de> {
    let format = format_description::parse(FORMAT).map_err(serde::de::Error::custom)?;
    let s = String::deserialize(deserializer)?;
    Ok(match PrimitiveDateTime::parse(&s, &format) {
        Ok(pdt) => Some(pdt),
        Err(_) => None,
    })
}