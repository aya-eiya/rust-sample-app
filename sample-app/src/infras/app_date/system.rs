use crate::domains::app_date::{AppDate, AppDateService};
use async_trait::async_trait;
use chrono::{DateTime, Datelike, Timelike, Utc};

pub struct SystemAppDate {
    current: DateTime<Utc>,
}
pub struct SystemAppDateService {}

impl AppDate for SystemAppDate {
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

impl SystemAppDateService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AppDateService for SystemAppDateService {
    async fn now(&self) -> Box<dyn AppDate> {
        Box::new(SystemAppDate {
            current: Utc::now(),
        })
    }
}
