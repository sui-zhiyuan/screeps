use crate::task::{Task, TaskId, Tasks};
use serde::{Deserialize, Serialize, Serializer};

impl From<TaskIdParse> for TaskId {
    fn from(value: TaskIdParse) -> Self {
        TaskId(value.id)
    }
}

#[derive(Serialize)]
struct TaskIdDisplay<'a> {
    id: usize,
    task: &'a Task,
}

#[derive(Deserialize)]
pub struct TaskIdParse {
    id: usize,
}

impl Serialize for TaskId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let result = Tasks::with(|t| {
            let task = &t.0[self.0];
            let td = TaskIdDisplay { id: self.0, task };
            Ok(td.serialize(serializer))
        });
        result.map_err(|e| serde::ser::Error::custom(format!("{}", e)))?
    }
}
