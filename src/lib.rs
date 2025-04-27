use anyhow::Result;
use log::*;
use screeps::game;
use wasm_bindgen::prelude::*;

mod actor;
mod entity;
mod logging;
mod memory;
mod task;

use crate::memory::Memory;
use entity::Entities;

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

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
    info!("loop starting! CPU: {}", game::cpu::get_used());

    let (memory_lock, mut memory) = Memory::load()?;
    let entities = Entities::new();
    actor::run(&entities, &mut memory);
    memory.store(memory_lock, &entities)?;

    info!("done! cpu: {}", game::cpu::get_used());
    Ok(())
}
