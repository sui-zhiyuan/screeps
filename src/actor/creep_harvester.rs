use crate::actor::CreepMemoryTrait;
use crate::actor::creep_actor::CreepMemory;
use anyhow::{Error, anyhow};
use screeps::action_error_codes::{HarvestErrorCode, TransferErrorCode};
use screeps::{
    Creep, HasId, ObjectId, ResourceType, SharedCreepProperties, Source, StructureSpawn, game,
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(
    into = "CreepHarvesterMemoryStore",
    try_from = "CreepHarvesterMemoryStore"
)]
pub struct CreepHarvesterMemory {
    source: ObjectId<Source>,
    spawn: ObjectId<StructureSpawn>,
}

impl TryFrom<CreepHarvesterMemoryStore> for CreepHarvesterMemory {
    type Error = Error;

    fn try_from(value: CreepHarvesterMemoryStore) -> Result<Self, Self::Error> {
        let source = value.source;
        let spawn = game::spawns().get(value.spawn).expect("spawn not found");

        Ok(CreepHarvesterMemory {
            source,
            spawn: spawn.id(),
        })
    }
}

impl From<CreepHarvesterMemory> for CreepHarvesterMemoryStore {
    fn from(value: CreepHarvesterMemory) -> Self {
        let source = value.source;

        let spawn = value
            .spawn
            .resolve()
            .expect("CreepHarvesterMemoryStore doesn't exist")
            .name()
            .to_string();
        CreepHarvesterMemoryStore { source, spawn }
    }
}

#[derive(Serialize, Deserialize)]
struct CreepHarvesterMemoryStore {
    source: ObjectId<Source>,
    spawn: String,
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
