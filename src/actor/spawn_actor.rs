use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::mem;
use std::str::FromStr;
// use crate::actor::Actor;
// use crate::actor::creep_actor::CreepMemory;
// use crate::actor::creep_builder::CreepBuilderMemory;
// use crate::actor::creep_harvester::CreepHarvesterMemory;
// use crate::actor::creep_upgrader::CreepUpgraderMemory;
// use crate::context::Context;
// use anyhow::{Result, anyhow};
use crate::actor::{ActorTrait, RoomActors, RoomId, RoomMemory};
use crate::common;
use crate::common::{IdManager, hash_map, hash_map_key};
use crate::memory::Memory;
use crate::task::{Task, TaskId, Tasks};
use anyhow::{Result, anyhow, ensure};
use screeps::{Part, game};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
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

//
// impl Display for CreepSpawnTask {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Spawn creep {:?} in {}", self.creep_class, self.room)
//     }
// }
//

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SpawnMemories {
    #[serde(default)]
    id_manager: IdManager<SpawnId>,
    #[serde(flatten)]
    values: HashMap<String, SpawnMemory>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SpawnMemory {
    spawn_task: Option<TaskId>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct SpawnId(usize);

pub struct SpawnActors {
    id_manager: IdManager<SpawnId>,
    actors: HashMap<SpawnId, SpawnActor>,
}

pub struct SpawnActor {
    prototype: screeps::StructureSpawn,
    memory: SpawnMemory,
    room_id: RoomId,
}

impl SpawnActors {
    pub fn build_actors(memories: &SpawnMemories) -> Result<SpawnActors> {
        let mut spawns = game::spawns()
            .entries()
            .map(|(key, value)| (SpawnId::from(key.as_str()), value))
            .collect::<HashMap<_, _>>();

        let mut actors = hash_map_key(
            &memories.values,
            |name| SpawnId::from(name.as_str()),
            |id, memory| {
                let Some(prototype) = spawns.remove(&id) else {
                    return None;
                };
                let room = prototype.room().expect("missing room for spawn");
                Some(SpawnActor {
                    prototype,
                    memory: memory.clone(),
                    room_id: RoomId::from(room.name()),
                })
            },
        );

        for (id, prototype) in spawns.into_iter() {
            let room = prototype.room().ok_or(anyhow!("missing room for spawn"))?;
            let actor = SpawnActor {
                prototype,
                memory: SpawnMemory::default(),
                room_id: RoomId::from(room.name()),
            };
            actors.insert(id, actor);
        }

        Ok(SpawnActors {
            id_manager: memories.id_manager.clone(),
            actors,
        })
    }

    pub fn store_memory(&self, memories: &mut SpawnMemories) -> Result<()> {
        let values = hash_map(
            &self.actors,
            |id| String::from(id),
            |actor| actor.memory.clone(),
        );
        *memories = SpawnMemories {
            id_manager: self.id_manager.clone(),
            values,
        };
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SpawnId, &SpawnActor)> {
        self.actors.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&SpawnId, &mut SpawnActor)> {
        self.actors.iter_mut()
    }

    // fn add_spawn(&mut self, spawn: SpawnActor) -> Result<SpawnId> {
    //     match self.alloc_id() {
    //         NewIdResult::NewId => {
    //             let id = self.actors.len();
    //             self.actors.push(spawn);
    //             Ok(SpawnId(id))
    //         }
    //         NewIdResult::ReusedId(id) => {
    //             self.actors[id] = spawn;
    //             Ok(SpawnId(id))
    //         }
    //     }
    // }

    // fn remove_spawn(&mut self, id: SpawnId) -> Result<SpawnActor> {
    //     let mut spawn = SpawnActor::Tombstone(self.free_id(id.0));
    //     mem::swap(&mut spawn, &mut self.actors[id.0]);
    //     Ok(spawn)
    // }
}

impl SpawnActor {}

impl ActorTrait for SpawnActor {
    fn assign(&mut self, tasks: &mut Tasks) -> Result<()> {
        // TODO lazy get curr_room;
        let curr_room = self
            .prototype
            .room()
            .ok_or(anyhow!("no room found for creep"))?;
        let Some((task_id, task)) = tasks
            .iter_mut::<CreepSpawnTask>()
            .find(|(_, t)| t.room == curr_room.name().into())
        else {
            return Ok(());
        };

        let body = [Part::Carry, Part::Work, Part::Move, Part::Move];
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
}

impl From<&str> for SpawnId {
    fn from(value: &str) -> Self {
        let s = value.strip_prefix('S').expect("invalid spawn id");
        let id: usize = s.trim_start_matches('0').parse().unwrap();
        SpawnId(id)
    }
}

impl From<SpawnId> for String {
    fn from(value: SpawnId) -> Self {
        format!("S{:03}", value.0)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreepSpawnTask {
    room: RoomId,
    creep_class: CreepClass,
    spawn: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum CreepClass {
    Worker,
}

impl CreepSpawnTask {
    pub fn new_task(room: RoomId, creep_class: CreepClass) -> Task {
        Task::CreepSpawn(CreepSpawnTask {
            room,
            creep_class,
            spawn: None,
        })
    }
}
