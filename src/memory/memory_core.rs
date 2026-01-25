use crate::actor::{CreepMemory, RoomMemories, SpawnMemories};
use crate::task::TaskMemory;
use js_sys::JsString;
use screeps::raw_memory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Memory {
    #[serde(default)]
    pub rooms: RoomMemories,
    #[serde(default)]
    pub spawns: SpawnMemories,
    #[serde(default)]
    pub creeps: HashMap<String, CreepMemory>,
    #[serde(default)]
    pub flags: HashMap<String, ()>,
    #[serde(default)]
    pub tasks: TaskMemory,
}

impl Memory {
    pub fn load_from_raw() -> anyhow::Result<Memory> {
        let js_memory = raw_memory::get();
        let json_value: String = js_memory.into();
        let memory: Memory = serde_json::from_str(&json_value)?;
        Ok(memory)
    }

    pub fn store_to_raw(&self) -> anyhow::Result<()> {
        let js_value = serde_json::to_string(&self)?;
        let js_memory = JsString::from(js_value);
        raw_memory::set(&js_memory);
        Ok(())
    }
}
