use crate::actor::CreepSpawnTask;
use crate::common::{
    EnumDispatcher, EnumDowncast, IdManager, NewIdResult, Tombstone, enum_downcast,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::format;
use std::mem;

pub struct Tasks {
    pointers: Option<(usize, usize)>,
    values: Vec<Task>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TaskMemory {
    pointers: Option<(usize, usize)>, // head , tail
    #[serde(flatten)]
    values: HashMap<String, Task>,
}

impl Tasks {
    pub fn from_memory(memory: &TaskMemory) -> Self {
        let length = memory.values.len();
        let mut values = Vec::with_capacity(length);
        for id in 0..length {
            let key = format!("T{:04}", id);
            values.push(memory.values.get(&key).expect("missing middle").clone());
        }
        Tasks {
            pointers: memory.pointers,
            values,
        }
    }

    pub fn store_memory(&self, memory: &mut TaskMemory) {
        let values = self
            .values
            .iter()
            .enumerate()
            .map(|(id, task)| (format!("T{:04}", id), task.clone()))
            .collect();
        *memory = TaskMemory {
            pointers: self.pointers,
            values,
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<TaskId> {
        match self.alloc_id() {
            NewIdResult::NewId => {
                let id = self.values.len();
                self.values.push(task);
                Ok(TaskId(id))
            }
            NewIdResult::ReusedId(id) => {
                self.values[id] = task;
                Ok(TaskId(id))
            }
        }
    }

    pub fn remove_task(&mut self, TaskId(task_id): TaskId) -> Result<Task> {
        let mut task = Task::Tombstone(self.free_id(task_id));
        mem::swap(&mut task, &mut self.values[task_id]);
        Ok(task)
    }

    pub fn iter_mut<'a, T: EnumDowncast<Task> + 'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (TaskId, &'a mut T)> {
        self.values
            .iter_mut()
            .enumerate()
            .filter_map(|(task_id, task)| task.downcast_mut().map(|t| (TaskId(task_id), t)))
    }

    pub fn iter<'a, T: EnumDowncast<Task> + 'a>(&'a self) -> impl Iterator<Item = (TaskId, &'a T)> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(task_id, task)| task.downcast_ref().map(|t| (TaskId(task_id), t)))
    }

    pub fn get<T: EnumDowncast<Task>>(&self, id: TaskId) -> anyhow::Result<&T> {
        self.values
            .get(id.0)
            .ok_or_else(|| anyhow::anyhow!("task id out of bounds"))
            .and_then(|task| {
                task.downcast_ref()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }

    pub fn get_mut<T: EnumDowncast<Task>>(&mut self, id: TaskId) -> anyhow::Result<&mut T> {
        self.values
            .get_mut(id.0)
            .ok_or_else(|| anyhow::anyhow!("task id out of bounds"))
            .and_then(|task| {
                task.downcast_mut()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }
}

impl IdManager for Tasks {
    fn get_pointer(&mut self) -> &mut Option<(usize, usize)> {
        &mut self.pointers
    }

    fn get_tombstone(&mut self, index: usize) -> &mut Tombstone {
        match &mut self.values[index] {
            Task::Tombstone(tombstone) => tombstone,
            _ => panic!("Expected NoTask at index {}", index),
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TaskId(usize);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(tag = "t")]
pub enum Task {
    Tombstone(Tombstone),
    CreepSpawn(CreepSpawnTask),
}

impl EnumDispatcher for Task {}
enum_downcast!(Task, Tombstone, Tombstone);
enum_downcast!(Task, CreepSpawn, CreepSpawnTask);
