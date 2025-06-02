mod serialize;

use crate::actor::CreepSpawnTask;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
pub use serialize::TaskSerializePhantom;
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

    pub fn add(task: Task) -> Result<TaskId> {
        Self::with(|v| {
            v.0.push(task);
            TaskId(v.0.len() - 1)
        })
    }

    fn get_guard() -> Result<MutexGuard<'static, Option<Tasks>>> {
        TASKS.lock().map_err(|e| anyhow!("load task error: {}", e))
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TaskId(usize);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "t")]
pub enum Task {
    NoTask,
    CreepSpawn(CreepSpawnTask),
}
