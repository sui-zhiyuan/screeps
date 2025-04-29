use crate::impl_caster;
use crate::task::{Task, TaskId, TaskTrait};
use screeps::{ObjectId, Source, StructureSpawn};

#[derive(Debug)]
pub struct BasicHarvestTask {
    id: TaskId,
    source: ObjectId<Source>,
    spawn: ObjectId<StructureSpawn>,
}

impl_caster!(BasicHarvestTask, BasicHarvest);

impl TaskTrait for BasicHarvestTask {
    fn task_id(&self) -> TaskId {
        self.id
    }
}

// impl Tasks {
//     pub fn new_energy_require(
//         &mut self,
//         target: &StructureSpawn,
//         energy: i32,
//     ) -> &EnergyRequireTask {
//         let id = self.available_id();
//         let task = EnergyRequireTask {
//             id,
//             target: target.id(),
//             energy,
//         };
//         self.insert(Task::EnergyRequire(task));
//         self.get(id)
//             .expect("insert just now")
//             .cast()
//             .expect("insert just now, should be no mismatch")
//     }
// }
