use crate::task::{DownCast, TaskTrait};
use crate::task::{Task, TaskId};
use anyhow::anyhow;
use std::collections::{BTreeSet, HashMap};
use std::mem::swap;

#[derive(Default, Debug)]
pub struct Tasks {
    tasks: HashMap<TaskId, Task>,
    available_ids: BTreeSet<TaskId>,
    max_id: TaskId,
}

impl Tasks {
    pub fn get<T: DownCast>(&self, id: TaskId) -> anyhow::Result<&T> {
        let task = self.tasks.get(&id).ok_or(anyhow!("missing task"))?;
        DownCast::cast(task).ok_or(anyhow!("task type not match"))
    }

    pub fn get_mut<T: DownCast>(&mut self, id: TaskId) -> anyhow::Result<&mut T> {
        let task = self.tasks.get_mut(&id).ok_or(anyhow!("missing task"))?;
        DownCast::cast_mut(task).ok_or(anyhow!("task type not match"))
    }

    pub(super) fn insert(&mut self, task: Task) {
        let id = TaskTrait::task_id(&task);
        self.tasks.insert(id, task);
    }

    pub(super) fn available_id(&mut self) -> TaskId {
        match self.available_ids.pop_first() {
            Some(id) => id,
            None => {
                let mut id = self.max_id.next();
                swap(&mut id, &mut self.max_id);
                id
            }
        }
    }
}
