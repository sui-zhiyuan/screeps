use crate::impl_caster;
use crate::task::{TaskId, TaskTrait};
use screeps::{ObjectId, StructureController, StructureSpawn};

#[derive(Debug)]
pub struct UpgradeTask {
    id: TaskId,
    #[allow(dead_code)]
    spawn: ObjectId<StructureSpawn>,
    #[allow(dead_code)]
    controller: ObjectId<StructureController>,
}

impl_caster!(UpgradeTask, Upgrade);

impl TaskTrait for UpgradeTask {
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
