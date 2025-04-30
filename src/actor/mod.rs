use anyhow::Result;
use screeps::{Creep, StructureSpawn};

mod creep_actor;
mod creep_builder;
mod creep_harvester;
mod creep_upgrader;
mod spawn_actor;
mod room_actor;

use crate::entity::Entities;
use crate::memory::Memory;
use crate::task::Tasks;
pub use creep_actor::{CreepMemory , CreepClass};
pub use spawn_actor::SpawnMemory;
pub use room_actor::RoomMemory;

trait Actor {
    fn plan(&self, memory: &mut Memory, tasks: &mut Tasks) -> Result<()>;
    fn run(&self, memory: &mut Memory) -> Result<()>;
}

pub fn run(entities: &Entities, memory: &mut Memory) {
    let mut tasks = Tasks::default();

    let spawns = entities.spawns.iter().map(|v| ActorEntity::from(v));
    let creeps = entities.creeps.iter().map(|v| ActorEntity::from(v));
    let actors = spawns.chain(creeps).collect::<Vec<_>>();

    for actor in actors.iter() {
        actor.plan(memory, &mut tasks).expect("todo");
    }

    for actor in actors.iter() {
        actor.run(memory).expect("todo")
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
    fn plan(&self, memory: &mut Memory, tasks: &mut Tasks) -> Result<()> {
        match self {
            ActorEntity::Spawn(spawn) => spawn.plan(memory, tasks),
            ActorEntity::Creep(c) => c.plan(memory, tasks),
        }
    }

    fn run(&self, memory: &mut Memory) -> Result<()> {
        match self {
            ActorEntity::Spawn(spawn) => spawn.run(memory),
            ActorEntity::Creep(creep) => creep.run(memory),
        }
    }
}
