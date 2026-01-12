use crate::actor::spawn_actor::SpawnActor;
use crate::actor::{Actor, SpawnMemory};
use crate::memory::Memory;
use crate::task::Tasks;
use anyhow::Result;
use screeps::game;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RoomMemory {}

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

impl Actor for RoomActor {
    fn name(&self) -> String {
        self.prototype.name().to_string()
    }

    fn assign(&mut self, tasks: &mut Tasks) -> Result<()> {
        Ok(())
    }

    fn run(&mut self, tasks: &Tasks) -> Result<()> {
        Ok(())
    }

    fn store_memory(&self, memory: &mut Memory) -> Result<()> {
        Ok(())
    }
}
