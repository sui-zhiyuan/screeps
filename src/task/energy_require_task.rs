use crate::impl_caster;
use crate::task::{Caster, Task, TaskId, TaskTrait, Tasks};
use screeps::{HasId, ObjectId, StructureSpawn};

// require energy
#[derive(Debug)]
pub struct EnergyRequireTask {
    id: TaskId,
    pub target: ObjectId<StructureSpawn>,
    pub energy: i32,
}

impl_caster!(EnergyRequireTask, EnergyRequire);

impl TaskTrait for EnergyRequireTask {
    fn task_id(&self) -> TaskId {
        self.id
    }
}

impl Tasks {
    pub fn new_energy_require(
        &mut self,
        target: &StructureSpawn,
        energy: i32,
    ) -> &EnergyRequireTask {
        let id = self.available_id();
        let task = EnergyRequireTask {
            id,
            target: target.id(),
            energy,
        };
        self.insert(Task::EnergyRequire(task));
        self.get(id)
            .expect("insert just now")
            .cast()
            .expect("insert just now, should be no mismatch")
    }
}
