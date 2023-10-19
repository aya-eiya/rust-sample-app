use crate::domains::tool::ToolLike;

pub type WorkerEfficiency = u32;

pub trait WorkerLike {
    fn id(&self) -> i32;
    fn name(&self) -> String;
    fn health(&self) -> i32;
    fn tool(&self) -> Option<Box<dyn ToolLike>>;
    fn efficiency(&self) -> WorkerEfficiency;
}
