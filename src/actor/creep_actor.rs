use crate::Memory;
use crate::actor::CreepMemoryTrait;
use crate::actor::creep_builder::CreepBuilderMemory;
use crate::actor::creep_harvester::CreepHarvesterMemory;
use crate::actor::creep_upgrader::CreepUpgraderMemory;
use anyhow::{Result, anyhow};
use screeps::{Creep, SharedCreepProperties};
use serde::{Deserialize, Serialize};

pub fn run(creep: &Creep) -> Result<()> {
    let mut memory = None;

    Memory::access(|m| {
        memory = m.creeps.get(&creep.name()).cloned();
    })?;

    let mut memory = memory.ok_or_else(|| anyhow!("memory not found"))?;
    match memory {
        CreepMemory::Harvester(ref mut memory) => memory.run(creep),
        CreepMemory::Upgrader(ref mut memory) => memory.run(creep),
        CreepMemory::Builder(ref mut memory) => memory.run(creep),
    }?;

    Memory::access(|m1| {
        m1.creeps.insert(creep.name(), memory);
    })?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreepMemory {
    Harvester(CreepHarvesterMemory),
    Upgrader(CreepUpgraderMemory),
    Builder(CreepBuilderMemory),
}
