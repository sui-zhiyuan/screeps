use anyhow::Result;
use gloo_utils::format::JsValueSerdeExt;
use screeps::Creep;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::actor::CreepMemoryTrait;
use crate::actor::creep_harvester::CreepHarvesterMemory;
use crate::actor::creep_upgrader::CreepUpgraderMemory;

pub fn run(creep: &Creep) -> Result<()> {
    let mut memory = CreepMemory::from_js_value(creep.memory())?;
    match &mut memory {
        CreepMemory::Harvester(memory) => memory.run(creep),
        CreepMemory::Upgrader(memory) => memory.run(creep),
    }?;

    creep.set_memory(&memory.to_js_value()?);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role")]
pub enum CreepMemory {
    Harvester(CreepHarvesterMemory),
    Upgrader(CreepUpgraderMemory),
}

impl CreepMemory {
    pub fn to_js_value(&self) -> Result<JsValue> {
        JsValue::from_serde(&self).map_err(|e| e.into())
    }

    pub fn from_js_value(value: JsValue) -> Result<Self> {
        value.into_serde().map_err(|e| e.into())
    }
}
