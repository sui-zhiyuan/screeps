use crate::actor::CreepMemoryTrait;
use crate::actor::creep_builder::CreepBuilderMemory;
use crate::actor::creep_harvester::CreepHarvesterMemory;
use crate::actor::creep_upgrader::CreepUpgraderMemory;
use crate::context::Context;
use crate::memory::MemoryAccessor;
use anyhow::Result;
use screeps::Creep;
use serde::{Deserialize, Serialize};

pub fn run(ctx: &Context, creep: &Creep) -> Result<()> {
    let mut memory = ctx.memory().load(creep);

    match memory {
        CreepMemory::None => todo!("invalid memory"),
        CreepMemory::Harvester(ref mut memory) => memory.run(creep),
        CreepMemory::Upgrader(ref mut memory) => memory.run(creep),
        CreepMemory::Builder(ref mut memory) => memory.run(creep),
    }?;

    ctx.memory().store(creep, memory);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum CreepMemory {
    #[default]
    None,
    Harvester(CreepHarvesterMemory),
    Upgrader(CreepUpgraderMemory),
    Builder(CreepBuilderMemory),
}
