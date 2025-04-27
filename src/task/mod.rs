use screeps::{ConstructionSite, ObjectId, Source, StructureController, StructureSpawn};

pub enum Task {
    // requirements
    Energy(EnergyTask),
    // tasks
    BasicHarvester(BasicHarvesterTask),
    Upgrade(UpgradeTask),
    Build(BuildTask),
}

// require energy
pub struct EnergyTask {
    pub target: ObjectId<StructureSpawn>,
}

pub struct BasicHarvesterTask {
    source: ObjectId<Source>,
    spawn: ObjectId<StructureSpawn>,
}

pub struct UpgradeTask {
    spawn: ObjectId<StructureSpawn>,
    controller: ObjectId<StructureController>,
}

pub struct BuildTask {
    spawn: ObjectId<StructureSpawn>,
    target: Option<ObjectId<ConstructionSite>>,
}
