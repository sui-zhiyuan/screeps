use anyhow::{Result, anyhow};
use log::info;
use screeps::action_error_codes::{HarvestErrorCode, TransferErrorCode};
use screeps::{Creep, ObjectId, ResourceType, SharedCreepProperties, Source, StructureController, StructureSpawn, find, HasId};
use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CreepHarvester {
    source: ObjectId<Source>,
    spawn: ObjectId<StructureSpawn>,
}

impl CreepHarvester{
    pub fn new(source :&Source ,spawn: &StructureSpawn) -> Self{
        CreepHarvester{
            source: source.id(),
            spawn: spawn.id(),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub(crate) struct CreepActorUpgrader {
    spawn: ObjectId<StructureSpawn>,
    controller: ObjectId<StructureController>,
}

trait CreepActor {
    fn to_memory(&self) -> JsValue;
    fn from_memory(mem: JsValue) -> Self;
}

pub(super) fn run(creep: &Creep) -> Result<()> {
    info!("running creep with {:?}", creep.spawning());
    if creep.spawning() {
        return Ok(());
    }
    let room = creep.room().ok_or(anyhow!("room not found"))?;

    if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
        info!("to source");
        let sources = room.find(find::SOURCES, None);
        let source = sources.first().ok_or(anyhow!("no source found"))?;

        let result = creep.harvest(source);
        if let Err(HarvestErrorCode::NotInRange) = result {
            creep.move_to(source)?;
            return Ok(());
        }
        result?;
    } else {
        info!("to spawn");
        let spawns = room.find(find::MY_SPAWNS, None);
        let spawn = spawns.first().ok_or(anyhow!("no spawn found"))?;
        let result = creep.transfer(spawn, ResourceType::Energy, None);
        if let Err(TransferErrorCode::NotInRange) = result {
            creep.move_to(spawn)?;
            return Ok(());
        }
        result?
    }
    Ok(())
}
