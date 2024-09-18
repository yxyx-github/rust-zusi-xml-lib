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

pub mod delphi_timestamp_format {
    use serde::{de, ser, Deserialize, Deserializer, Serializer};
    use time::{Duration, PrimitiveDateTime};

    use crate::xml::format::{DELPHI_EPOCH, MILLISECONDS_OF_SINGLE_DAY};

    pub fn serialize<S>(pdt: &Option<PrimitiveDateTime>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match pdt {
            None => serializer.serialize_str("").map_err(ser::Error::custom),
            Some(pdt) => {
                serializer.serialize_f64(datetime_to_delphi_timestamp(*pdt)).map_err(ser::Error::custom)
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error> where D: Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        if value == "" {
            return Ok(None);
        }
        let value: f64 = value.parse().map_err(de::Error::custom)?;
        Ok(Some(delphi_timestamp_to_datetime(value)))
    }

    fn datetime_to_delphi_timestamp(pdt: PrimitiveDateTime) -> f64 {
        let value = pdt - DELPHI_EPOCH;
        let days = value.whole_days();
        let fraction = (value - Duration::days(days)).whole_milliseconds() as f64 / MILLISECONDS_OF_SINGLE_DAY as f64;
        let timestamp = days as f64 + fraction;
        timestamp
    }

    fn delphi_timestamp_to_datetime(timestamp: f64) -> PrimitiveDateTime {
        let datetime = DELPHI_EPOCH + Duration::days(timestamp.trunc() as i64);
        let datetime = datetime + Duration::milliseconds(
            (timestamp.fract() * MILLISECONDS_OF_SINGLE_DAY as f64) as i64
        );
        datetime
    }

    #[cfg(test)]
    mod tests {
        use time::macros::datetime;
        use super::*;

        #[test]
        fn test_datetime_to_delphi_timestamp() {
            assert_eq!(datetime_to_delphi_timestamp(datetime!(2000-12-30 12:30)), 36890.520833333336);
        }

        #[test]
        fn test_delphi_timestamp_to_datetime() {
            assert_eq!(delphi_timestamp_to_datetime(36890.520833333336), datetime!(2000-12-30 12:30:00));
        }
    }
}
