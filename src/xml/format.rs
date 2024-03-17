const FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

pub mod date_time_format {
    use serde::{Deserialize, Deserializer, ser, Serializer};
    use time::{format_description, PrimitiveDateTime};
    use crate::xml::format::FORMAT;

    pub fn serialize<S>(pdt: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let format = format_description::parse(FORMAT).map_err(ser::Error::custom)?;
        let formatted = pdt.format(&format).map_err(ser::Error::custom)?;
        serializer.serialize_str(&formatted).map_err(ser::Error::custom)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error> where D: Deserializer<'de> {
        let format = format_description::parse(FORMAT).map_err(serde::de::Error::custom)?;
        let s = String::deserialize(deserializer)?;
        PrimitiveDateTime::parse(&s, &format).map_err(serde::de::Error::custom)
    }
}

pub mod date_time_format_option {
    use serde::{Deserialize, Deserializer, ser, Serializer};
    use time::{format_description, PrimitiveDateTime};
    use crate::xml::format::{date_time_format, FORMAT};

    pub fn serialize<S>(pdt: &Option<PrimitiveDateTime>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match pdt {
            None => serializer.serialize_str("").map_err(ser::Error::custom),
            Some(pdt) => {
                date_time_format::serialize(pdt, serializer)
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
}