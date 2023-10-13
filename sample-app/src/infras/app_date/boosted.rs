use crate::domains::app_date::{AppDate, AppDateService};
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

pub struct BoostedAppDate {
    current: DateTime<Utc>,
}

pub struct BoostedAppDateService {
    base: DateTime<Utc>,
    offset_days: i32,
    coefficient: u32,
}

impl AppDate for BoostedAppDate {
    fn year(&self) -> i32 {
        self.current.year()
    }

    fn month(&self) -> u32 {
        self.current.month()
    }

    fn date(&self) -> u32 {
        self.current.day()
    }

    fn hour(&self) -> u32 {
        self.current.hour()
    }

    fn min(&self) -> u32 {
        self.current.minute()
    }

    fn second(&self) -> u32 {
        self.current.second()
    }

    fn milli_second(&self) -> u32 {
        self.current.timestamp_subsec_millis()
    }
}

impl BoostedAppDateService {
    pub fn new(base_date: &'static str, offset_days: i32, coefficient: u32) -> Self {
        let base = match DateTime::parse_from_rfc3339(base_date) {
            Ok(d) => d.with_timezone(&Utc),
            Err(_) => Utc::now(),
        };

        Self {
            base,
            offset_days,
            coefficient,
        }
    }
}

impl AppDateService for BoostedAppDateService {
    fn now(&self) -> Box<dyn AppDate> {
        let base = self.base.timestamp_millis();
        let system = Utc::now().timestamp_millis();
        let current = system + i64::from(self.offset_days) * 24 * 60 * 60 * 1000;
        let now = base + (current - base) * i64::from(self.coefficient);

        Box::new(BoostedAppDate {
            current: match NaiveDateTime::from_timestamp_millis(now) {
                Some(d) => d.and_utc(),
                None => Utc::now(),
            },
        })
    }
}
