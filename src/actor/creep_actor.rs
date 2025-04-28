use crate::Memory;
use crate::actor::creep_builder::CreepBuilderMemory;
use crate::actor::creep_harvester::CreepHarvesterMemory;
use crate::actor::creep_upgrader::CreepUpgraderMemory;
use crate::actor::{Actor, CreepMemoryTrait};
use crate::entity::Entities;
use crate::task::{Task, Tasks};
use anyhow::{Result, anyhow};
use screeps::{Creep, SharedCreepProperties, StructureSpawn};
use serde::{Deserialize, Serialize};

pub fn run(creep: &Creep, memory: &mut Memory) -> Result<()> {
    let memory = memory
        .creeps
        .get_mut(&creep.name())
        .ok_or(anyhow!("memory not found"))?;
    match memory {
        CreepMemory::Harvester(memory) => memory.run(creep),
        CreepMemory::Upgrader(memory) => memory.run(creep),
        CreepMemory::Builder(memory) => memory.run(creep),
    }
}

impl Actor for Creep {
    fn plan(&self, memory: &mut Memory, tasks: &mut Tasks) -> Result<()> {
        todo!()
    }

    fn run(&self, memory: &mut Memory) -> Result<()> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role")]
pub enum CreepMemory {
    Harvester(CreepHarvesterMemory),
    Upgrader(CreepUpgraderMemory),
    Builder(CreepBuilderMemory),
}
