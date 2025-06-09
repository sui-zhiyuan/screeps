mod memory;
mod serde_memory;
mod task;
mod task_id;

use crate::actor::{CreepMemory, CreepSpawnTask, RoomMemory};
pub use memory::MemoryAccessor;
use screeps::RoomName;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Default)]
pub struct Memory(RefCell<MemoryInner>);
#[derive(Default)]
pub struct Tasks(RefCell<Vec<Task>>);
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TaskId(usize);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "t")]
pub enum Task {
    NoTask(NoTask),
    CreepSpawn(CreepSpawnTask),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NoTask {}

impl Display for NoTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoTask")
    }
}

impl Memory {
    fn with<TR>(&self, f: impl FnOnce(&mut MemoryInner) -> TR) -> TR {
        let memory = &mut *self.0.borrow_mut();
        f(memory)
    }
}

#[derive(Deserialize, Default)]
struct MemoryInner {
    rooms: HashMap<RoomName, RoomMemory>,
    spawns: HashMap<String, ()>,
    creeps: HashMap<String, CreepMemory>,
    flags: HashMap<String, ()>,
}

pub trait DownCast<T> {
    fn try_cast(&mut self) -> Option<&mut T>;

    fn cast(&mut self) -> &mut T {
        self.try_cast().expect("fail to cast")
    }
}

impl<T> DownCast<T> for T {
    fn try_cast(&mut self) -> Option<&mut T> {
        Some(self)
    }

    fn cast(&mut self) -> &mut T {
        self
    }
}

impl DownCast<CreepSpawnTask> for Task {
    fn try_cast(&mut self) -> Option<&mut CreepSpawnTask> {
        match self {
            Task::CreepSpawn(task) => Some(task),
            _ => None,
        }
    }
}
