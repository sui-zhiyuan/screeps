use std::cell::RefCell;
use screeps::game;
use tracing::{error, info};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::actor::Actors;
use crate::memory::Memory;
use crate::planner;
use crate::task::Tasks;

thread_local! {
    static CONTEXT: RefCell<Option<Context>> = const { RefCell::new(None) };
}

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    if let Err(e) = CONTEXT.with(|ctx| {
        let ctx = &mut *ctx.borrow_mut();
        game_loop_inner(ctx)
    }) {
        error!("loop failed : {}", e);
    }
}

pub struct Context {
    actors: Actors,
    memory: Memory,
    tasks: Tasks,
}

fn game_loop_inner(ctx: &mut Option<Context>) -> anyhow::Result<()> {
    let local_ctx = match ctx.take() {
        Some(ctx) => ctx,
        None => bootstrap()?,
    };

    info!("loop starting! CPU: {}", game::cpu::get_used());
    let Context {
        mut actors,
        mut memory,
        mut tasks,
    } = local_ctx;

    planner::plan(&mut actors, &mut tasks)?;
    actors.run(&mut tasks)?;

    actors.store_memory(&mut memory)?;
    tasks.store_memory(&mut memory);
    memory.store_to_raw()?;

    // if anything goes wrong, not setting ctx back trigger a restart in next loop;
    ctx.replace(Context {
        actors,
        memory,
        tasks,
    });

    info!("loop complete! cpu: {}", game::cpu::get_used());
    Ok(())
}

fn bootstrap() -> anyhow::Result<Context> {
    console_error_panic_hook::set_once();
    crate::tracing::init();
    info!("bootstrapping! CPU: {}", game::cpu::get_used());

    let mut memory = Memory::load_from_raw()?;
    let tasks = Tasks::from_memory(&memory);
    let actors = Actors::build_actors(&mut memory)?;
    Ok(Context {
        actors,
        memory,
        tasks,
    })
}