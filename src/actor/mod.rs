use anyhow::Result;
use log::error;
use screeps::game::creeps;
use screeps::{Creep, SharedCreepProperties, StructureSpawn, game};
use std::collections::HashSet;

mod creep_actor;
mod creep_builder;
mod creep_harvester;
mod creep_upgrader;
mod spawn_actor;

use crate::entity::Entities;
use crate::memory::Memory;
use crate::task::Task;
pub use creep_actor::CreepMemory;

trait Actor {
    fn plan(&self, entities: &Entities, memory: &mut Memory, tasks: &mut Vec<Task>) -> Result<()>;
    fn run(&self, memory: &mut Memory) -> Result<()>;
}

pub fn run(entities: &Entities, memory: &mut Memory) {
    let spawns = entities.spawns.values().map(|v| ActorEntity::from(v));

    let creeps = entities.creeps.values().map(|v| ActorEntity::from(v));

    let mut tasks = Vec::new();

    let actors = spawns.chain(creeps).collect::<Vec<_>>();

    for actor in actors {
        actor.plan(entities, memory, &mut tasks).expect("todo");
    }

    // let spawns = ctx.spawns.iter().map(|(name, spawn)| ()).collect();
    //
    // let mut result = RunResult::default();
    // for s in game::spawns().values() {
    //     if let Err(e) = spawn_actor::run(&s, memory) {
    //         error!("spawn running error {}", e);
    //     }
    //
    //     result.spawns.insert(s.name());
    // }
    //
    // for c in game::creeps().values() {
    //     if let Err(e) = creep_actor::run(&c, memory) {
    //         error!("creep run error {}", e);
    //     }
    //     result.creeps.insert(c.name());
    // }
    //
    // result
}

trait CreepMemoryTrait {
    fn run(&mut self, creep: &Creep) -> anyhow::Result<()>;
}

enum ActorEntity<'a> {
    Spawn(&'a StructureSpawn),
    Creep(&'a Creep),
}

impl<'a> From<&'a StructureSpawn> for ActorEntity<'a> {
    fn from(value: &'a StructureSpawn) -> Self {
        ActorEntity::Spawn(value)
    }
}

impl<'a> From<&'a Creep> for ActorEntity<'a> {
    fn from(value: &'a Creep) -> Self {
        ActorEntity::Creep(value)
    }
}

impl<'a> Actor for ActorEntity<'a> {
    fn plan(&self, entities: &Entities, memory: &mut Memory, tasks: &mut Vec<Task>) -> Result<()> {
        match self {
            ActorEntity::Spawn(spawn) => spawn.plan(entities, memory, tasks),
            ActorEntity::Creep(c) => c.plan(entities, memory, tasks),
        }
    }

    fn run(&self, memory: &mut Memory) -> Result<()> {
        match self {
            ActorEntity::Spawn(spawn) => spawn.run(memory),
            ActorEntity::Creep(creep) => creep.run(memory),
        }
    }
}
