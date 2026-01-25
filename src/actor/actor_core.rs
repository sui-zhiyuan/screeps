use crate::actor::{RoomActors, SpawnActors};
use crate::memory::Memory;
use crate::task::Tasks;
use anyhow::Result;

pub struct Actors {
    pub room_actors: RoomActors,
    pub spawn_actors: SpawnActors,
}

impl Actors {
    pub fn build_actors(memory: &Memory) -> Result<Actors> {
        let room_actors = RoomActors::build_actors(&memory.rooms)?;
        let spawn_actors = SpawnActors::build_actors(&memory.spawns)?;

        Ok(Actors {
            room_actors,
            spawn_actors,
        })
    }

    pub fn run(&mut self, tasks: &mut Tasks) -> Result<()> {
        SpawnActors::assign(self, tasks)?;
        SpawnActors::run(self, tasks)?;
        Ok(())
    }

    pub fn store_memory(&self, memory: &mut Memory) -> Result<()> {
        self.room_actors.store_memory(&mut memory.rooms)?;
        self.spawn_actors.store_memory(&mut memory.spawns)?;
        Ok(())
    }
}
