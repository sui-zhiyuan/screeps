use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone,
)]
pub struct TaskId(u64);

impl TaskId {
    pub fn next(&self) -> TaskId {
        TaskId(self.0 + 1)
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}