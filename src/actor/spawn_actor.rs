use crate::actor::Actor;
use crate::actor::creep_actor::CreepMemory;
use crate::actor::creep_builder::CreepBuilderMemory;
use crate::actor::creep_harvester::CreepHarvesterMemory;
use crate::actor::creep_upgrader::CreepUpgraderMemory;
use crate::memory::Memory;
use crate::task::{Task, TaskId};
use anyhow::{Result, anyhow};
use screeps::{Part, RoomName, StructureSpawn, find, game};
use serde::{Deserialize, Serialize};
use tracing::info;

pub(crate) fn run(spawn: &StructureSpawn) -> Result<()> {
    if spawn.spawning().is_some() {
        info!("spawning...");
        return Ok(());
    }

    let room = spawn.room().ok_or(anyhow!("room not found"))?;
    let creeps = room.find(find::CREEPS, None);
    let structure = match creeps.len() {
        0 => CreepStructure::new_harvest(spawn)?,
        1 => CreepStructure::new_builder(spawn)?,
        2 => CreepStructure::new_upgrader(spawn)?,
        _ => return Ok(()),
    };

    info!("creating");
    if room.energy_available() < structure.cost() {
        info!("not enough energy");
        return Ok(());
    }

    spawn.spawn_creep(&structure.body, &structure.name)?;
    Memory::with(|memory| {
        memory.creeps.insert(structure.name, structure.memory);
        Ok(())
    })?;
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

    fn new_builder(spawn: &StructureSpawn) -> Result<CreepStructure> {
        let name_base = game::time();
        let name = format!("{name_base}-0");

        let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];
        let memory = CreepBuilderMemory::new_memory(spawn);

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

pub struct SpawnMemory {
    spawn_task: TaskId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CreepSpawnTask {
    room: RoomName,
    creep_class: CreepClass,
}

impl CreepSpawnTask {
    pub fn new_task(room: RoomName, creep_class: CreepClass) -> Task {
        Task::CreepSpawn(CreepSpawnTask { room, creep_class })
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum CreepClass {
    Worker,
}

impl Actor for StructureSpawn {
    fn name(&self) -> String {
        // TODO
        "todo spawn".to_string()
    }
    fn plan(&self) -> Result<()> {
        Ok(())
    }

    fn assign(&self) -> Result<()> {
        Ok(())
    }

    fn run(&self) -> Result<()> {
        todo!()
    }
}
