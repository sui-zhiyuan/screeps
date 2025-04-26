use anyhow::{Result, anyhow, bail};
use gloo_utils::format::JsValueSerdeExt;
use log::info;
use screeps::action_error_codes::{HarvestErrorCode, TransferErrorCode};
use screeps::{
    Creep, HasId, ObjectId, ResourceType, SharedCreepProperties, Source, StructureController,
    StructureSpawn,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

pub fn run(creep: &Creep) -> Result<()> {
    let mut memory = CreepMemory::from_js_value(creep.memory())?;
    match &mut memory {
        CreepMemory::Harvester(memory) => memory.run(&creep),
        CreepMemory::Upgrader(memory) => memory.run(&creep),
    }?;

    // TODO save memory

    Ok(())
}

trait CreepMemoryTrait {
    fn run(&mut self, creep: &Creep) -> Result<()>;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role")]
pub enum CreepMemory {
    Harvester(CreepHarvesterMemory),
    Upgrader(CreepUpgraderMemory),
}

impl CreepMemory {
    pub fn new_harvester(source: &Source, spawn: &StructureSpawn) -> Self {
        Self::Harvester(CreepHarvesterMemory {
            source: source.id(),
            spawn: spawn.id(),
        })
    }

    pub fn new_upgrader(spawn: &StructureSpawn, controller: &StructureController) -> Self {
        Self::Upgrader(CreepUpgraderMemory {
            spawn: spawn.id(),
            controller: controller.id(),
        })
    }

    pub fn to_js_value(&self) -> Result<JsValue> {
        JsValue::from_serde(&self).map_err(|e| e.into())
    }

    pub fn from_js_value(value: JsValue) -> Result<Self> {
        info!("before parse memory {value:?}");
        let result = value.into_serde()?;
        info!("after parse memory");
        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreepHarvesterMemory {
    source: ObjectId<Source>,
    spawn: ObjectId<StructureSpawn>,
}

impl CreepMemoryTrait for CreepHarvesterMemory {
    fn run(&mut self, creep: &Creep) -> Result<()> {
        info!("running creep with {:?}", creep.spawning());

        if creep.spawning() {
            return Ok(());
        }

        if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            info!("to source");
            let source = &self.source.resolve().ok_or(anyhow!("source not found"))?;
            let result = creep.harvest(source);
            if let Err(HarvestErrorCode::NotInRange) = result {
                creep.move_to(&source)?;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreepUpgraderMemory {
    spawn: ObjectId<StructureSpawn>,
    controller: ObjectId<StructureController>,
}

impl CreepMemoryTrait for CreepUpgraderMemory {
    fn run(&mut self, _creep: &Creep) -> Result<()> {
        bail!("not completed")
    }
}
