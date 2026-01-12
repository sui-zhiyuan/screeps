use std::collections::HashMap;
// use crate::actor::Actor;
// use crate::actor::creep_actor::CreepMemory;
// use crate::actor::creep_builder::CreepBuilderMemory;
// use crate::actor::creep_harvester::CreepHarvesterMemory;
// use crate::actor::creep_upgrader::CreepUpgraderMemory;
// use crate::context::Context;
// use anyhow::{Result, anyhow};
use crate::actor::{Actor, RoomMemory};
use crate::memory::Memory;
use crate::task::{NoTask, Task, TaskId, Tasks};
use anyhow::{Result, anyhow, ensure};
use screeps::{Part, Room, RoomName, StructureSpawn, find, game};
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::common::EnumDowncast;

// use std::fmt::Display;
// use tracing::info;
//
//
// struct CreepStructure {
//     name: String,
//     body: Vec<Part>,
//     // memory: CreepMemory,
// }
//
// impl CreepStructure {
//     fn new_harvest(spawn: &StructureSpawn) -> Result<CreepStructure> {
//         let name_base = game::time();
//         let name = format!("{name_base}-0");
//
//         let room = spawn.room().ok_or(anyhow!("room not found"))?;
//         let sources = room.find(find::SOURCES, None);
//         let source = sources.first().ok_or(anyhow!("no source found"))?;
//
//         let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];
//         let memory = CreepHarvesterMemory::new_memory(source, spawn);
//
//         Ok(CreepStructure { name, body, memory })
//     }
//
//     fn new_builder(spawn: &StructureSpawn) -> Result<CreepStructure> {
//         let name_base = game::time();
//         let name = format!("{name_base}-0");
//
//         let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];
//         let memory = CreepBuilderMemory::new_memory(spawn);
//
//         Ok(CreepStructure { name, body, memory })
//     }
//
//     fn new_upgrader(spawn: &StructureSpawn) -> Result<CreepStructure> {
//         let name_base = game::time();
//         let name = format!("{name_base}-0");
//
//         let room = spawn.room().ok_or(anyhow!("room not found"))?;
//         let controller = room.controller().ok_or(anyhow!("controller not found"))?;
//
//         let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];
//         let memory = CreepUpgraderMemory::new_memory(spawn, &controller);
//
//         Ok(CreepStructure { name, body, memory })
//     }
//
//     fn cost(&self) -> u32 {
//         self.body.iter().map(|p| p.cost()).sum()
//     }
// }
//

//
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreepSpawnTask {
    room: String,
    creep_class: CreepClass,
    spawn: Option<String>,
}

impl EnumDowncast<Task> for CreepSpawnTask {
    fn enum_downcast(from: Task) -> Option<Self> {
        match from {
            Task::CreepSpawn(item) => Some(item),
            _ => None,
        }
    }
    fn enum_downcast_ref(from: &Task) -> Option<&Self> {
        match from {
            Task::CreepSpawn(item) => Some(item),
            _ => None,
        }
    }
    fn enum_downcast_mut(from: &mut Task) -> Option<&mut Self> {
        match from {
            Task::CreepSpawn(item) => Some(item),
            _ => None,
        }
    }
}

impl CreepSpawnTask {
    pub fn new_task(room: String, creep_class: CreepClass) -> Task {
        Task::CreepSpawn(CreepSpawnTask {
            room,
            creep_class,
            spawn: None,
        })
    }
}
//
// impl Display for CreepSpawnTask {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Spawn creep {:?} in {}", self.creep_class, self.room)
//     }
// }
//
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum CreepClass {
    Worker,
}

pub struct SpawnActor {
    prototype: screeps::StructureSpawn,
    memory: SpawnMemory,
    // TODO as room actor
    // room: Room,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SpawnMemory {
    spawn_task: Option<TaskId>,
}

impl SpawnActor {
    pub fn build_actors(memories: &HashMap<String, SpawnMemory>) -> Result<Vec<SpawnActor>> {
        let spawns = game::spawns().values();
        let mut actors = Vec::new();

        for spawn in spawns {
            let memory = memories.get(&spawn.name()).cloned().unwrap_or_default();
            actors.push(SpawnActor {
                prototype: spawn.clone(),
                memory,
            });
        }

        Ok(actors)
    }
}

impl Actor for SpawnActor {
    fn name(&self) -> String {
        self.prototype.name().to_string()
    }

    fn assign(&mut self, tasks: &mut Tasks) -> Result<()> {
        // TODO lazy get curr_room;
        let curr_room = self
            .prototype
            .room()
            .ok_or(anyhow!("no room found for creep"))?;
        let Some((task_id, task)) = tasks
            .iter_mut::<CreepSpawnTask>()
            .find(|(_, t)| t.room == curr_room.name())
        else {
            return Ok(());
        };

        let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];
        let cost = body.iter().map(|p| p.cost()).sum();
        task.spawn = Some(self.prototype.name().to_string());
        self.memory.spawn_task = Some(task_id);

        if curr_room.energy_available() < cost {
            info!("not enough energy");
            return Ok(());
        }
        // TODO pre assign energy usage in current round

        Ok(())
    }

    fn run(&mut self, tasks: &Tasks) -> Result<()> {
        if self.prototype.spawning().is_some() {
            let task_id = self
                .memory
                .spawn_task
                .ok_or(anyhow!("should be task running"))?;
            ensure!(
                tasks.get::<CreepSpawnTask>(task_id).is_ok(),
                "task should exist"
            );
            info!("spawning...");
            return Ok(());
        }

        let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];

        info!("creating");
        self.prototype.spawn_creep(&body, "t1")?;
        // TODO handle creep memory
        // ctx.memory()
        //     .store_creep_memory(&structure.name, structure.memory);
        // Ok(())

        Ok(())
    }

    fn store_memory(&self, memory: &mut Memory) -> Result<()> {
        memory
            .spawns
            .insert(self.prototype.name(), self.memory.clone());
        Ok(())
    }
}
