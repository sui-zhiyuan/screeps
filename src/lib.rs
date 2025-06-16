mod actor;
mod context;
mod memory;
mod tracing;

use crate::context::Context;
use ::tracing::{error, info};
use anyhow::Result;
use screeps::game;
use wasm_bindgen::prelude::*;

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    if let Err(e) = game_loop_inner() {
        error!("loop failed : {}", e);
    }
}

fn game_loop_inner() -> Result<()> {
    console_error_panic_hook::set_once();
    tracing::init();

    info!("loop starting! CPU: {}", game::cpu::get_used());
    let ctx = Context::new()?;
    actor::run(&ctx);

    ctx.store()?;
    info!("done! cpu: {}", game::cpu::get_used());

    Ok(())
}
