use crate::actor::{CreepMemory, RoomMemorySerialize};
use crate::context::Context;
use crate::memory::{Memory, MemoryInner, Task, Tasks};
use anyhow::Result;
use js_sys::JsString;
use screeps::{RoomName, SharedCreepProperties, game, raw_memory};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

impl Memory {
    pub fn load_from_raw() -> Result<(Memory, Tasks)> {
        let js_memory = raw_memory::get();
        let json_value: String = js_memory.into();
        let MemoryDeserialize { memory, tasks } = serde_json::from_str(&json_value)?;
        let memory = Memory(RefCell::new(memory));
        let tasks = Tasks(RefCell::new(tasks));
        Ok((memory, tasks))
    }

    pub fn store_to_raw(ctx: &Context, memory: &Memory, tasks: &Tasks) -> Result<()> {
        let memory = &mut *memory.0.borrow_mut();
        memory.clean_up_memory();
        let memory = &memory.to_serialize(ctx);
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

impl MemoryInner {
    fn to_serialize<'a>(&'a self, ctx: &'a Context) -> MemoryInnerSerialize {
        MemoryInnerSerialize {
            rooms: self
                .rooms
                .iter()
                .map(|(&name, value)| (name, value.to_serialize(ctx)))
                .collect(),
            spawns: self.spawns.clone(),
            creeps: self.creeps.clone(),
            flags: self.flags.clone(),
        }
    }
}

#[derive(Serialize)]
struct MemorySerialize<'a> {
    #[serde(flatten)]
    memory: &'a MemoryInnerSerialize,
    tasks: &'a Vec<Task>,
}

#[derive(Serialize)]
struct MemoryInnerSerialize {
    rooms: HashMap<RoomName, RoomMemorySerialize>,
    spawns: HashMap<String, ()>,
    creeps: HashMap<String, CreepMemory>,
    flags: HashMap<String, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::{CreepClass, CreepSpawnTask};

    use crate::memory::NoTask;
    use screeps::RoomName;

    #[test]
    fn test_initialize() {
        let value = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{}}"#;
        let MemoryDeserialize { memory, mut tasks } = serde_json::from_str(&value).unwrap();

        assert_eq!(tasks.len(), 0);
        tasks.push(Task::NoTask(NoTask {}));

        let ctx = Context::default();
        let full_memory = MemorySerialize {
            memory: &memory.to_serialize(&ctx),
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
        assert_eq!(Task::NoTask(NoTask {}), tasks[0]);

        tasks.pop();
        tasks.push(CreepSpawnTask::new_task(
            RoomName::new("E1N3").unwrap(),
            CreepClass::Worker,
        ));

        let ctx = Context::default();
        let full_memory = MemorySerialize {
            memory: &memory.to_serialize(&ctx),
            tasks: &tasks,
        };
        let json_memory = serde_json::to_string(&full_memory).unwrap();
        let result = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{},"tasks":[{"t":"CreepSpawn","room":"E1N3","creep_class":"Worker"}]}"#;
        assert_eq!(result, json_memory);
    }
}
