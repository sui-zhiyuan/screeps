use crate::actor::ActorTrait;
use crate::memory::Memory;
use crate::task::Tasks;
use anyhow::Result;
use screeps::game;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
pub struct RoomId(usize);

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RoomMemory {
    id: RoomId,
    name: String,
}

pub struct RoomActor {
    prototype: screeps::Room,
    memory: RoomMemory,
}

impl RoomActor {
    pub fn build_actors(memories: &HashMap<String, RoomMemory>) -> Result<Vec<RoomActor>> {
        let rooms = game::rooms().values();
        let mut actors = Vec::new();

        for room in rooms {
            let memory = memories
                .get(&room.name().to_string())
                .cloned()
                .unwrap_or_default();
            actors.push(RoomActor {
                prototype: room.clone(),
                memory,
            });
        }

        Ok(actors)
    }
}

impl ActorTrait for RoomActor {
    fn name(&self) -> String {
        self.prototype.name().to_string()
    }

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
