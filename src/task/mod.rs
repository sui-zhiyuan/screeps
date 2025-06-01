mod serialize;

pub use serialize::TaskSerializePhantom;

use anyhow::{Result, anyhow};
use screeps::RoomName;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex, MutexGuard};

static TASKS: LazyLock<Mutex<Option<Tasks>>> = LazyLock::new(|| Mutex::new(None));

pub struct Tasks(Vec<Task>);

impl Tasks {
    pub fn with<TR>(f: impl FnOnce(&mut Tasks) -> TR) -> Result<TR> {
        let mut guard = Self::get_guard()?;
        let v = guard
            .as_mut()
            .ok_or_else(|| anyhow!("tasks not initialized"))?;
        Ok(f(v))
    }

    fn get_guard() -> Result<MutexGuard<'static, Option<Tasks>>> {
        TASKS.lock().map_err(|e| anyhow!("load task error: {}", e))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Task {
    NoTask,
    CreepSpawn(CreepSpawnTask),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CreepSpawnTask {
    room_name: RoomName,
}
