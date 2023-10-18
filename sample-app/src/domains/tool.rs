type ToolEfficiency = u32;

pub trait Tool {
    fn price(&self) -> u32;
    fn attrition_rate(&self) -> u32;
    fn efficiency(&self) -> ToolEfficiency;
}
