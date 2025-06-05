use crate::actor::{Actor, CreepClass, CreepSpawnTask};
use crate::context::Context;
use crate::memory::{MemoryAccessor, TaskId};
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

    fn plan(&self, ctx: &Context) -> Result<()> {
        info!("room planning");
        let mut memory = ctx.memory().load(self);
        if memory.spawn_task.is_some() {
            return Ok(());
        }
        let task_id = ctx
            .tasks()
            .add(CreepSpawnTask::new_task(self.name(), CreepClass::Worker));
        memory.spawn_task.replace(task_id);
        ctx.memory().store(self, memory);
        Ok(())
    }

    fn assign(&self, _: &Context) -> Result<()> {
        Ok(())
    }

    fn run(&self, _: &Context) -> Result<()> {
        Ok(())
    }
}
