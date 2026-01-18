use enum_dispatch::enum_dispatch;
use screeps::Creep;
use tracing::error;
use crate::actor::RoomActor;
use crate::actor::spawn_actor::SpawnActor;
use crate::common::EnumDowncast;
use crate::memory::Memory;
use crate::task::Tasks;

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
        self.actors
            .iter()
            .filter_map(|actor| actor.downcast_ref().map(|t| t))
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
enum ActorDispatch {
    Room(RoomActor),
    Spawn(SpawnActor),
}

impl EnumDowncast<ActorDispatch> for RoomActor {
    fn enum_downcast(from: ActorDispatch) -> Option<Self> {
        match from {
            ActorDispatch::Room(item) => Some(item),
            _ => None,
        }
    }
    fn enum_downcast_ref(from: &ActorDispatch) -> Option<&Self> {
        match from {
            ActorDispatch::Room(item) => Some(item),
            _ => None,
        }
    }
    fn enum_downcast_mut(from: &mut ActorDispatch) -> Option<&mut Self> {
        match from {
            ActorDispatch::Room(item) => Some(item),
            _ => None,
        }
    }
}

impl EnumDowncast<ActorDispatch> for SpawnActor {
    fn enum_downcast(from: ActorDispatch) -> Option<Self> {
        match from {
            ActorDispatch::Spawn(item) => Some(item),
            _ => None,
        }
    }
    fn enum_downcast_ref(from: &ActorDispatch) -> Option<&Self> {
        match from {
            ActorDispatch::Spawn(item) => Some(item),
            _ => None,
        }
    }
    fn enum_downcast_mut(from: &mut ActorDispatch) -> Option<&mut Self> {
        match from {
            ActorDispatch::Spawn(item) => Some(item),
            _ => None,
        }
    }
}

impl ActorDispatch {
    fn downcast<T: EnumDowncast<Self>>(self) -> Option<T> {
        T::enum_downcast(self)
    }
    fn downcast_ref<T: EnumDowncast<Self>>(&self) -> Option<&T> {
        T::enum_downcast_ref(self)
    }
    fn downcast_mut<T: EnumDowncast<Self>>(&mut self) -> Option<&mut T> {
        T::enum_downcast_mut(self)
    }
}
