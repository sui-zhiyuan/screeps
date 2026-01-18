use crate::actor::RoomActor;
use crate::actor::spawn_actor::SpawnActor;
use crate::common::{EnumDispatcher, EnumDowncast, enum_downcast};
use crate::memory::Memory;
use crate::task::{Task, Tasks};
use enum_dispatch::enum_dispatch;
use screeps::Creep;
use tracing::error;

pub struct Actors {
    actors: Vec<ActorDispatch>,
}

impl Actors {
    pub fn build_actors(memory: &Memory) -> anyhow::Result<Actors> {
        let mut actors = Actors { actors: Vec::new() };
        let spawns = SpawnActor::build_actors(&memory.spawns)?;
        actors
            .actors
            .extend(spawns.into_iter().map(ActorDispatch::Spawn));

        let rooms = RoomActor::build_actors(&memory.rooms)?;
        actors
            .actors
            .extend(rooms.into_iter().map(ActorDispatch::Room));
        Ok(actors)
    }

    pub fn run(&mut self, tasks: &mut Tasks) -> anyhow::Result<()> {
        for a in self.actors.iter_mut() {
            if let Err(e) = a.assign(tasks) {
                error!("plan error on {} : {}", a.name(), e);
            };
        }

        for a in self.actors.iter_mut() {
            if let Err(e) = a.run(tasks) {
                error!("run error on {} : {}", a.name(), e);
            };
        }

        Ok(())
    }

    pub fn store_memory(&self, memory: &mut Memory) -> anyhow::Result<()> {
        for a in self.actors.iter() {
            a.store_memory(memory)?;
        }
        Ok(())
    }

    pub fn iter<'a, T: EnumDowncast<ActorDispatch> + 'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.actors.iter().filter_map(|actor| actor.downcast_ref())
    }
}

trait CreepMemoryTrait {
    fn run(&mut self, creep: &Creep) -> anyhow::Result<()>;
}

#[enum_dispatch]
pub trait Actor: Sized {
    fn name(&self) -> String;
    fn assign(&mut self, tasks: &mut Tasks) -> anyhow::Result<()>;
    fn run(&mut self, tasks: &Tasks) -> anyhow::Result<()>;
    fn store_memory(&self, memory: &mut Memory) -> anyhow::Result<()>;
}

#[enum_dispatch(Actor)]
pub enum ActorDispatch {
    Room(RoomActor),
    Spawn(SpawnActor),
}

impl EnumDispatcher for ActorDispatch {}
enum_downcast!(ActorDispatch, Room, RoomActor);
enum_downcast!(ActorDispatch, Spawn, SpawnActor);
