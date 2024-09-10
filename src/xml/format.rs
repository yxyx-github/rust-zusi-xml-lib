const DATE_TIME_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

pub mod date_time_format {
    use serde::{Deserialize, Deserializer, ser, de, Serializer};
    use time::{format_description, PrimitiveDateTime};

    use crate::xml::format::DATE_TIME_FORMAT;

    pub fn serialize<S>(pdt: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let format = format_description::parse(DATE_TIME_FORMAT).map_err(ser::Error::custom)?;
        let formatted = pdt.format(&format).map_err(ser::Error::custom)?;
        serializer.serialize_str(&formatted).map_err(ser::Error::custom)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error> where D: Deserializer<'de> {
        let format = format_description::parse(DATE_TIME_FORMAT).map_err(de::Error::custom)?;
        let s = String::deserialize(deserializer)?;
        PrimitiveDateTime::parse(&s, &format).map_err(de::Error::custom)
    }
}
