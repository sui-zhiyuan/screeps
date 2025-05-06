use log::error;
use screeps::{Creep, SharedCreepProperties, game};
use std::collections::HashSet;

mod creep_actor;
mod creep_builder;
mod creep_harvester;
mod creep_upgrader;
mod spawn_actor;

use crate::Memory;
pub use creep_actor::CreepMemory;
pub use creep_harvester::CreepHarvesterMemory;

pub struct RoomMemory();

impl RoomMemory {
    pub fn new() -> RoomMemory {
        RoomMemory()
    }
}
pub struct SpawnMemory();
pub struct FlagMemory();

#[derive(Default)]
pub struct RunResult {
    pub spawns: HashSet<String>,
    pub creeps: HashSet<String>,
}

pub fn run(memory: &mut Memory) -> RunResult {
    let mut result = RunResult::default();
    for s in game::spawns().values() {
        if let Err(e) = spawn_actor::run(&s, memory) {
            error!("spawn running error {}", e);
        }

        result.spawns.insert(s.name());
    }

    for c in game::creeps().values() {
        if let Err(e) = creep_actor::run(&c, memory) {
            error!("creep run error {}", e);
        }
        result.creeps.insert(c.name());
    }

    result
}

trait CreepMemoryTrait {
    fn run(&mut self, creep: &Creep) -> anyhow::Result<()>;
}
