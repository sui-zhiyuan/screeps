use crate::actor::{CreepHarvesterMemory, CreepMemory, FlagMemory, RoomMemory, SpawnMemory};
use crate::entity::Entities;
use crate::entity::EntitiesStore;
use anyhow::{Result, anyhow, bail};
use js_sys::JsString;
use log::{error, info};
use screeps::{Creep, Flag, MaybeHasId, ObjectId, Room, StructureSpawn, raw_memory, HasId, SharedCreepProperties, Source};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Mutex, MutexGuard};

static MEMORY: Mutex<Option<Memory>> = Mutex::new(None);

pub struct Memory {
    pub rooms: HashMap<String, RoomMemory>,
    pub spawns: HashMap<ObjectId<StructureSpawn>, SpawnMemory>,
    pub creeps: HashMap<ObjectId<Creep>, CreepMemory>,
    pub flags: HashMap<String, FlagMemory>,
}

impl Memory {
    pub fn load(entities: &Entities) -> Result<(MutexGuard<'static, Option<Memory>>, Memory)> {
        let mut memory_lock = MEMORY
            .lock()
            .map_err(|e| anyhow!("memory lock err: {}", e))?;
        let memory = match memory_lock.take() {
            Some(memory) => memory,
            None => Self::load_memory(entities)?,
        };

        Ok((memory_lock, memory))
    }

    pub fn store(
        mut self,
        mut lock: MutexGuard<Option<Memory>>,
        entities: &Entities,
    ) -> Result<()> {
        // todo cleanup every 10 round
        self.clean_up_memory(entities);
        self.store_memory(entities)?;
        // write back memory
        _ = lock.insert(self);
        Ok(())
    }

    fn load_memory(entities: &Entities) -> Result<Memory> {
        info!("loading memory");
        let js_memory = raw_memory::get();
        let json_memory: String = js_memory.into();
        let memory: StoreMemory = serde_json::from_str(&json_memory)?;
        Ok(memory.parse(entities)?)
    }

    fn clean_up_memory(&mut self, entities: &Entities) {
        let creeps = entities
            .creeps
            .iter()
            .map(|v| v.try_id().unwrap())
            .collect::<HashSet<_>>();
        self.creeps.retain(|id, _| creeps.contains(id));
    }

    fn store_memory(&self, entities: &Entities) -> Result<()> {
        let store_memory = StoreMemory::pack(self, entities)?;
        let json_memory = serde_json::to_string(&store_memory)?;
        let js_memory = JsString::from(json_memory);
        raw_memory::set(&js_memory);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct StoreMemory {
    rooms: HashMap<String, StoreRoomMemory>,
    spawns: HashMap<String, StoreSpawnMemory>,
    creeps: HashMap<String, StoreCreepMemory>,
    flags: HashMap<String, StoreFlagMemory>,
}

impl StoreMemory {
    fn parse(&self, entities: &Entities) -> Result<Memory> {
        let rooms = Self::parse_inner::<StoreRoomMemory>(entities, entities.iter::<Room>(), &self.rooms)?;
        let spawns =
            Self::parse_inner::<StoreSpawnMemory>(entities, entities.iter::<StructureSpawn>(), &self.spawns)?;
        let creeps = Self::parse_inner::<StoreCreepMemory>(entities, entities.iter::<Creep>(), &self.creeps)?;
        let flags = Self::parse_inner::<StoreFlagMemory>(entities, entities.iter::<Flag>(), &self.flags)?;

        Ok(Memory {
            rooms,
            spawns,
            creeps,
            flags,
        })
    }

    fn pack(target: &Memory, entities: &Entities) -> Result<Self> {
        let rooms = Self::pack_inner::<StoreRoomMemory>(entities, entities.iter::<Room>(), &target.rooms)?;
        let spawns = Self::pack_inner::<StoreSpawnMemory>(
            entities,
            entities.iter::<StructureSpawn>(),
            &target.spawns,
        )?;
        let creeps =
            Self::pack_inner::<StoreCreepMemory>(entities, entities.iter::<Creep>(), &target.creeps)?;
        let flags = Self::pack_inner::<StoreFlagMemory>(entities, entities.iter::<Flag>(), &target.flags)?;

        Ok(Self {
            rooms,
            spawns,
            creeps,
            flags,
        })
    }
}

impl StoreMemory {
    fn parse_inner<'a, TStoreMemory>(
        entities: &Entities,
        targets: impl Iterator<Item=&'a TStoreMemory::Entity>,
        sources: &HashMap<TStoreMemory::StoreKey, TStoreMemory>,
    ) -> Result<HashMap<TStoreMemory::MemoryKey, TStoreMemory::Memory>>
    where
        TStoreMemory: StoreMemoryTrait,
        TStoreMemory::StoreKey: Hash + Eq,
        TStoreMemory::MemoryKey: Hash + Eq,
        TStoreMemory::Entity: 'static,
    {
        let values = targets.map(|entity| {
            let store_key = TStoreMemory::store_key(&entity);
            let store = sources.get(&store_key);
            let memory_key = TStoreMemory::memory_key(&entity);
            let memory = match store {
                None => TStoreMemory::new_memory(entities, &entity),
                Some(store) => store.parse(entities, &entity),
            };
            (memory_key, memory)
        });

        Self::check_errors(values)
    }

    fn pack_inner<'a, TStoreMemory>(
        entities: &Entities,
        targets: impl Iterator<Item=&'a TStoreMemory::Entity>,
        sources: &HashMap<TStoreMemory::MemoryKey, TStoreMemory::Memory>,
    ) -> Result<HashMap<TStoreMemory::StoreKey, TStoreMemory>>
    where
        TStoreMemory: StoreMemoryTrait,
        TStoreMemory::StoreKey: Hash + Eq,
        TStoreMemory::MemoryKey: Hash + Eq,
        TStoreMemory::Entity: 'static,
    {
        let values = targets.map(|entity| {
            let memory_key = TStoreMemory::memory_key(&entity);
            let memory = sources.get(&memory_key);
            let store_key = TStoreMemory::store_key(&entity);
            let store = match memory {
                None => Err(anyhow!("missing memory")),
                Some(memory) => TStoreMemory::pack(entities, memory, entity),
            };
            (store_key, store)
        });

        Self::check_errors(values)
    }

    fn check_errors<TKey, TValue>(
        input: impl Iterator<Item=(TKey, Result<TValue>)>,
    ) -> Result<HashMap<TKey, TValue>>
    where
        TKey: Hash + Eq,
    {
        let (oks, errs): (Vec<_>, Vec<_>) = input.partition(|v| v.1.is_ok());

        let oks = oks
            .into_iter()
            .map(|v| (v.0, v.1.unwrap()))
            .collect::<HashMap<_, _>>();
        let errs = errs
            .into_iter()
            .map(|v| (v.0, v.1.err().unwrap()))
            .collect::<Vec<_>>();
        for (_, error) in errs.iter() {
            error!("fail to parse memory {}", error);
        }

        if !errs.is_empty() {
            return Err(errs.into_iter().next().unwrap().1);
        }

        Ok(oks)
    }
}

pub trait StoreMemoryTrait: Sized {
    type Entity;
    type MemoryKey;
    type StoreKey;
    type Memory;

    fn memory_key(entity: &Self::Entity) -> Self::MemoryKey {
        todo!()
    }
    fn store_key(entity: &Self::Entity) -> Self::StoreKey {
        todo!()
    }

    fn new_memory(entities: &Entities, entity: &Self::Entity) -> Result<Self::Memory> {
        todo!()
    }
    fn parse(&self, entities: &Entities, entity: &Self::Entity) -> Result<Self::Memory> {
        todo!()
    }
    fn pack(entities: &Entities, memory: &Self::Memory, entitiy: &Self::Entity) -> Result<Self> {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
struct StoreRoomMemory {}

impl StoreMemoryTrait for StoreRoomMemory {
    type Entity = Room;
    type MemoryKey = String;
    type StoreKey = String;
    type Memory = RoomMemory;

    fn memory_key(entity: &Room) -> String {
        entity.name().to_string()
    }
    fn store_key(entity: &Room) -> String {
        entity.name().to_string()
    }

    fn new_memory(_: &Entities, entity: &Room) -> Result<RoomMemory> {
        Ok(RoomMemory::new())
    }
    fn parse(&self, _: &Entities, entitiy: &Room) -> Result<RoomMemory> {
        Ok(RoomMemory())
    }
    fn pack(_: &Entities, memory: &RoomMemory, entitiy: &Room) -> Result<StoreRoomMemory> {
        Ok(StoreRoomMemory {})
    }
}

#[derive(Serialize, Deserialize)]
struct StoreSpawnMemory {}

impl StoreMemoryTrait for StoreSpawnMemory {
    type Entity = StructureSpawn;
    type MemoryKey = ObjectId<StructureSpawn>;
    type StoreKey = String;
    type Memory = SpawnMemory;

    fn memory_key(entity: &StructureSpawn) -> ObjectId<StructureSpawn> {
        entity.id()
    }
    fn store_key(entity: &StructureSpawn) -> String {
        entity.name().to_string()
    }

    fn new_memory(_: &Entities, entity: &StructureSpawn) -> Result<SpawnMemory> {
        Ok(SpawnMemory {})
    }
    fn parse(&self, _: &Entities, entitiy: &StructureSpawn) -> Result<SpawnMemory> {
        Ok(SpawnMemory {})
    }
    fn pack(_: &Entities, memory: &SpawnMemory, entitiy: &StructureSpawn) -> Result<StoreSpawnMemory> {
        Ok(StoreSpawnMemory {})
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "role")]
enum StoreCreepMemory {
    Harvester { source: String, spawn: String },
    Upgrader { spawn: String, target: String , state: String},
    Builder { spawn: String, target: String },
}

impl StoreMemoryTrait for StoreCreepMemory {
    type Entity = Creep;
    type MemoryKey = ObjectId<Creep>;
    type StoreKey = String;
    type Memory = CreepMemory;

    fn memory_key(entity: &Creep) -> ObjectId<Creep> {
        entity.try_id().unwrap()
    }
    fn store_key(entity: &Creep) -> String {
        entity.name().to_string()
    }

    fn new_memory(_: &Entities, entity: &Creep) -> Result<CreepMemory> {
        bail!("missing memory")
    }
    fn parse(&self, entities: &Entities, _: &Creep) -> Result<CreepMemory> {
        let memory = match self {
            StoreCreepMemory::Harvester { source, spawn } => {
                let source = entities.get_by_name::<Source>(source).ok_or(anyhow!("missing source"))?;
                let spawn = entities.get_by_name::<StructureSpawn>(spawn).ok_or(anyhow!("missing spawn"))?;
                CreepMemory::Harvester(CreepHarvesterMemory{
                    source: source.id(),
                    spawn:spawn.id(),
                })
            },
            StoreCreepMemory::Upgrader {spawn , target, state}=> {
                todo!()
            },
            StoreCreepMemory::Builder {..} => {
                todo!()
            }
        };
        Ok(memory)
    }
    fn pack(entities: &Entities, memory: &CreepMemory, entitiy: &Creep) -> Result<StoreCreepMemory> {
        let store = match memory {
            CreepMemory::Harvester(harvester) => {
                let source = entities.get_by_id(harvester.source).ok_or(anyhow!("missing source"))?;
                let spawn = entities.get_by_id(harvester.spawn).ok_or(anyhow!("missing spawn"))?;
                StoreCreepMemory::Harvester {source: "todo".to_owned(), spawn: spawn.name()}
            }
            _ => {
                todo!()
            }
        };
        Ok(store)
    }
}

#[derive(Serialize, Deserialize)]
struct StoreFlagMemory {}

impl StoreMemoryTrait for StoreFlagMemory {
    type Entity = Flag;
    type MemoryKey = String;
    type StoreKey = String;
    type Memory = FlagMemory;
}
