use ::tracing::{error, info};
use anyhow::Result;
use screeps::game;
use wasm_bindgen::prelude::*;
use crate::entity::test1;

mod actor;
mod memory;
mod tracing;
mod entity;
mod task;

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    if let Err(e) = game_loop_inner() {
        error!("loop failed : {}", e);
    }
}

fn game_loop_inner() -> Result<()> {
    tracing::init();

    info!("loop starting! CPU: {}", game::cpu::get_used());

    actor::run();

    memory::store()?;
    test1();

    info!("done! cpu: {}", game::cpu::get_used());

    Ok(())
}
