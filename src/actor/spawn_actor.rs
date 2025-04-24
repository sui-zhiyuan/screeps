use crate::actor::creep_actor::CreepHarvester;
use anyhow::{Result, anyhow};
use log::info;
use screeps::{Part, SpawnOptions, StructureSpawn, find, game};
use wasm_bindgen::JsValue;

pub(crate) fn run(spawn: &StructureSpawn) -> Result<()> {
    if spawn.spawning().is_some() {
        info!("spawning...");
        return Ok(());
    }

    let room = spawn.room().ok_or(anyhow!("room not found"))?;
    let creeps = room.find(find::CREEPS, None);
    if !creeps.is_empty() {
        return Ok(());
    }

    info!("creating worker");
    let body = [Part::Carry, Part::Work, Part::Move, Part::Move];
    let cost = body.iter().map(|p| p.cost()).sum();

    if room.energy_available() < cost {
        info!("not enough energy");
        return Ok(());
    }

    let additional = 0;
    // create a unique name, spawn.
    let name_base = game::time();
    let name = format!("{}-{}", name_base, additional);

    let sources = room.find(find::SOURCES, None);
    let source = sources.first().ok_or(anyhow!("no source found"))?;

    let memory = CreepHarvester::new(source, spawn);
    let memory: JsValue =
        serde_wasm_bindgen::to_value(&memory).map_err(|e| anyhow!(e.to_string()))?;
    let option = SpawnOptions::new().memory(memory);

    spawn.spawn_creep_with_options(&body, &name, &option)?;
    Ok(())
}
