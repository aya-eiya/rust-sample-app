use async_trait::async_trait;

pub trait AppDate {
    fn year(&self) -> i32;
    fn month(&self) -> u32;
    fn date(&self) -> u32;
    fn hour(&self) -> u32;
    fn min(&self) -> u32;
    fn second(&self) -> u32;
    fn milli_second(&self) -> u32;

    #[inline]
    fn display(&self) -> String {
        let y = self.year();
        let m = self.month();
        let d = self.date();
        let h = self.hour();
        let n = self.min();
        let s = self.second();
        let i = self.milli_second();
        let str = format!("{y}-{m:02}-{d:02} {h:02}:{n:02}:{s:02}:{s:02}.{i:03}");
        return str;
    }
}

#[async_trait]
pub trait AppDateService {
    async fn now(&self) -> Box<dyn AppDate>;
}

pub trait AppDateTimeSpan {
    fn display(&self) -> String;
}
