use crate::actor::CreepSpawnTask;
use crate::common::{
    EnumDispatcher, EnumDowncast, IdManager, enum_downcast, hash_map, hash_map_key,
};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Tasks {
    id_manager: IdManager<TaskId>,
    values: HashMap<TaskId, Task>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TaskMemory {
    #[serde(default)]
    id_manager: IdManager<TaskId>,
    #[serde(flatten)]
    values: HashMap<String, Task>,
}

impl Tasks {
    pub fn from_memory(memory: &TaskMemory) -> Self {
        let values = hash_map_key(
            &memory.values,
            |task_name| TaskId::from(task_name.as_str()),
            |_, task| Some(task.clone()),
        );
        Tasks {
            id_manager: memory.id_manager.clone(),
            values,
        }
    }

    pub fn store_memory(&self, memory: &mut TaskMemory) {
        let values = hash_map(&self.values, String::from, |task| task.clone());
        *memory = TaskMemory {
            id_manager: self.id_manager.clone(),
            values,
        }
    }

    pub fn add_task(&mut self, f: impl FnOnce(TaskId) -> Task) -> Result<TaskId> {
        let id = self.id_manager.alloc_id();
        self.values.insert(id, f(id));
        Ok(id)
    }

    pub fn remove_task(&mut self, id: TaskId) -> Result<Task> {
        let task = self
            .values
            .remove(&id)
            .ok_or_else(|| anyhow!("invalid task id"))?;
        Ok(task)
    }

    pub fn iter_mut<'a, T: EnumDowncast<Task> + 'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut T> {
        self.values
            .values_mut()
            .filter_map(|task| task.downcast_mut())
    }

    pub fn iter<'a, T: EnumDowncast<Task> + 'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.values.values().filter_map(|task| task.downcast_ref())
    }

    pub fn get<T: EnumDowncast<Task>>(&self, id: TaskId) -> Result<&T> {
        self.values
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("invalid task id"))
            .and_then(|task| {
                task.downcast_ref()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }

    pub fn get_mut<T: EnumDowncast<Task>>(&mut self, id: TaskId) -> Result<&mut T> {
        self.values
            .get_mut(&id)
            .ok_or_else(|| anyhow::anyhow!("invalid task id"))
            .and_then(|task| {
                task.downcast_mut()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TaskId(usize);

impl From<TaskId> for String {
    fn from(value: TaskId) -> Self {
        format!("T{:04}", value.0)
    }
}

impl From<&str> for TaskId {
    fn from(s: &str) -> Self {
        let s = s.strip_prefix('T').expect("invalid task id");
        let id: usize = s.parse().unwrap();
        TaskId(id)
    }
}

impl From<usize> for TaskId {
    fn from(value: usize) -> Self {
        TaskId(value)
    }
}

impl From<TaskId> for usize {
    fn from(value: TaskId) -> Self {
        value.0
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "t")]
pub enum Task {
    CreepSpawn(CreepSpawnTask),
}

impl EnumDispatcher for Task {}
enum_downcast!(Task, CreepSpawn, CreepSpawnTask);
