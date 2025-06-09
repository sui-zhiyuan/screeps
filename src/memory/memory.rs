use crate::actor::{CreepMemory, RoomMemory};
use crate::memory::Memory;
use screeps::{Creep, Room, SharedCreepProperties};

pub trait MemoryAccessor<TEntity> {
    type TMemory: Clone;

    fn with<TR>(&self, e: &TEntity, f: impl FnOnce(&mut Self::TMemory) -> TR) -> TR;
    fn load(&self, e: &TEntity) -> Self::TMemory {
        self.with(e, |memory| memory.clone())
    }
    fn store(&self, e: &TEntity, memory: Self::TMemory) {
        self.with(e, |m| *m = memory)
    }
}

impl MemoryAccessor<Room> for Memory {
    type TMemory = RoomMemory;

    fn with<TR>(&self, e: &Room, f: impl FnOnce(&mut Self::TMemory) -> TR) -> TR {
        self.with(|memory| {
            let m = memory.rooms.entry(e.name()).or_default();
            f(m)
        })
    }
}

impl Memory {
    pub fn store_creep_memory(&self, creep_name: &str, m: CreepMemory) {
        self.with(|memory| {
            memory.creeps.insert(creep_name.to_string(), m);
        })
    }
}

impl MemoryAccessor<Creep> for Memory {
    type TMemory = CreepMemory;

    fn with<TR>(&self, e: &Creep, f: impl FnOnce(&mut Self::TMemory) -> TR) -> TR {
        self.with(|memory| {
            let m = memory.creeps.entry(e.name().to_string()).or_default();
            f(m)
        })
    }
}
