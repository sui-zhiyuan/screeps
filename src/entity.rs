use screeps::{Creep, StructureSpawn, game};

pub struct Entities {
    pub spawns: Vec<StructureSpawn>,
    pub creeps: Vec<Creep>,
}

impl Entities {
    pub fn new() -> Self {
        let spawns = game::spawns().values().collect::<Vec<_>>();

        let creeps = game::creeps().values().collect::<Vec<_>>();

        Entities { spawns, creeps }
    }
}
