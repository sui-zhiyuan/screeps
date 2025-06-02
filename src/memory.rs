use crate::actor::{CreepMemory, RoomMemory};
use crate::task::TaskSerializePhantom;
use anyhow::{Result, anyhow};
use js_sys::JsString;
use screeps::{Room, RoomName, SharedCreepProperties, game, raw_memory};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::DerefMut;
use std::sync::{LazyLock, Mutex, MutexGuard};
use tracing::info;

static MEMORY: LazyLock<Result<Mutex<Memory>>> =
    LazyLock::new(|| Memory::load_from_raw().map(Mutex::new));

#[derive(Serialize, Deserialize, Default)]
pub struct Memory {
    pub rooms: HashMap<RoomName, RoomMemory>,
    pub spawns: HashMap<String, ()>,
    pub creeps: HashMap<String, CreepMemory>,
    pub flags: HashMap<String, ()>,
    #[serde(default)]
    pub tasks: TaskSerializePhantom,
}

impl Memory {
    // TODO make f return Result
    pub fn with<TR>(f: impl FnOnce(&mut Memory) -> Result<TR>) -> Result<TR> {
        let mut guard = Memory::get_guard()?;
        let v = guard.deref_mut();
        f(v)
    }

    pub fn store_to_raw() -> Result<()> {
        let mut guard = Memory::get_guard()?;
        guard.clean_up_memory();
        guard.store_memory()?;
        Ok(())
    }

    fn load_from_raw() -> Result<Memory> {
        info!("loading memory");
        let js_memory = raw_memory::get();
        let json_memory: String = js_memory.into();
        let memory: Memory = serde_json::from_str(&json_memory)?;
        Ok(memory)
    }

    fn get_guard() -> Result<MutexGuard<'static, Memory>> {
        MEMORY
            .as_ref()
            .map_err(|e| anyhow!("load memory error: {}", e))?
            .lock()
            .map_err(|e| anyhow!("memory lock err: {}", e))
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

pub trait MemoryAccessor<TEntity> {
    type TMemory: Clone;

    fn with<TR>(e: &TEntity, f: impl FnOnce(&mut Self::TMemory) -> Result<TR>) -> Result<TR>;
    fn load(e: &TEntity) -> Result<Self::TMemory> {
        Self::with(e, |memory| Ok(memory.clone()))
    }
    fn store(e: &TEntity, memory: Self::TMemory) -> Result<()> {
        Self::with(e, |m| {
            *m = memory;
            Ok(())
        })
    }
}

impl MemoryAccessor<Room> for Memory {
    type TMemory = RoomMemory;

    fn with<TR>(e: &Room, f: impl FnOnce(&mut Self::TMemory) -> Result<TR>) -> Result<TR> {
        Memory::with(|memory| {
            let m = memory.rooms.entry(e.name()).or_default();
            f(m)
        })
    }
}
