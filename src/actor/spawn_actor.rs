use anyhow::{Result, anyhow};
use log::info;
use screeps::{Part, SpawnOptions, StructureSpawn, find, game};

use crate::actor::creep_actor::CreepMemory;
use crate::actor::creep_harvester::CreepHarvesterMemory;
use crate::actor::creep_upgrader::CreepUpgraderMemory;

pub(crate) fn run(spawn: &StructureSpawn) -> Result<()> {
    if spawn.spawning().is_some() {
        info!("spawning...");
        return Ok(());
    }

    let room = spawn.room().ok_or(anyhow!("room not found"))?;
    let creeps = room.find(find::CREEPS, None);
    let structure = match creeps.len() {
        0 => CreepStructure::new_harvest(spawn)?,
        1 => CreepStructure::new_upgrader(spawn)?,
        _ => return Ok(()),
    };

    info!("creating");
    if room.energy_available() < structure.cost() {
        info!("not enough energy");
        return Ok(());
    }

    let option = SpawnOptions::new().memory(structure.memory.to_js_value()?);

    spawn.spawn_creep_with_options(&structure.body, &structure.name, &option)?;
    Ok(())
}

struct CreepStructure {
    name: String,
    body: Vec<Part>,
    memory: CreepMemory,
}

impl CreepStructure {
    fn new_harvest(spawn: &StructureSpawn) -> Result<CreepStructure> {
        let name_base = game::time();
        let name = format!("{name_base}-0");

        let room = spawn.room().ok_or(anyhow!("room not found"))?;
        let sources = room.find(find::SOURCES, None);
        let source = sources.first().ok_or(anyhow!("no source found"))?;

        let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];
        let memory = CreepHarvesterMemory::new_memory(source, spawn);

        Ok(CreepStructure { name, body, memory })
    }

    fn new_upgrader(spawn: &StructureSpawn) -> Result<CreepStructure> {
        let name_base = game::time();
        let name = format!("{name_base}-0");

        let room = spawn.room().ok_or(anyhow!("room not found"))?;
        let controller = room.controller().ok_or(anyhow!("controller not found"))?;

        let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];
        let memory = CreepUpgraderMemory::new_memory(spawn, &controller);

        Ok(CreepStructure { name, body, memory })
    }

    fn cost(&self) -> u32 {
        self.body.iter().map(|p| p.cost()).sum()
    }
}
