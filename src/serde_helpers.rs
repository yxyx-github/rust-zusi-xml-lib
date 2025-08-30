pub mod delphi_timestamp_option_format {
    use serde::{de, ser, Deserialize, Deserializer, Serializer};
    use crate::delphi_timestamp::DelphiTimestamp;

    pub fn serialize<S>(timestamp: &Option<DelphiTimestamp>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match timestamp {
            None => serializer.serialize_str("").map_err(ser::Error::custom),
            Some(timestamp) => {
                serializer.serialize_f64(timestamp.0).map_err(ser::Error::custom)
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DelphiTimestamp>, D::Error> where D: Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        if value == "" {
            return Ok(None);
        }
        let value: f64 = value.parse().map_err(de::Error::custom)?;
        Ok(Some(DelphiTimestamp::from(value).into()))
    }
}