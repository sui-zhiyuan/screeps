use crate::common::{hash_map, hash_map_key};
use anyhow::Result;
use screeps::{RoomName, game};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RoomMemories {
    #[serde(flatten)]
    values: HashMap<String, RoomMemory>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RoomMemory {}

#[derive(Deserialize, Serialize, Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct RoomId(u16);

pub struct RoomActors {
    actors: HashMap<RoomId, RoomActor>,
}

pub struct RoomActor {
    prototype: screeps::Room,
    memory: RoomMemory,
}

impl RoomActors {
    pub fn build_actors(memories: &RoomMemories) -> Result<RoomActors> {
        let mut rooms = game::rooms()
            .entries()
            .map(|(name, room)| (RoomId::from(name), room))
            .collect::<HashMap<_, _>>();

        let mut actors = hash_map_key(
            &memories.values,
            |name| RoomId::from(name.as_str()),
            |id, memory| {
                let prototype = rooms.remove(&id)?;
                Some(RoomActor {
                    prototype,
                    memory: memory.clone(),
                })
            },
        );

        for (id, room) in rooms {
            actors.insert(
                id,
                RoomActor {
                    prototype: room,
                    memory: RoomMemory::default(),
                },
            );
        }

        Ok(RoomActors { actors })
    }

    pub fn store_memory(&self, memories: &mut RoomMemories) -> Result<()> {
        let values = hash_map(&self.actors, String::from, |actor| actor.memory.clone());
        *memories = RoomMemories { values };
        Ok(())
    }

    pub fn get(&self, room_id: &RoomId) -> Option<&RoomActor> {
        self.actors.get(room_id)
    }

    pub fn get_mut(&mut self, room_id: &RoomId) -> Option<&mut RoomActor> {
        self.actors.get_mut(room_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RoomId, &RoomActor)> {
        self.actors.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&RoomId, &mut RoomActor)> {
        self.actors.iter_mut()
    }
}

impl RoomActor {
    pub fn energy_available(&self) -> u32 {
        // TODO: deduct energy planned
        self.prototype.energy_available()
    }
}

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

impl From<&str> for RoomId {
    fn from(room_name: &str) -> Self {
        let room_name: RoomName = room_name.parse().unwrap();
        RoomId::from(room_name)
    }
}

impl From<RoomId> for String {
    fn from(room_id: RoomId) -> Self {
        let room_name: RoomName = room_id.into();
        room_name.to_string()
    }
}
