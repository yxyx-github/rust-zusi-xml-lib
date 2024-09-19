use time::macros::datetime;
use time::{Duration, PrimitiveDateTime};

const DATE_TIME_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";
const DELPHI_EPOCH: PrimitiveDateTime = datetime!(1899-12-30 0:00);
const MILLISECONDS_OF_SINGLE_DAY: i128 = Duration::days(1).whole_milliseconds();

pub mod date_time_format {
    use serde::{de, ser, Deserialize, Deserializer, Serializer};
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

pub mod delphi_timestamp_option_format {
    use serde::{de, ser, Deserialize, Deserializer, Serializer};
    use time::{Duration, PrimitiveDateTime};

    use crate::xml::format::{DELPHI_EPOCH, MILLISECONDS_OF_SINGLE_DAY};

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
        Ok(Some(DelphiTimestamp(value).into()))
    }

    #[derive(Debug)]
    pub struct DelphiTimestamp(pub(crate) f64);

    impl PartialEq for DelphiTimestamp {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl From<PrimitiveDateTime> for DelphiTimestamp {
        fn from(pdt: PrimitiveDateTime) -> Self {
            let value = pdt - DELPHI_EPOCH;
            let days = value.whole_days();
            let fraction = (value - Duration::days(days)).whole_milliseconds() as f64 / MILLISECONDS_OF_SINGLE_DAY as f64;
            let timestamp = days as f64 + fraction;
            Self(timestamp)
        }
    }

    impl From<DelphiTimestamp> for PrimitiveDateTime {
        fn from(timestamp: DelphiTimestamp) -> Self {
            let pdt = DELPHI_EPOCH + Duration::days(timestamp.0.trunc() as i64);
            let pdt = pdt + Duration::milliseconds(
                (timestamp.0.fract() * MILLISECONDS_OF_SINGLE_DAY as f64) as i64
            );
            pdt
        }
    }

    #[cfg(test)]
    mod tests {
        use time::macros::datetime;
        use super::*;

        #[test]
        fn test_primitive_date_time_to_delphi_timestamp() {
            assert_eq!(DelphiTimestamp::from(datetime!(2000-12-30 12:30)), DelphiTimestamp(36890.520833333336));
        }

        #[test]
        fn test_delphi_timestamp_to_primitive_date_time() {
            assert_eq!(PrimitiveDateTime::from(DelphiTimestamp(36890.520833333336)), datetime!(2000-12-30 12:30:00));
        }
    }
}
