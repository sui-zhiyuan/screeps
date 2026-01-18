use crate::actor::CreepSpawnTask;
use crate::common::{EnumDispatcher, EnumDowncast, enum_downcast};
use anyhow::{Result, anyhow, bail, ensure};
use screeps::game;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::mem::swap;

pub struct Tasks {
    memory: TaskMemory,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TaskMemory {
    tasks: Vec<Task>,
    pointers: Option<(usize, usize)>, // head , tail
}

impl Tasks {
    const FREE_COOLDOWN_TIME: u32 = 300; // ticks

    pub fn from_memory(memory: &TaskMemory) -> Self {
        Tasks {
            memory: memory.clone(),
        }
    }

    pub fn store_memory(&self, memory: &mut TaskMemory) {
        *memory = self.memory.clone();
    }

    pub fn add_task(&mut self, task: Task) -> Result<TaskId> {
        let curr_time = game::time();
        let Some((head, tail)) = &mut self.memory.pointers else {
            let id = self.memory.tasks.len();
            self.memory.tasks.push(task);
            return Ok(TaskId(id));
        };

        let Task::NoTask(no_task) = &self.memory.tasks[*head] else {
            panic!("Expected NoTask at free index");
        };

        if no_task.free_time + Self::FREE_COOLDOWN_TIME <= curr_time {
            let id = self.memory.tasks.len();
            self.memory.tasks.push(task);
            return Ok(TaskId(id));
        }

        let free_index = *head;
        if *head == *tail {
            self.memory.pointers = None;
        } else {
            *head = no_task
                .next_free
                .ok_or(anyhow!("Invalid next_free pointer"))?;
        }

        self.memory.tasks[free_index] = task;
        Ok(TaskId(free_index))
    }

    pub fn remove_task(&mut self, TaskId(task_id): TaskId) -> Result<Task> {
        ensure!(task_id < self.memory.tasks.len(), "task id out of bounds");
        ensure!(
            !matches!(self.memory.tasks[task_id], Task::NoTask(_)),
            "task already removed"
        );

        let mut task = Task::NoTask(NoTask {
            free_time: game::time(),
            next_free: None,
        });
        swap(&mut task, &mut self.memory.tasks[task_id]);

        let Some((_, tail)) = &mut self.memory.pointers else {
            self.memory.pointers = Some((task_id, task_id));
            return Ok(task);
        };

        let Task::NoTask(tail_task) = &mut self.memory.tasks[*tail] else {
            bail!("Tail pointer corrupted")
        };
        ensure!(tail_task.next_free.is_none(), "corrupted free list");
        tail_task.next_free = Some(task_id);
        *tail = task_id;

        Ok(task)
    }

    pub fn iter_mut<'a, T: EnumDowncast<Task> + 'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (TaskId, &'a mut T)> {
        self.memory
            .tasks
            .iter_mut()
            .enumerate()
            .filter_map(|(task_id, task)| task.downcast_mut().map(|t| (TaskId(task_id), t)))
    }

    pub fn iter<'a, T: EnumDowncast<Task> + 'a>(&'a self) -> impl Iterator<Item = (TaskId, &'a T)> {
        self.memory
            .tasks
            .iter()
            .enumerate()
            .filter_map(|(task_id, task)| task.downcast_ref().map(|t| (TaskId(task_id), t)))
    }

    pub fn get<T: EnumDowncast<Task>>(&self, id: TaskId) -> anyhow::Result<&T> {
        self.memory
            .tasks
            .get(id.0)
            .ok_or_else(|| anyhow::anyhow!("task id out of bounds"))
            .and_then(|task| {
                task.downcast_ref()
                    .ok_or_else(|| anyhow::anyhow!("task is not of the expected type"))
            })
    }

    pub fn get_mut<T: EnumDowncast<Task>>(&mut self, id: TaskId) -> anyhow::Result<&mut T> {
        self.memory
            .tasks
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
pub struct NoTask {
    free_time: u32,
    next_free: Option<usize>,
}

impl Display for NoTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoTask")
    }
}

impl EnumDispatcher for Task {}
enum_downcast!(Task, NoTask, NoTask);
enum_downcast!(Task, CreepSpawn, CreepSpawnTask);
