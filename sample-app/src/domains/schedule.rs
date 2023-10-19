use crate::domains::app_date::{AppDate, AppDateTimeSpan};

pub trait Task {
    fn command_name(&self) -> String;
}

pub trait ScheduleLike {
    fn ready(&self) -> Vec<Box<dyn Task>>;
    fn available(&self, span: dyn AppDateTimeSpan) -> Vec<Box<dyn Task>>;
    fn cancel_next(&self, task: dyn Task) -> Box<dyn ScheduleLike>;
}

pub trait BookLike {
    fn book_immediatly(&self, task: dyn Task) -> Box<dyn ScheduleLike>;
    fn book_at(&self, task: dyn Task, time: dyn AppDate) -> Box<dyn ScheduleLike>;
    fn book_in(&self, task: dyn Task, time: dyn AppDateTimeSpan) -> Box<dyn ScheduleLike>;
    fn book_every(
        &self,
        task: dyn Task,
        start: dyn AppDate,
        span: dyn AppDateTimeSpan,
    ) -> Box<dyn ScheduleLike>;
}
