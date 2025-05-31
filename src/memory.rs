use crate::actor::CreepMemory;
use anyhow::{Result, anyhow};
use js_sys::JsString;
use tracing::info;
use screeps::{SharedCreepProperties, game, raw_memory};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard, OnceLock};

static MEMORY: OnceLock<Result<Mutex<Memory>>> = OnceLock::new();

#[derive(Serialize, Deserialize, Default)]
pub struct Memory {
    pub rooms: HashMap<String, ()>,
    pub spawns: HashMap<String, ()>,
    pub creeps: HashMap<String, CreepMemory>,
    pub flags: HashMap<String, ()>,
}

impl Memory {
    pub fn access(f: impl FnOnce(&mut Memory)) -> Result<()> {
        let mut guard = Self::get_guard()?;
        let v = guard.deref_mut();
        f(v);
        Ok(())
    }

    pub fn store() -> Result<()> {
        let mut guard = Self::get_guard()?;
        guard.clean_up_memory();
        guard.store_memory()?;
        Ok(())
    }

    fn get_guard() -> Result<MutexGuard<'static, Memory>> {
        MEMORY
            .get_or_init(|| Self::load().map(Mutex::new))
            .as_ref()
            .map_err(|e| anyhow!("load memory error: {}", e))?
            .lock()
            .map_err(|e| anyhow!("memory lock err: {}", e))
    }

    fn load() -> Result<Memory> {
        info!("loading memory");
        let js_memory = raw_memory::get();
        let json_memory: String = js_memory.into();
        let memory: Memory = serde_json::from_str(&json_memory)?;
        Ok(memory)
    }

    fn clean_up_memory(&mut self) {
        let creeps = game::creeps()
            .values()
            .map(|c| c.name())
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
