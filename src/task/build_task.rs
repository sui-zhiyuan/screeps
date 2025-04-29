use crate::impl_caster;
use crate::task::{TaskId, TaskTrait};
use screeps::{ConstructionSite, ObjectId, StructureSpawn};

#[derive(Debug)]
pub struct BuildTask {
    id: TaskId,
    #[allow(dead_code)]
    spawn: ObjectId<StructureSpawn>,
    #[allow(dead_code)]
    target: Option<ObjectId<ConstructionSite>>,
}

impl_caster!(BuildTask, Build);

impl TaskTrait for BuildTask {
    fn task_id(&self) -> TaskId {
        self.id
    }
}
