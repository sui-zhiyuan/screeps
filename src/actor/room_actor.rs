use crate::actor::ActorTrait;
use crate::memory::Memory;
use crate::task::Tasks;
use anyhow::Result;
use js_sys::JsString;
use screeps::{RoomName, game};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RoomMemories {
    actors: HashMap<String, RoomMemory>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RoomMemory {}

#[derive(Deserialize, Serialize, Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct RoomId(u16);

pub struct RoomActors {
    actors: HashMap<RoomId, RoomActor>,
}

pub struct RoomActor {
    prototype: Option<screeps::Room>,
    memory: RoomMemory,
}

impl RoomActors {
    pub fn build_actors(memories: &HashMap<String, RoomMemory>) -> Result<RoomActors> {
        let mut room_actors = RoomActors {
            actors: HashMap::new(),
        };
        let mut rooms = game::rooms()
            .entries()
            .map(|(name, room)| (name.to_string(), room))
            .collect::<HashMap<_, _>>();

        for (name, memory) in memories {
            let room_prototype = rooms.remove(name);
            let room_name_js: JsString = name.clone().into();
            let room_name: RoomName = room_name_js.try_into().expect("unknown room name");
            let room_id = room_name.into();
            room_actors.actors.insert(
                room_id,
                RoomActor {
                    prototype: room_prototype,
                    memory: memory.clone(),
                },
            );
        }

        for (_, room) in rooms {
            room_actors.actors.insert(
                room.name().into(),
                RoomActor {
                    prototype: Some(room),
                    memory: RoomMemory::default(),
                },
            );
        }

        Ok(room_actors)
    }

    pub fn store_memory(&self, memories: &mut HashMap<String, RoomMemory>) -> Result<()> {
        for (room_id, actor) in self.actors.iter() {
            let room_name: RoomName = (*room_id).into();
            memories.insert(room_name.to_string(), actor.memory.clone());
        }
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RoomId, &RoomActor)> {
        self.actors.iter()
    }
}

impl RoomActor {}

impl From<RoomName> for RoomId {
    fn from(room_name: RoomName) -> Self {
        RoomId(room_name.packed_repr())
    }
}

impl From<RoomId> for RoomName {
    fn from(room_id: RoomId) -> Self {
        RoomName::from_packed(room_id.0)
    }
}

impl ActorTrait for RoomActor {
    fn assign(&mut self, _tasks: &mut Tasks) -> Result<()> {
        Ok(())
    }

    fn run(&mut self, _tasks: &Tasks) -> Result<()> {
        Ok(())
    }

    fn store_memory(&self, _memory: &mut Memory) -> Result<()> {
        Ok(())
    }
}
