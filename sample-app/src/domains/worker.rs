use crate::domains::tool::ToolLike;

pub type WorkerEfficiency = u32;

pub trait WorkerLike {
    fn id(&self) -> u32;
    fn name(&self) -> String;
    fn health(&self) -> u32;
    fn tool(&self) -> Option<Box<dyn ToolLike>>;
    fn efficiency(&self) -> WorkerEfficiency;
}
