use tracing::error;
use screeps::{Creep, game};

mod creep_actor;
mod creep_builder;
mod creep_harvester;
mod creep_upgrader;
mod spawn_actor;

pub use creep_actor::CreepMemory;

pub fn run() {
    for s in game::spawns().values() {
        if let Err(e) = spawn_actor::run(&s) {
            error!("spawn running error {}", e);
        }
    }

    for c in game::creeps().values() {
        if let Err(e) = creep_actor::run(&c) {
            error!("creep run error {}", e);
        }
    }
}

trait CreepMemoryTrait {
    fn run(&mut self, creep: &Creep) -> anyhow::Result<()>;
}
