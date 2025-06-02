mod serde_id;
mod serde_task;

use crate::actor::CreepSpawnTask;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_id::TaskIdParse;
pub use serde_task::TaskSerializePhantom;
use std::sync::{LazyLock, Mutex, MutexGuard};

static TASKS: LazyLock<Mutex<Option<Tasks>>> = LazyLock::new(|| Mutex::new(None));

pub struct Tasks(Vec<Task>);

impl Tasks {
    pub fn with<TR>(f: impl FnOnce(&mut Tasks) -> Result<TR>) -> Result<TR> {
        let mut guard = Self::get_guard()?;
        let v = guard
            .as_mut()
            .ok_or_else(|| anyhow!("tasks not initialized"))?;
        f(v)
    }

    pub fn add(task: Task) -> Result<TaskId> {
        Self::with(|v| {
            v.0.push(task);
            Ok(TaskId(v.0.len() - 1))
        })
    }

    fn get_guard() -> Result<MutexGuard<'static, Option<Tasks>>> {
        TASKS.lock().map_err(|e| anyhow!("load task error: {}", e))
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(from = "TaskIdParse")]
pub struct TaskId(usize);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "t")]
pub enum Task {
    NoTask,
    CreepSpawn(CreepSpawnTask),
}
