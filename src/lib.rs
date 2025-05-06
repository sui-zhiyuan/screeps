use crate::actor::{CreepMemory, RunResult};
use anyhow::{Result, anyhow};
use js_sys::JsString;
use log::*;
use screeps::{game, raw_memory};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

mod actor;
mod entity;
mod logging;
mod memory;

#[derive(Serialize, Deserialize, Default)]
struct Memory {
    rooms: HashMap<String, ()>,
    spawns: HashMap<String, ()>,
    creeps: HashMap<String, CreepMemory>,
    flags: HashMap<String, ()>,
}

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

static MEMORY: Mutex<Option<Memory>> = Mutex::new(None);

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    if let Err(e) = game_loop_inner() {
        error!("loop failed : {}", e);
    }
}

fn game_loop_inner() -> Result<()> {
    INIT_LOGGING.call_once(|| {
        // show all output of Info level, adjust as needed
        logging::setup_logging(logging::Info);
    });

    let mut memory_lock = MEMORY
        .lock()
        .map_err(|e| anyhow!("memory lock err: {}", e))?;
    let memory = match memory_lock.take() {
        Some(memory) => memory,
        None => load_memory()?,
    };
    let memory = memory_lock.insert(memory);

    info!("loop starting! CPU: {}", game::cpu::get_used());

    let called = actor::run(memory);
    store_memory(memory, &called)?;

    info!("done! cpu: {}", game::cpu::get_used());

    Ok(())
}

fn load_memory() -> Result<Memory> {
    info!("loading memory");
    let js_memory = raw_memory::get();
    let json_memory: String = js_memory.into();
    let memory: Memory = serde_json::from_str(&json_memory)?;
    Ok(memory)
}

fn store_memory(memory: &mut Memory, called: &RunResult) -> Result<()> {
    memory.creeps.retain(|name, _| called.creeps.contains(name));

    let json_memory = serde_json::to_string(&memory)?;
    let js_memory = JsString::from(json_memory);
    raw_memory::set(&js_memory);
    Ok(())
}
