use crate::actor::CreepSpawnTask;
use crate::common::{EnumDispatcher, EnumDowncast, enum_downcast};
use crate::memory::Memory;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    pub fn from_memory(memory: &Memory) -> Self {
        Tasks {
            tasks: memory.tasks.clone(),
        }
    }

    pub fn store_memory(&self, memory: &mut Memory) {
        memory.tasks = self.tasks.clone();
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn iter_mut<'a, T: EnumDowncast<Task> + 'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (TaskId, &'a mut T)> {
        self.tasks
            .iter_mut()
            .enumerate()
            .filter_map(|(task_id, task)| task.downcast_mut().map(|t| (TaskId(task_id), t)))
    }

    pub fn iter<'a, T: EnumDowncast<Task> + 'a>(&'a self) -> impl Iterator<Item = (TaskId, &'a T)> {
        self.tasks
            .iter()
            .enumerate()
            .filter_map(|(task_id, task)| task.downcast_ref().map(|t| (TaskId(task_id), t)))
    }

    pub fn get<T: EnumDowncast<Task>>(&self, id: TaskId) -> anyhow::Result<&T> {
        self.tasks
            .get(id.0)
            .ok_or_else(|| anyhow::anyhow!("task id out of bounds"))
            .and_then(|task| {
                task.downcast_ref()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }

    pub fn get_mut<T: EnumDowncast<Task>>(&mut self, id: TaskId) -> anyhow::Result<&mut T> {
        self.tasks
            .get_mut(id.0)
            .ok_or_else(|| anyhow::anyhow!("task id out of bounds"))
            .and_then(|task| {
                task.downcast_mut()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TaskId(usize);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(tag = "t")]
pub enum Task {
    NoTask(NoTask),
    CreepSpawn(CreepSpawnTask),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct NoTask {}

impl Display for NoTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoTask")
    }
}

impl EnumDispatcher for Task {}
enum_downcast!(Task, NoTask, NoTask);
enum_downcast!(Task, CreepSpawn, CreepSpawnTask);
