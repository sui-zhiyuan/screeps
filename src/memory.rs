use crate::actor::{CreepMemory, SpawnMemory};
use crate::entity::Entities;
use anyhow::{Result, anyhow};
use js_sys::JsString;
use log::info;
use screeps::{SharedCreepProperties, raw_memory};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, MutexGuard};

static MEMORY: Mutex<Option<Memory>> = Mutex::new(None);

#[derive(Serialize, Deserialize, Default)]
pub struct Memory {
    pub rooms: HashMap<String, ()>,
    pub spawns: HashMap<String, SpawnMemory>,
    pub creeps: HashMap<String, CreepMemory>,
    pub flags: HashMap<String, ()>,
}

impl Memory {
    pub fn load() -> Result<(MutexGuard<'static, Option<Memory>>, Memory)> {
        let mut memory_lock = MEMORY
            .lock()
            .map_err(|e| anyhow!("memory lock err: {}", e))?;
        let memory = match memory_lock.take() {
            Some(memory) => memory,
            None => Self::load_memory()?,
        };

        Ok((memory_lock, memory))
    }

    pub fn store(
        mut self,
        mut lock: MutexGuard<Option<Memory>>,
        entities: &Entities,
    ) -> Result<()> {
        self.clean_up_memory(entities);
        self.store_memory()?;
        // write back memory
        _ = lock.insert(self);
        Ok(())
    }

    fn load_memory() -> Result<Memory> {
        info!("loading memory");
        let js_memory = raw_memory::get();
        let json_memory: String = js_memory.into();
        let memory: Memory = serde_json::from_str(&json_memory)?;
        Ok(memory)
    }

    fn clean_up_memory(&mut self, entities: &Entities) {
        let creeps = entities
            .creeps
            .iter()
            .map(|v| v.name())
            .collect::<HashSet<_>>();
        self.creeps.retain(|name, _| creeps.contains(name));
    }

    fn store_memory(&self) -> Result<()> {
        let json_memory = serde_json::to_string(&self)?;
        let js_memory = JsString::from(json_memory);
        raw_memory::set(&js_memory);
        Ok(())
    }
}
