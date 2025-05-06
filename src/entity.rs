use anyhow::{Result, anyhow};
use screeps::{Creep, Flag, ObjectId, Room, Source, StructureSpawn, find, game, SharedCreepProperties};
use std::collections::HashMap;
use std::hash::Hash;

pub struct Entities {
    rooms: Vec<Room>,
    spawns: Vec<StructureSpawn>,
    creeps: Vec<Creep>,
    flags: Vec<Flag>,
    source: Vec<Source>,

    room_name: HashMap<String, usize>,
}

impl Entities {
    pub fn new() -> Self {
        let rooms = game::rooms().values().collect::<Vec<_>>();
        let spawns = game::spawns().values().collect::<Vec<_>>();
        let creeps = game::creeps().values().collect::<Vec<_>>();
        let flags = game::flags().values().collect::<Vec<_>>();
        let source = rooms
            .iter()
            .map(|v| v.find(find::SOURCES, None))
            .flatten()
            .collect::<Vec<_>>();

        let room_name = Self::get_map(&rooms, |r| r.name().to_string());
        let spawn_name = Self::get_map(&spawns, |s| s.name().to_string());
        let creep_name = Self::get_map(&creeps, |c| c.name().to_string());
        let flag_name = Self::get_map(&flags, |f| f.name().to_string());
        let source_name = Self::get_map(&source, |s| s.name().to_string());

        Entities {
            rooms,
            spawns,
            creeps,
            flags,
            source,

            room_name,
        }
    }

    pub fn get_by_name<T>(&self, name: &str) -> Result<&T>
    where
        Self: EntitiesStore<T>,
    {
        EntitiesStore::get_by_name(self, name)
            .ok_or_else(|| anyhow!("no such entity, name: {}", name))
    }

    pub fn get<T>(&self, id: ObjectId<T>) -> Result<&T>
    where
        Self: EntitiesStoreId<T>,
    {
        EntitiesStoreId::get(self, id).ok_or_else(|| anyhow!("no such entity, id: {}", id))
    }

    pub fn iter<T>(&self) -> impl Iterator<Item = &T>
    where
        Self: EntitiesStore<T>,
        T: 'static,
    {
        EntitiesStore::iter(self)
    }

    fn get_map<TKey, TValue, F>(source: &Vec<TValue>, mut f: F) -> HashMap<TKey, usize>
    where
        TKey: Eq + Hash,
        TValue: Clone,
        F: FnMut(&TValue) -> TKey,
    {
        source.iter().enumerate().map(|(i, r)| (f(r), i)).collect()
    }
}

pub trait EntitiesStore<T> {
    fn get_by_name(&self, name: &str) -> Option<&T>;
    fn iter(&self) -> impl Iterator<Item = &T>
    where
        T: 'static;
}

trait EntitiesStoreId<T>: EntitiesStore<T> {
    fn get(&self, id: ObjectId<T>) -> Option<&T>;
}

impl EntitiesStore<Room> for Entities {
    fn get_by_name(&self, name: &str) -> Option<&Room> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = &Room> {
        self.rooms.iter()
    }
}

impl EntitiesStore<StructureSpawn> for Entities {
    fn get_by_name(&self, name: &str) -> Option<&StructureSpawn> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = &StructureSpawn> {
        self.spawns.iter()
    }
}

impl EntitiesStoreId<StructureSpawn> for Entities {
    fn get_by_id(&self, id: ObjectId<StructureSpawn>) -> Option<&StructureSpawn> {
        todo!()
    }
}

impl EntitiesStore<Creep> for Entities {
    fn get_by_name(&self, name: &str) -> Option<&Creep> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = &Creep> {
        self.creeps.iter()
    }
}

impl EntitiesStoreId<Creep> for Entities {
    fn get_by_id(&self, id: ObjectId<Creep>) -> Option<&Creep> {
        todo!()
    }
}

impl EntitiesStore<Flag> for Entities {
    fn get_by_name(&self, name: &str) -> Option<&Flag> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = &Flag> {
        self.flags.iter()
    }
}

impl EntitiesStore<Source> for Entities {
    fn get_by_name(&self, name: &str) -> Option<&Source> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = &Source>
    where
        Source: 'static,
    {
        self.source.iter()
    }
}

impl EntitiesStoreId<Source> for Entities {
    fn get_by_id(&self, id: ObjectId<Source>) -> Option<&Source> {
        todo!()
    }
}
