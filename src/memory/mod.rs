mod memory;
mod serde_memory;
mod task;
mod task_id;

use crate::actor::{CreepMemory, CreepSpawnTask, RoomMemory};
use screeps::RoomName;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
pub use memory::MemoryAccessor;

pub struct Memory(RefCell<MemoryInner>);
pub struct Tasks(RefCell<Vec<Task>>);
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TaskId(pub(super) usize);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "t")]
pub enum Task {
    NoTask,
    CreepSpawn(CreepSpawnTask),
}

impl Memory {
    fn with<TR>(&self, f: impl FnOnce(&mut MemoryInner) -> TR) -> TR {
        let memory = &mut *self.0.borrow_mut();
        f(memory)
    }
}

#[derive(Serialize, Deserialize, Default)]
struct MemoryInner {
    pub rooms: HashMap<RoomName, RoomMemory>,
    pub spawns: HashMap<String, ()>,
    pub creeps: HashMap<String, CreepMemory>,
    pub flags: HashMap<String, ()>,
}