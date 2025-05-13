use anyhow::Result;
use log::*;
use screeps::game;
use wasm_bindgen::prelude::*;
use crate::memory::Memory;

mod actor;
mod logging;
mod memory;

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

    // let mut memory_lock = MEMORY
    //     .lock()
    //     .map_err(|e| anyhow!("memory lock err: {}", e))?;
    // let memory = match memory_lock.take() {
    //     Some(memory) => memory,
    //     None => load_memory()?,
    // };

    let (memory_guard , mut memory) = Memory::load()?;

    info!("loop starting! CPU: {}", game::cpu::get_used());

    actor::run(&mut memory);
    memory.store(memory_guard)?;

    info!("done! cpu: {}", game::cpu::get_used());

    Ok(())
}
