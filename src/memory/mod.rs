mod memory;
mod serde_memory;
mod task;
mod task_id;

use crate::actor::{CreepMemory, CreepSpawnTask, RoomMemory, SpawnMemory};
// pub use memory::MemoryAccessor;
use screeps::RoomName;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use crate::task::Task;
// impl Memory {
//     fn with<TR>(&self, f: impl FnOnce(&mut MemoryInner) -> TR) -> TR {
//         let memory = &mut *self.0.borrow_mut();
//         f(memory)
//     }
// }

#[derive(Serialize, Deserialize, Default)]
pub struct Memory {
    #[serde(default)]
    pub rooms: HashMap<String, RoomMemory>,
    #[serde(default)]
    pub spawns: HashMap<String, SpawnMemory>,
    #[serde(default)]
    pub creeps: HashMap<String, CreepMemory>,
    #[serde(default)]
    pub flags: HashMap<String, ()>,
    #[serde(default)]
    pub tasks: Vec<Task>,
}
//
// pub trait DownCast<T> {
//     fn try_cast(&mut self) -> Option<&mut T>;
//
//     fn cast(&mut self) -> &mut T {
//         self.try_cast().expect("fail to cast")
//     }
// }
//
// impl<T> DownCast<T> for T {
//     fn try_cast(&mut self) -> Option<&mut T> {
//         Some(self)
//     }
//
//     fn cast(&mut self) -> &mut T {
//         self
//     }
// }

// impl DownCast<CreepSpawnTask> for Task {
//     fn try_cast(&mut self) -> Option<&mut CreepSpawnTask> {
//         match self {
//             Task::CreepSpawn(task) => Some(task),
//             _ => None,
//         }
//     }
// }
