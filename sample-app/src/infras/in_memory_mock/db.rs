use crate::domains::{
    gold::Gold,
    resource::{ResourceLike, WorkStats},
    tool::ToolEfficiency,
    tool::ToolLike,
    worker::{WorkerEfficiency, WorkerLike},
};
use once_cell::sync::Lazy;

type ResouceId = i32;
type WorkerId = i32;
type ToolId = i32;
type ToolMasterId = i32;

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: ResouceId,
    pub deposit_amount: u32,
}

#[derive(Debug, Clone)]
pub struct Worker {
    pub id: WorkerId,
    pub name: String,
    pub health: i32,
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
    pub id: ToolId,
    pub name: String,
    pub price: i32,
    pub master_id: ToolMasterId,
}

#[derive(Debug, Clone)]
pub struct ToolMasterBody {
    pub name: String,
    pub price: i32,
    pub master_id: ToolMasterId,
}

#[derive(Debug, Clone)]
pub struct Master {
    pub tools: Vec<ToolMaster>,
}

#[derive(Debug, Clone)]
pub struct DataBase {
    resources: Vec<Resource>,
    workers: Vec<Worker>,
    tools: Vec<Tool>,
}

impl DataBase {
    fn new() -> DataBase {
        DataBase {
            resources: vec![],
            workers: vec![],
            tools: vec![],
        }
    }
}
impl Master {
    fn new() -> Master {
        Master { tools: vec![] }
    }
}

static DB: Lazy<DataBase> = Lazy::new(|| DataBase::new());
static MASTER: Lazy<Master> = Lazy::new(|| Master::new());

trait DbMember {
    fn db(&self) -> DataBase {
        DB.clone()
    }
    fn master(&self) -> Master {
        MASTER.clone()
    }
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
    fn find_resource_by_id(&self, id: ResouceId) -> Option<Resource>;
    fn update_resource(&self, id: ResouceId, item: Resource) -> Option<Resource>;
}

pub trait WorkerAsTable {
    fn find_worker_by_id(&self, id: WorkerId) -> Option<Worker>;
    fn update_worker(&self, id: WorkerId, item: Worker) -> Option<Worker>;
}

pub trait ToolAsTable {
    fn create_tool(&self, item: ToolBody) -> Tool;
    fn find_tool_by_id(&self, id: ToolId) -> Option<Tool>;
    fn update_tool(&self, id: ToolId, item: Tool) -> Option<Tool>;
}

pub trait ToolMasterAsTable {
    fn create_tool_master(&self, item: ToolMasterBody) -> ToolMaster;
    fn find_tool_master_by_id(&self, id: ToolMasterId) -> Option<ToolMaster>;
}

impl ResourcesAsTable for DataBase {
    fn find_resource_by_id(&self, id: ResouceId) -> Option<Resource> {
        todo!()
    }

    fn update_resource(&self, id: ResouceId, item: Resource) -> Option<Resource> {
        todo!()
    }
}

impl DbMember for Worker {}

impl WorkerLike for Worker {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn health(&self) -> i32 {
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
    fn id(&self) -> i32 {
        self.id
    }

    fn price(&self) -> u32 {
        self.price()
    }

    fn attrition_rate(&self) -> u32 {
        self.attrition_rate()
    }

    fn efficiency(&self) -> ToolEfficiency {
        self.master().find_tool_master_by_id(self.master_id);
    }
}

impl ToolAsTable for DataBase {
    fn find_tool_by_id(&self, id: ToolId) -> Option<Tool> {
        self.tools.iter().find(|i| i.id == id).map(|i| i.clone())
    }

    fn update_tool(&self, id: ToolId, item: Tool) -> Option<Tool> {
        todo!()
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
