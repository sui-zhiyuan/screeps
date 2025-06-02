use crate::actor::{Actor, CreepClass, CreepSpawnTask};
use crate::memory::{Memory, MemoryAccessor};
use crate::task::{TaskId, Tasks};
use anyhow::Result;
use screeps::Room;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RoomMemory {
    spawn_task: Option<TaskId>,
}

impl Actor for Room {
    fn name(&self) -> String {
        self.name().to_string()
    }

    fn plan(&self) -> Result<()> {
        info!("room planning");
        let mut memory = Memory::load(self)?;
        if memory.spawn_task.is_some() {
            return Ok(());
        }
        let task_id = Tasks::add(CreepSpawnTask::new_task(self.name(), CreepClass::Worker))?;
        memory.spawn_task.replace(task_id);
        Memory::store(self, memory)?;
        Ok(())
    }

    fn assign(&self) -> Result<()> {
        Ok(())
    }

    fn run(&self) -> Result<()> {
        Ok(())
    }
}
