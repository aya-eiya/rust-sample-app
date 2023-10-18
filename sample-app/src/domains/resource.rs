use crate::domains::gold::Gold;
use crate::domains::worker::Worker;

pub struct WorkStats {
    worker: Box<dyn Worker>,
    gold: Gold,
}
pub trait Resource {
    fn try_dig(&self, worker: dyn Worker) -> WorkStats;
}
