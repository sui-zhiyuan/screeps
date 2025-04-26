use log::error;
use screeps::{game, Creep};

mod creep_actor;
mod spawn_actor;
mod creep_harvester;
mod creep_upgrader;

pub fn run() {
    for s in game::spawns().values() {
        match spawn_actor::run(&s) {
            Ok(_) => (),
            Err(e) => {
                error!("spawn running error {}", e);
            }
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