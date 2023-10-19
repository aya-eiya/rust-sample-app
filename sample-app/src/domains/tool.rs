pub type ToolEfficiency = u32;

pub trait ToolLike {
    fn id(&self) -> i32;
    fn price(&self) -> u32;
    fn attrition_rate(&self) -> u32;
    fn efficiency(&self) -> ToolEfficiency;
}
