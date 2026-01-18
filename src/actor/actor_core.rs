use crate::actor::RoomActor;
use crate::actor::spawn_actor::SpawnActor;
use crate::common::{EnumDispatcher, EnumDowncast, enum_downcast};
use crate::memory::Memory;
use crate::task::Tasks;
use anyhow::Result;
use enum_dispatch::enum_dispatch;

pub struct Actors {
    actors: Vec<Actor>,
}

impl Actors {
    pub fn build_actors(memory: &Memory) -> Result<Actors> {
        let mut actors = Actors { actors: Vec::new() };
        let spawns = SpawnActor::build_actors(&memory.spawns)?;
        actors
            .actors
            .extend(spawns.into_iter().map(Actor::Spawn));

        let rooms = RoomActor::build_actors(&memory.rooms)?;
        actors
            .actors
            .extend(rooms.into_iter().map(Actor::Room));
        Ok(actors)
    }

    pub fn run(&mut self, tasks: &mut Tasks) -> Result<()> {
        for a in self.actors.iter_mut() {
            a.assign(tasks)?
        }

        for a in self.actors.iter_mut() {
            a.run(tasks)?
        }

        Ok(())
    }

    pub fn store_memory(&self, memory: &mut Memory) -> Result<()> {
        for a in self.actors.iter() {
            a.store_memory(memory)?;
        }
        Ok(())
    }

    pub fn iter<'a, T: EnumDowncast<Actor> + 'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.actors.iter().filter_map(|actor| actor.downcast_ref())
    }
}

#[enum_dispatch]
pub trait ActorTrait: Sized {
    fn name(&self) -> String;
    fn assign(&mut self, tasks: &mut Tasks) -> Result<()>;
    fn run(&mut self, tasks: &Tasks) -> Result<()>;
    fn store_memory(&self, memory: &mut Memory) -> Result<()>;
}

#[enum_dispatch(ActorTrait)]
pub enum Actor {
    Room(RoomActor),
    Spawn(SpawnActor),
}

impl EnumDispatcher for Actor {}
enum_downcast!(Actor, Room, RoomActor);
enum_downcast!(Actor, Spawn, SpawnActor);
