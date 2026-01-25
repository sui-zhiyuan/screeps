use crate::actor::{RoomActor, RoomActors, SpawnActor, SpawnActors};
use crate::common::{EnumDispatcher, enum_downcast};
use crate::memory::Memory;
use crate::task::Tasks;
use anyhow::Result;
use enum_dispatch::enum_dispatch;

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
        for (_, room) in self.room_actors.iter_mut() {
            room.assign(tasks)?;
        }

        for (_, spawn) in self.spawn_actors.iter_mut() {
            spawn.assign(tasks)?;
        }

        for (_, room) in self.room_actors.iter_mut() {
            room.run(tasks)?;
        }

        for (_, spawn) in self.spawn_actors.iter_mut() {
            spawn.run(tasks)?;
        }

        // TODO
        // for a in self.actors.iter_mut() {
        //     a.assign(tasks)?
        // }
        //
        // for a in self.actors.iter_mut() {
        //     a.run(tasks)?
        // }

        Ok(())
    }

    pub fn store_memory(&self, memory: &mut Memory) -> Result<()> {
        self.room_actors.store_memory(&mut memory.rooms)?;
        self.spawn_actors.store_memory(&mut memory.spawns)?;
        Ok(())
    }
}

#[enum_dispatch]
pub trait ActorTrait: Sized {
    fn assign(&mut self, tasks: &mut Tasks) -> Result<()>;
    fn run(&mut self, tasks: &Tasks) -> Result<()>;
}

// #[enum_dispatch(ActorTrait)]
// pub enum Actor {
//     Room(RoomActor),
//     Spawn(SpawnActor),
// }

//
// impl EnumDispatcher for Actor {}
// enum_downcast!(Actor, Room, RoomActor);
// enum_downcast!(Actor, Spawn, SpawnActor);
