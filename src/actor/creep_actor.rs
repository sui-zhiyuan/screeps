use anyhow::{Result, anyhow};
use gloo_utils::format::JsValueSerdeExt;
use log::info;
use screeps::action_error_codes::{HarvestErrorCode, TransferErrorCode};
use screeps::{
    Creep, HasId, HasPosition, ObjectId, ResourceType, SharedCreepProperties, Source,
    StructureController, StructureSpawn,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

pub fn run(creep: &Creep) -> Result<()> {
    let mut memory = CreepMemory::from_js_value(creep.memory())?;
    match &mut memory {
        CreepMemory::Harvester(memory) => memory.run(&creep),
        CreepMemory::Upgrader(memory) => memory.run(&creep),
    }?;

    creep.set_memory(&memory.to_js_value()?);
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
            state: State::Loading,
        })
    }

    pub fn to_js_value(&self) -> Result<JsValue> {
        JsValue::from_serde(&self).map_err(|e| e.into())
    }

    pub fn from_js_value(value: JsValue) -> Result<Self> {
        value.into_serde().map_err(|e| e.into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreepHarvesterMemory {
    source: ObjectId<Source>,
    spawn: ObjectId<StructureSpawn>,
}

impl CreepMemoryTrait for CreepHarvesterMemory {
    fn run(&mut self, creep: &Creep) -> Result<()> {
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
    state: State,
}

#[derive(Serialize, Deserialize, Debug)]
enum State {
    Loading,
    Upgrading,
}

impl CreepMemoryTrait for CreepUpgraderMemory {
    fn run(&mut self, creep: &Creep) -> Result<()> {
        if creep.spawning() {
            return Ok(());
        }

        if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
            let spawn = &self.spawn.resolve().ok_or(anyhow!("spawn not found"))?;
            if !creep.pos().is_near_to(spawn.pos()) {
                creep.move_to(&spawn)?;
                return Ok(());
            }
            if creep.store().get_free_capacity(Some(ResourceType::Energy))
                > spawn.store().get_used_capacity(Some(ResourceType::Energy)) as i32
            {
                return Ok(());
            }

            creep.withdraw(spawn, ResourceType::Energy, None)?;
            Ok(())
        } else {
            let controller = &self
                .controller
                .resolve()
                .ok_or(anyhow!("controller not found"))?;
            if !creep.pos().is_near_to(controller.pos()) {
                creep.move_to(&controller)?;
                return Ok(());
            }

            creep.upgrade_controller(&controller)?;
            Ok(())
        }
    }
}
