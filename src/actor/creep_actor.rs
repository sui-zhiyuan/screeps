use crate::Memory;
use crate::actor::CreepMemoryTrait;
use crate::actor::creep_builder::CreepBuilderMemory;
use crate::actor::creep_harvester::CreepHarvesterMemory;
use crate::actor::creep_upgrader::CreepUpgraderMemory;
use anyhow::{Result, anyhow};
use screeps::{Creep, SharedCreepProperties};
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role")]
pub enum CreepMemory {
    Harvester(CreepHarvesterMemory),
    Upgrader(CreepUpgraderMemory),
    Builder(CreepBuilderMemory),
}
