use crate::actor::CreepMemoryTrait;
use crate::actor::creep_actor::CreepMemory;
use anyhow::anyhow;
use log::info;
use screeps::action_error_codes::{HarvestErrorCode, TransferErrorCode};
use screeps::{
    Creep, HasId, ObjectId, ResourceType, SharedCreepProperties, Source, StructureSpawn,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreepHarvesterMemory {
    pub source: ObjectId<Source>,
    pub spawn: ObjectId<StructureSpawn>,
}

impl CreepHarvesterMemory {
    pub fn new_memory(source: &Source, spawn: &StructureSpawn) -> CreepMemory {
        CreepMemory::Harvester(CreepHarvesterMemory {
            source: source.id(),
            spawn: spawn.id(),
        })
    }
}

impl CreepMemoryTrait for CreepHarvesterMemory {
    fn run(&mut self, creep: &Creep) -> anyhow::Result<()> {
        if creep.spawning() {
            return Ok(());
        }

        if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            info!("to source");
            let source = &self.source.resolve().ok_or(anyhow!("source not found"))?;
            let result = creep.harvest(source);
            if let Err(HarvestErrorCode::NotInRange) = result {
                creep.move_to(source)?;
                return Ok(());
            }
            result?;
        } else {
            info!("to spawn");
            let spawn = &self.spawn.resolve().ok_or(anyhow!("spawn not found"))?;
            let result = creep.transfer(spawn, ResourceType::Energy, None);
            if let Err(TransferErrorCode::NotInRange) = result {
                creep.move_to(spawn)?;
                return Ok(());
            }
            result?
        }
        Ok(())
    }
}
