use time::macros::datetime;
use time::{Duration, PrimitiveDateTime};

const DELPHI_EPOCH: PrimitiveDateTime = datetime!(1899-12-30 0:00);
const MILLISECONDS_OF_SINGLE_DAY: i128 = Duration::days(1).whole_milliseconds();

#[derive(Copy, Clone, Debug)]
pub struct DelphiTimestamp(pub(crate) f64);

impl PartialEq for DelphiTimestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl From<f64> for DelphiTimestamp {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<DelphiTimestamp> for f64 {
    fn from(value: DelphiTimestamp) -> Self {
        value.0
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
        assert_eq!(DelphiTimestamp::from(datetime!(2000-12-30 12:30)), DelphiTimestamp::from(36890.520833333336));
    }

    #[test]
    fn test_delphi_timestamp_to_primitive_date_time() {
        assert_eq!(PrimitiveDateTime::from(DelphiTimestamp::from(36890.520833333336)), datetime!(2000-12-30 12:30:00));
    }
}
