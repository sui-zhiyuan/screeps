use crate::memory::{Task, TaskId, Tasks};

impl Tasks {
    pub fn with<TR>(&self, f: impl FnOnce(&mut Vec<Task>) -> TR) -> TR {
        let tasks = &mut *self.0.borrow_mut();
        f(tasks)
    }

    pub fn add(&self, task: Task) -> TaskId {
        self.with(|v| {
            v.push(task);
            // todo find NoTask
            TaskId(v.len() - 1)
        })
    }
}
