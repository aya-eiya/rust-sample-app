use crate::domains::tool::Tool;

type WorkerEfficiency = u32;

pub trait Worker {
    fn health(&self) -> i32;
    fn tool(&self) -> Box<dyn Tool>;
    fn efficiency(&self) -> WorkerEfficiency;
}
