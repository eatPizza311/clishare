use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, From, Deserialize, Serialize)]
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn from_naive_utc(datetime: NaiveDateTime) -> Self {
        Time(DateTime::from_naive_utc_and_offset(datetime, Utc))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // s should be in format like 2024-09-19
        match format!("{}T00:00:00Z", s).parse::<DateTime<Utc>>() {
            // derive_more From will convert DateTime<Utc> -> Time<DateTime<Utc>>
            Ok(time) => Ok(time.into()),
            Err(e) => Err(e),
        }
    }
}
