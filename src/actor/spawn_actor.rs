use crate::Memory;
use crate::actor::Actor;
use crate::actor::creep_actor::CreepMemory;
use crate::actor::creep_builder::CreepBuilderMemory;
use crate::actor::creep_harvester::CreepHarvesterMemory;
use crate::actor::creep_upgrader::CreepUpgraderMemory;
use crate::task::TaskTrait;
use crate::task::{Task, TaskId, Tasks};
use anyhow::{Result, anyhow, bail};
use log::info;
use screeps::ResourceType::Energy;
use screeps::{HasId, Part, StructureSpawn, find, game};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SpawnMemory {
    income_energy: i32,
    outgo_energy: i32,
    income_task: Option<TaskId>,
    outgo_task: Vec<TaskId>,
}

impl Actor for StructureSpawn {
    fn plan(&self, memory: &mut Memory, tasks: &mut Tasks) -> Result<()> {
        let memory = memory
            .spawns
            .entry(self.name())
            .or_insert_with(Default::default);

        let required_energy = self.store().get_free_capacity(Some(Energy)) - memory.income_energy;
        if required_energy <= 0 {
            return Ok(());
        }

        let Some(task_id) = memory.income_task else {
            let task = tasks.new_energy_require(self, required_energy);
            memory.income_task = Some(task.id());
            memory.income_energy = task.energy;
            return Ok(());
        };

        let task = tasks
            .get_mut(task_id)
            .ok_or(anyhow!("missing task id {task_id}"))?;

        match task {
            Task::EnergyRequire(task) => {
                task.energy += required_energy;
                memory.income_energy = task.energy;
            }
            _ => bail!("unmatched task type {task:?}"),
        }

        Ok(())
    }

    fn run(&self, memory: &mut Memory) -> Result<()> {
        todo!()
    }
}

pub(crate) fn run(spawn: &StructureSpawn, memory: &mut Memory) -> Result<()> {
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
    memory.creeps.insert(structure.name, structure.memory);
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
