use crate::actor::CreepClass;
use crate::impl_caster;
use crate::task::{Task, TaskId, TaskTrait, Tasks};
use screeps::{ObjectId, Room, Source, StructureSpawn};

#[derive(Debug)]
pub struct CreepSpawnTask {
    id: TaskId,
    room_name: String,
    creep_class: CreepClass,
}

impl_caster!(CreepSpawnTask, CreepSpawn);

impl TaskTrait for CreepSpawnTask {
    fn task_id(&self) -> TaskId {
        self.id
    }
}

impl Tasks {
    pub fn new_creep_spawn(&mut self, room: &Room, creep_class: CreepClass) -> &CreepSpawnTask {
        let id = self.available_id();
        let task = CreepSpawnTask {
            id,
            room_name: room.name().to_string(),
            creep_class,
        };
        self.insert(Task::CreepSpawn(task));
        self.get(id)
            .expect(&format!("CreepSpawnTask {} not found", id))
    }
}
