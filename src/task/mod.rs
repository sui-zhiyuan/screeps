use crate::actor::CreepSpawnTask;
use crate::memory::Memory;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::common::EnumDowncast;

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

    pub fn get<T: EnumDowncast<Task>>(&self, id: TaskId) -> Result<&T> {
        self.tasks
            .get(id.0)
            .ok_or_else(|| anyhow::anyhow!("task id out of bounds"))
            .and_then(|task| {
                task.downcast_ref()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }

    pub fn get_mut<T: EnumDowncast<Task>>(&mut self, id: TaskId) -> Result<&mut T> {
        self.tasks
            .get_mut(id.0)
            .ok_or_else(|| anyhow::anyhow!("task id out of bounds"))
            .and_then(|task| {
                task.downcast_mut()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }
}

// trait TasksDowncast<T: TaskTrait> {
//     fn get(&self, id: TaskId) -> Result<&T>;
//     fn get_mut(&mut self, id: TaskId) -> Result<&mut T>;
//
//     type Iter<'a>: Iterator<Item = (TaskId, &'a T)>
//     where
//         T: 'a,
//         Self: 'a;
//
//     type IterMut<'a>: Iterator<Item = (TaskId, &'a mut T)>
//     where
//         T: 'a,
//         Self: 'a;
//
//     fn iter(&self) -> Self::Iter<'_>;
//     fn iter_mut(&mut self) -> Self::IterMut<'_>;
// }
//
// impl TasksDowncast<CreepSpawnTask> for Tasks {
//     fn get(&self, id: TaskId) -> Result<&CreepSpawnTask> {
//         match &self.tasks[id.0] {
//             Task::CreepSpawn(task) => Ok(task),
//             _ => Err(anyhow::anyhow!("task is not CreepSpawnTask")),
//         }
//     }
//
//     fn get_mut(&mut self, id: TaskId) -> Result<&mut CreepSpawnTask> {
//         match &mut self.tasks[id.0] {
//             Task::CreepSpawn(task) => Ok(task),
//             _ => Err(anyhow::anyhow!("task is not CreepSpawnTask")),
//         }
//     }
//
//     type Iter<'a> = impl Iterator<Item = (TaskId, &'a CreepSpawnTask)> + 'a;
//     type IterMut<'a> = impl Iterator<Item = (TaskId, &'a mut CreepSpawnTask)> + 'a;
//
//     fn iter(&self) -> Self::Iter<'_> {
//         self.tasks.iter().enumerate().filter_map(|(i, t)| match t {
//             Task::CreepSpawn(task) => Some((TaskId(i), task)),
//             _ => None,
//         })
//     }
//
//     fn iter_mut(&mut self) -> impl Iterator<Item = (TaskId, &'_ mut CreepSpawnTask)> {
//         self.tasks
//             .iter_mut()
//             .enumerate()
//             .filter_map(|(i, t)| match t {
//                 Task::CreepSpawn(task) => Some((TaskId(i), task)),
//                 _ => None,
//             })
//     }
// }

// #[derive(Default)]
// pub struct Memory(RefCell<MemoryInner>);
// #[derive(Default)]
// pub struct Tasks(RefCell<Vec<Task>>);
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TaskId(usize);

impl Task {
    fn downcast<T: EnumDowncast<Self>>(self) -> Option<T> {
        T::enum_downcast(self)
    }
    fn downcast_ref<T: EnumDowncast<Self>>(&self) -> Option<&T> {
        T::enum_downcast_ref(self)
    }
    fn downcast_mut<T: EnumDowncast<Self>>(&mut self) -> Option<&mut T> {
        T::enum_downcast_mut(self)
    }
}

impl EnumDowncast<Task> for NoTask {
    fn enum_downcast(from: Task) -> Option<Self> {
        match from {
            Task::NoTask(item) => Some(item),
            _ => None,
        }
    }
    fn enum_downcast_ref(from: &Task) -> Option<&Self> {
        match from {
            Task::NoTask(item) => Some(item),
            _ => None,
        }
    }
    fn enum_downcast_mut(from: &mut Task) -> Option<&mut Self> {
        match from {
            Task::NoTask(item) => Some(item),
            _ => None,
        }
    }
}

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
