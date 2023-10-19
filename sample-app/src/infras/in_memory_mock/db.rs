use std::borrow::BorrowMut;

use crate::domains::{
    gold::Gold,
    resource::{ResourceLike, WorkStats},
    tool::ToolEfficiency,
    tool::ToolLike,
    worker::{WorkerEfficiency, WorkerLike},
};
use futures::lock::Mutex;
use once_cell::sync::Lazy;

type ResouceId = u32;
type WorkerId = u32;
type ToolId = u32;
type ToolMasterId = u32;

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: ResouceId,
    pub deposit_amount: u32,
}

#[derive(Debug, Clone)]
pub struct Worker {
    pub id: WorkerId,
    pub name: String,
    pub health: u32,
    pub tool_id: Option<ToolId>,
}

#[derive(Debug, Clone)]
pub struct Tool {
    id: ToolId,
    attrition_rate: u32,
    master_id: ToolMasterId,
}

#[derive(Debug, Clone)]
pub struct ToolBody {
    pub attrition_rate: u32,
    pub master_id: ToolMasterId,
}

#[derive(Debug, Clone)]
pub struct ToolMaster {
    pub id: ToolMasterId,
    pub name: String,
    pub price: u32,
    pub efficiency: ToolEfficiency,
}

#[derive(Debug, Clone)]
pub struct ToolMasterBody {
    pub name: String,
    pub price: i32,
    pub master_id: ToolMasterId,
}

#[derive(Debug)]
pub struct Master {
    tools: Mutex<Vec<ToolMaster>>,
}

#[derive(Debug)]
pub struct DataBase {
    resources: Mutex<Vec<Resource>>,
    workers: Mutex<Vec<Worker>>,
    tools: Mutex<Vec<Tool>>,
}

impl DataBase {
    fn new() -> DataBase {
        DataBase {
            resources: Mutex::new(vec![]),
            workers: Mutex::new(vec![]),
            tools: Mutex::new(vec![]),
        }
    }
}

impl Master {
    fn new() -> Master {
        Master {
            tools: Mutex::new(vec![]),
        }
    }
}

struct InMemoryDBContext {
    db: DataBase,
    master: Master,
}

impl InMemoryDBContext {
    fn new(&self) -> InMemoryDBContext {
        InMemoryDBContext {
            db: DataBase::new(),
            master: Master::new(),
        }
    }
}

static DATA_BASE: Lazy<InMemoryDBContext> = Lazy<InMemoryDBContext>::new(InMemoryDBContext::new);

trait DbMember {
    fn db(&self) -> DataBase;
    fn master(&self) -> Master;
}

impl DbMember for Resource {}
impl ResourceLike for Resource {
    fn try_dig(&self, worker: Box<dyn WorkerLike>) -> WorkStats {
        let hlth = worker.health() - 1;
        let tool = match worker
            .tool()
            .and_then(|t| self.db().find_tool_by_id(t.id()))
        {
            Some(tt) => self.db().update_tool(
                tt.id,
                Tool {
                    id: tt.id,
                    master_id: tt.master_id,
                    attrition_rate: if tt.attrition_rate - 1 > 0 {
                        tt.attrition_rate - 1
                    } else {
                        100
                    },
                },
            ),
            None => Some(self.db().create_tool(ToolBody {
                attrition_rate: 100,
                master_id: 0,
            })),
        };
        let max = self.deposit_amount / 100;
        let amount = max * (worker.tool().map_or(1, |t| t.efficiency()) * worker.efficiency());
        let remain = max - amount;
        let up_worker = self.db().update_worker(
            worker.id(),
            Worker {
                id: worker.id(),
                name: worker.name(),
                health: if hlth > 0 { hlth } else { 0 },
                tool_id: tool.map(|t| t.id),
            },
        );
        self.db().update_resource(
            self.id,
            Resource {
                id: self.id,
                deposit_amount: remain,
            },
        );
        WorkStats {
            worker: match up_worker {
                Some(w) => Box::new(w),
                None => worker,
            },
            gold: Gold { amount },
        }
    }
}

pub trait ResourcesAsTable {
    fn create_resource(&mut self, item: ToolBody) -> Tool;
    fn find_resource_by_id(&self, id: ResouceId) -> Option<Resource>;
    fn update_resource(&mut self, id: ResouceId, item: Resource) -> Option<Resource>;
}

pub trait WorkerAsTable {
    fn create_worker(&mut self, item: ToolBody) -> Tool;
    fn find_worker_by_id(&self, id: WorkerId) -> Option<Worker>;
    fn update_worker(&mut self, id: WorkerId, item: Worker) -> Option<Worker>;
}

pub trait ToolAsTable {
    fn create_tool(&mut self, item: ToolBody) -> Tool;
    fn find_tool_by_id(&self, id: ToolId) -> Option<Tool>;
    fn update_tool(&mut self, id: ToolId, item: Tool) -> Option<Tool>;
}

pub trait ToolMasterAsTable {
    fn create_tool_master(&self, item: ToolMasterBody) -> ToolMaster;
    fn find_tool_master_by_id(&self, id: ToolMasterId) -> Option<ToolMaster>;
}

impl ResourcesAsTable for DataBase {
    fn find_resource_by_id(&self, id: ResouceId) -> Option<Resource> {
        todo!()
    }

    fn update_resource(&mut self, id: ResouceId, item: Resource) -> Option<Resource> {
        todo!()
    }

    fn create_resource(&mut self, item: ToolBody) -> Tool {
        todo!()
    }
}

impl DbMember for Worker {}

impl WorkerLike for Worker {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn health(&self) -> u32 {
        self.health
    }

    fn tool(&self) -> Option<Box<dyn ToolLike>> {
        let t = self
            .tool_id
            .and_then(|id| self.db().find_tool_by_id(id))
            .map(|t| Box::new(t) as Box<dyn ToolLike>);
        return t;
    }

    fn efficiency(&self) -> WorkerEfficiency {
        todo!()
    }
}

impl WorkerAsTable for DataBase {
    fn find_worker_by_id(&self, id: WorkerId) -> Option<Worker> {
        self.workers.iter().find(|i| i.id == id).map(|i| i.clone())
    }

    fn update_worker(&self, id: WorkerId, item: Worker) -> Option<Worker> {
        todo!()
    }
}

impl DbMember for Tool {}

impl ToolLike for Tool {
    fn id(&self) -> u32 {
        self.id
    }

    fn attrition_rate(&self) -> u32 {
        self.attrition_rate
    }

    fn price(&self) -> u32 {
        let t = self
            .master()
            .find_tool_master_by_id(self.master_id)
            .map_or(0, |m| m.price / 100 * self.attrition_rate());
        return t;
    }

    fn efficiency(&self) -> ToolEfficiency {
        let t = self
            .master()
            .find_tool_master_by_id(self.master_id)
            .map_or(0, |m| m.efficiency / 100 * self.attrition_rate());
        return t;
    }
}

impl ToolAsTable for DataBase {
    fn find_tool_by_id(&self, id: ToolId) -> Option<Tool> {
        self.tools.iter().find(|i| i.id() == id).map(|i| i.clone())
    }

    fn update_tool(&mut self, id: ToolId, item: Tool) -> Option<Tool> {
        Some(item)
    }

    fn create_tool(&self, item: ToolBody) -> Tool {
        todo!()
    }
}

impl ToolMasterAsTable for Master {
    fn create_tool_master(&self, item: ToolMasterBody) -> ToolMaster {
        todo!()
    }

    fn find_tool_master_by_id(&self, id: ToolMasterId) -> Option<ToolMaster> {
        todo!()
    }
}
