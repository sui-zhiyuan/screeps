use crate::memory::{Memory, MemoryInner, Task, Tasks};
use anyhow::Result;
use js_sys::JsString;
use screeps::{SharedCreepProperties, game, raw_memory};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;

impl Memory {
    pub fn load_from_raw() -> Result<(Memory, Tasks)> {
        let js_memory = raw_memory::get();
        let json_value: String = js_memory.into();
        let MemoryDeserialize { memory, tasks } = serde_json::from_str(&json_value)?;
        let memory = Memory(RefCell::new(memory));
        let tasks = Tasks(RefCell::new(tasks));
        Ok((memory, tasks))
    }

    pub fn store_to_raw(memory: &Memory, tasks: &Tasks) -> Result<()> {
        let memory = &mut *memory.0.borrow_mut();
        memory.clean_up_memory();
        let tasks = &*tasks.0.borrow();
        let full_memory = MemorySerialize { memory, tasks };
        let js_value = serde_json::to_string(&full_memory)?;
        let js_memory = JsString::from(js_value);
        raw_memory::set(&js_memory);
        Ok(())
    }
}

impl MemoryInner {
    fn clean_up_memory(&mut self) {
        let creeps = game::creeps()
            .values()
            .map(|c| c.name())
            .collect::<HashSet<_>>();
        self.creeps.retain(|name, _| creeps.contains(name));
    }
}

#[derive(Deserialize)]
struct MemoryDeserialize {
    #[serde(flatten)]
    memory: MemoryInner,
    #[serde(default)]
    tasks: Vec<Task>,
}

#[derive(Serialize)]
struct MemorySerialize<'a> {
    #[serde(flatten)]
    memory: &'a MemoryInner,
    tasks: &'a Vec<Task>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::{CreepClass, CreepSpawnTask};
    
    use screeps::RoomName;

    #[test]
    fn test_initialize() {
        let value = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{}}"#;
        let MemoryDeserialize { memory, mut tasks } = serde_json::from_str(&value).unwrap();

        assert_eq!(tasks.len(), 0);
        tasks.push(Task::NoTask);

        let full_memory = MemorySerialize {
            memory: &memory,
            tasks: &tasks,
        };
        let json_memory = serde_json::to_string(&full_memory).unwrap();
        let result = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{},"tasks":[{"t":"NoTask"}]}"#;
        assert_eq!(result, json_memory);
    }

    #[test]
    fn test_serialize() {
        let value = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{},"tasks":[{"t":"NoTask"}]}"#;
        let MemoryDeserialize { memory, mut tasks } = serde_json::from_str(&value).unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(Task::NoTask, tasks[0]);

        tasks.pop();
        tasks.push(CreepSpawnTask::new_task(
            RoomName::new("E1N3").unwrap(),
            CreepClass::Worker,
        ));

        let full_memory = MemorySerialize {
            memory: &memory,
            tasks: &tasks,
        };
        let json_memory = serde_json::to_string(&full_memory).unwrap();
        let result = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{},"tasks":[{"t":"CreepSpawn","room":"E1N3","creep_class":"Worker"}]}"#;
        assert_eq!(result, json_memory);
    }
}
