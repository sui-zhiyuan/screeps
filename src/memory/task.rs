use crate::memory::{DownCast, Task, TaskId, Tasks};

impl Tasks {
    pub fn add(&self, task: Task) -> TaskId {
        let tasks = &mut *self.0.borrow_mut();
        tasks.push(task);
        // todo find NoTask
        TaskId(tasks.len() - 1)
    }

    pub fn with<T, TR>(&self, task_id: TaskId, f: impl FnOnce(&mut T) -> TR) -> TR
    where
        Task: DownCast<T>,
    {
        let tasks = &mut *self.0.borrow_mut();
        let task = &mut tasks[task_id.0];
        f(task.cast())
    }

    pub fn try_with<T, TR>(&self, task_id: TaskId, f: impl FnOnce(Option<&mut T>) -> TR) -> TR
    where
        Task: DownCast<T>,
    {
        let tasks = &mut *self.0.borrow_mut();
        let task = &mut tasks[task_id.0];
        f(task.try_cast())
    }
}
