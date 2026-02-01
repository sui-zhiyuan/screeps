use crate::actor::RoomActor;
use crate::actor::{Actors, RoomId};
use crate::common::{IdManager, hash_map, hash_map_key};
use crate::task::{Task, TaskId, Tasks};
use anyhow::{Result, anyhow, bail};
use screeps::{Part, RoomObjectProperties, game};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread::spawn;
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

    pub fn iter(&self) -> impl Iterator<Item = &SpawnActor> {
        self.actors.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SpawnActor> {
        self.actors.values_mut()
    }

    pub fn run(actors: &mut Actors, tasks: &mut Tasks) -> Result<()> {
        let Actors {
            room_actors,
            spawn_actors,
        } = actors;
        for curr_spawn in spawn_actors.iter_mut() {
            curr_spawn.clean_up(tasks)?;
        }
        for curr_spawn in spawn_actors.iter_mut() {
            let curr_room = room_actors
                .get_mut(&curr_spawn.room_id)
                .ok_or(anyhow!("room not found"))?;
            for curr_task in tasks.iter_mut::<CreepSpawnTask>() {
                if curr_spawn.run_spawn_task(curr_task, curr_room)? {
                    break;
                }
            }
        }
        Ok(())
    }
}

impl SpawnActor {
    fn clean_up(&mut self, tasks: &mut Tasks) -> Result<()> {
        if let Some(spawn_task_id) = self.memory.spawn_task
            && self.prototype.spawning().is_none()
        {
            info!("Spawned {}", self.prototype.name());
            let task = tasks.get_mut::<CreepSpawnTask>(spawn_task_id)?;
            task.spawn = None;
            self.memory.spawn_task = None;
            tasks.remove_task(spawn_task_id)?;
        }

        Ok(())
    }

    fn run_spawn_task(&mut self, task: &mut CreepSpawnTask, room: &mut RoomActor) -> Result<bool> {
        if task.room_id != self.room_id || task.spawn.is_some() || self.memory.spawn_task.is_some()
        {
            return Ok(false);
        }
        if self.prototype.spawning().is_some() {
            bail!("spawn is busy");
        }

        let body = [Part::Carry, Part::Work, Part::Move, Part::Move];
        let cost = body.iter().map(|p| p.cost()).sum();
        if room.energy_available() < cost {
            info!("not enough energy");
            return Ok(false);
        }

        task.spawn = Some(self.id);
        self.memory.spawn_task = Some(task.id);
        info!("creating");
        self.prototype.spawn_creep(&body, "t1")?;

        Ok(true)
    }
}

impl From<&str> for SpawnId {
    fn from(value: &str) -> Self {
        let s = value.strip_prefix('S').expect("invalid spawn id");
        let id: usize = s.parse().unwrap();
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
    room_id: RoomId,
    creep_class: CreepClass,
    spawn: Option<SpawnId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum CreepClass {
    Worker,
}

impl CreepSpawnTask {
    pub fn new_task(id: TaskId, room_id: RoomId, creep_class: CreepClass) -> Task {
        Task::CreepSpawn(CreepSpawnTask {
            id,
            room_id,
            creep_class,
            spawn: None,
        })
    }
}
