use screeps::{Creep, SharedCreepProperties, StructureSpawn, game};
use std::collections::HashMap;

pub struct Entities {
    pub spawns: HashMap<String, StructureSpawn>,
    pub creeps: HashMap<String, Creep>,
}

impl Entities {
    pub fn new() -> Self {
        let spawns = game::spawns()
            .values()
            .map(|v| (v.name(), v))
            .collect::<HashMap<_, _>>();

        let creeps = game::creeps()
            .values()
            .map(|v| (v.name(), v))
            .collect::<HashMap<_, _>>();

        Entities { spawns, creeps }
    }
}
