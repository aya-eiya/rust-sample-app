use crate::domains::gold::Gold;
use crate::domains::worker::WorkerLike;

pub struct WorkStats {
    pub worker: Box<dyn WorkerLike>,
    pub gold: Gold,
}
pub trait ResourceLike {
    fn try_dig(&self, worker: Box<dyn WorkerLike>) -> WorkStats;
}
