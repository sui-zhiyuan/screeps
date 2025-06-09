use crate::memory::{Memory, Tasks};
use anyhow::Result;

#[derive(Default)]
pub struct Context {
    memory: Memory,
    tasks: Tasks,
}

impl Context {
    pub fn new() -> Result<Context> {
        let (memory, tasks) = Memory::load_from_raw()?;
        Ok(Context { memory, tasks })
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn tasks(&self) -> &Tasks {
        &self.tasks
    }

    pub fn store(&self) -> Result<()> {
        Memory::store_to_raw(self, &self.memory, &self.tasks)
    }
}
