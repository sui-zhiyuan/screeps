use crate::actor::RoomActor;
use crate::actor::{Actors, RoomId};
use crate::common::{IdManager, hash_map, hash_map_key};
use crate::task::{Task, TaskId, Tasks};
use anyhow::{Result, anyhow};
use screeps::{Part, game};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
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

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SpawnMemories {
    #[serde(default)]
    id_manager: IdManager<SpawnId>,
    #[serde(flatten)]
    values: HashMap<String, SpawnMemory>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SpawnMemory {
    spawn_task: Option<TaskId>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct SpawnId(usize);

pub struct SpawnActors {
    id_manager: IdManager<SpawnId>,
    actors: HashMap<SpawnId, SpawnActor>,
}

pub struct SpawnActor {
    id: SpawnId,
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
                let prototype = spawns.remove(&id)?;
                let room = prototype.room().expect("missing room for spawn");
                Some(SpawnActor {
                    id,
                    prototype,
                    memory: memory.clone(),
                    room_id: RoomId::from(room.name()),
                })
            },
        );

        for (id, prototype) in spawns.into_iter() {
            let room = prototype.room().ok_or(anyhow!("missing room for spawn"))?;
            let actor = SpawnActor {
                id,
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
        let values = hash_map(&self.actors, String::from, |actor| actor.memory.clone());
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

    pub fn assign(actors: &mut Actors, tasks: &mut Tasks) -> Result<()> {
        let spawn_ids = actors
            .spawn_actors
            .actors
            .keys()
            .copied()
            .collect::<Vec<_>>();
        for spawn_id in spawn_ids {
            let curr_spawn = actors.spawn_actors.actors.get_mut(&spawn_id).unwrap();
            let curr_room = actors
                .room_actors
                .get_mut(&curr_spawn.room_id)
                .ok_or(anyhow!("room not found"))?;
            let Some(task) = tasks
                .iter_mut::<CreepSpawnTask>()
                .find(|t| t.room == curr_spawn.room_id)
            else {
                continue;
            };

            curr_spawn.assign_spawn_task(task, curr_room)?;
        }
        Ok(())
    }

    pub fn run(actors: &mut Actors, tasks: &mut Tasks) -> Result<()> {
        let spawn_ids = actors
            .spawn_actors
            .actors
            .keys()
            .copied()
            .collect::<Vec<_>>();
        for spawn_id in spawn_ids {
            let curr_spawn = actors.spawn_actors.actors.get_mut(&spawn_id).unwrap();
            let Some(spawn_task_id) = curr_spawn.memory.spawn_task else {
                continue;
            };
            let task = tasks.get_mut::<CreepSpawnTask>(spawn_task_id)?;
            curr_spawn.run_spawn_task(task)?;
        }
        Ok(())
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

impl SpawnActor {
    fn assign_spawn_task(
        &mut self,
        task: &mut CreepSpawnTask,
        room: &mut RoomActor,
    ) -> Result<bool> {
        let body = [Part::Carry, Part::Work, Part::Move, Part::Move];
        let cost = body.iter().map(|p| p.cost()).sum();
        if room.energy_available() < cost {
            info!("not enough energy");
            return Ok(false);
        }

        task.spawn = Some(self.id);
        self.memory.spawn_task = Some(task.id);

        Ok(true)
    }

    fn run_spawn_task(&mut self, task: &mut CreepSpawnTask) -> Result<()> {
        if self.prototype.spawning().is_some() {
            let task_id = self
                .memory
                .spawn_task
                .ok_or(anyhow!("should be task running"))?;
            info!("spawning...");
            return Ok(());
        }

        let body = vec![Part::Carry, Part::Work, Part::Move, Part::Move];

        info!("creating");
        self.prototype.spawn_creep(&body, "t1")?;

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

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CreepSpawnTask {
    id: TaskId,
    room: RoomId,
    creep_class: CreepClass,
    spawn: Option<SpawnId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum CreepClass {
    Worker,
}

impl CreepSpawnTask {
    pub fn new_task(id: TaskId, room: RoomId, creep_class: CreepClass) -> Task {
        Task::CreepSpawn(CreepSpawnTask {
            id,
            room,
            creep_class,
            spawn: None,
        })
    }
}
