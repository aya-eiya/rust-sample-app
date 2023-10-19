extern crate rand;

use std::borrow::BorrowMut;

use crate::domains::{
    gold::Gold,
    resource::{ResourceLike, WorkStats},
    tool::ToolEfficiency,
    tool::ToolLike,
    worker::{WorkerEfficiency, WorkerLike},
};
use once_cell::sync::Lazy;

type ResouceId = u32;
type WorkerId = u32;
type ToolId = u32;
type ToolMasterId = u32;

#[derive(Debug, Clone)]
pub struct Resource {
    id: ResouceId,
    deposit_amount: u32,
}

#[derive(Debug, Clone)]
pub struct ResourceBody {
    pub id: ResouceId,
    pub deposit_amount: u32,
}

#[derive(Debug, Clone)]
pub struct Worker {
    id: WorkerId,
    name: String,
    health: u32,
    tool_id: Option<ToolId>,
}

#[derive(Debug, Clone)]
pub struct WorkerBody {
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
    pub price: u32,
    pub efficiency: ToolEfficiency,
}

#[derive(Debug, Clone)]
pub struct Master {
    tools: Vec<ToolMaster>,
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

struct InMemoryDBContext {
    db: DataBase,
    master: Master,
}

impl InMemoryDBContext {
    fn new() -> InMemoryDBContext {
        InMemoryDBContext {
            db: DataBase::new(),
            master: Master::new(),
        }
    }
}

static DATA_BASE: Lazy<InMemoryDBContext> = Lazy::new(|| InMemoryDBContext::new());

trait DbMember {
    fn db(&self) -> DataBase {
        DATA_BASE.db.to_owned()
    }
    fn master(&self) -> Master {
        DATA_BASE.master.to_owned()
    }
}

impl DbMember for InMemoryDBContext {}

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
    fn create_resource(&mut self, item: ResourceBody) -> Resource;
    fn find_resource_by_id(&self, id: ResouceId) -> Option<Resource>;
    fn update_resource(&mut self, id: ResouceId, item: Resource) -> Option<Resource>;
}

pub trait WorkerAsTable {
    fn create_worker(&mut self, item: WorkerBody) -> Worker;
    fn find_worker_by_id(&self, id: WorkerId) -> Option<Worker>;
    fn update_worker(&mut self, id: WorkerId, item: Worker) -> Option<Worker>;
}

pub trait ToolAsTable {
    fn create_tool(&mut self, item: ToolBody) -> Tool;
    fn find_tool_by_id(&self, id: ToolId) -> Option<Tool>;
    fn update_tool(&mut self, id: ToolId, item: Tool) -> Option<Tool>;
}

pub trait ToolMasterAsTable {
    fn create_tool_master(&mut self, item: ToolMasterBody) -> ToolMaster;
    fn find_tool_master_by_id(&self, id: ToolMasterId) -> Option<ToolMaster>;
}

impl ResourcesAsTable for DataBase {
    fn find_resource_by_id(&self, id: ResouceId) -> Option<Resource> {
        self.resources
            .iter()
            .find(|i| i.id == id)
            .map(|i| i.clone())
    }

    fn update_resource(&mut self, id: ResouceId, item: Resource) -> Option<Resource> {
        let mut rs = vec![];
        rs.append(self.resources.borrow_mut());
        if let Ok(found) = self.resources.binary_search_by(|i| i.id.cmp(&id)) {
            rs[found] = item.clone();
            return Some(item);
        }
        self.resources = rs;
        return None;
    }

    fn create_resource(&mut self, item: ResourceBody) -> Resource {
        loop {
            let nid: u32 = rand::random();
            if self.find_resource_by_id(nid).is_none() {
                let item = Resource {
                    id: nid,
                    deposit_amount: item.deposit_amount,
                };
                self.resources.push(item.to_owned());
                return item;
            }
        }
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

    fn update_worker(&mut self, id: WorkerId, item: Worker) -> Option<Worker> {
        let mut rs = vec![];
        rs.append(self.workers.borrow_mut());
        if let Ok(found) = self.workers.binary_search_by(|i| i.id.cmp(&id)) {
            rs[found] = item.clone();
            return Some(item);
        }
        self.workers = rs;
        return None;
    }

    fn create_worker(&mut self, item: WorkerBody) -> Worker {
        loop {
            let nid: u32 = rand::random();
            if self.find_worker_by_id(nid).is_none() {
                let item = Worker {
                    id: nid,
                    health: item.health,
                    name: item.name,
                    tool_id: item.tool_id,
                };
                self.workers.push(item.to_owned());
                return item;
            }
        }
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
        let mut rs = vec![];
        rs.append(self.tools.borrow_mut());
        if let Ok(found) = self.tools.binary_search_by(|i| i.id.cmp(&id)) {
            rs[found] = item.clone();
            return Some(item);
        }
        self.tools = rs;
        return None;
    }

    fn create_tool(&mut self, item: ToolBody) -> Tool {
        loop {
            let nid: u32 = rand::random();
            if self.find_tool_by_id(nid).is_none() {
                let item = Tool {
                    id: nid,
                    attrition_rate: item.attrition_rate,
                    master_id: item.master_id,
                };
                self.tools.push(item.to_owned());
                return item;
            }
        }
    }
}

impl ToolMasterAsTable for Master {
    fn find_tool_master_by_id(&self, id: ToolMasterId) -> Option<ToolMaster> {
        self.tools.iter().find(|i| i.id == id).map(|i| i.clone())
    }

    fn create_tool_master(&mut self, item: ToolMasterBody) -> ToolMaster {
        loop {
            let nid: u32 = rand::random();
            if self.find_tool_master_by_id(nid).is_none() {
                let item = ToolMaster {
                    id: nid,
                    efficiency: item.efficiency,
                    name: item.name,
                    price: item.price,
                };
                self.tools.push(item.to_owned());
                return item;
            }
        }
    }
}

#[cfg(test)]
#[test]
fn it_works() {
    let m = &mut DATA_BASE.master();
    let d = &mut DATA_BASE.db();
    let tm1 = m.create_tool_master(ToolMasterBody {
        efficiency: 1,
        name: String::from("tool_0"),
        price: 100,
    });
    let tm2_opt = m.find_tool_master_by_id(tm1.id);
    if let Some(tm2) = tm2_opt {
        assert_eq!(tm1.id, tm2.id);
        assert_eq!(tm1.name, tm2.name);
        assert_eq!(tm1.price, tm2.price);
    } else {
        panic!("master not found");
    }
    let t1 = d.create_tool(ToolBody {
        attrition_rate: 100,
        master_id: tm1.id,
    });
    assert_eq!(t1.price(), tm1.price)
}
