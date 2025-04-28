use screeps::{ConstructionSite, HasId, ObjectId, Source, StructureController, StructureSpawn};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::fmt::Display;
use std::mem::swap;
use anyhow::{anyhow, Result};
use std::ops::IndexMut;

#[derive(
    Serialize, Deserialize, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone,
)]
pub struct TaskId(u64);

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TaskId {
    pub fn next(&self) -> TaskId {
        TaskId(self.0 + 1)
    }
}

#[derive(Default, Debug)]
pub struct Tasks {
    tasks: HashMap<TaskId, Task>,
    available_ids: BTreeSet<TaskId>,
    max_id: TaskId,
}

trait Getter<T>{
    fn get_inner(&self) -> &T;
}

impl Getter<EnergyRequireTask> for Task {
    fn get_inner(&self) -> &EnergyRequireTask {
        match self {
            Task::EnergyRequire(task) => task,
            _ => panic!(),

        }
    }
}

impl<T:TaskTrait> Getter<T> for Tasks{
    fn get_inner(&self) -> &T {
        unreachable!()
    }
}

impl Tasks {
    pub fn get<T: TaskTrait>(&self, id: TaskId) -> Result<&T> {
        let task = self.tasks.get(&id).ok_or(anyhow!("missing task"))?;
        let v= Getter::<T>::get_inner(task);
        todo!()
    }

    pub fn get_mut(&mut self, id: TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    fn available_id(&mut self) -> TaskId {
        match self.available_ids.pop_first() {
            Some(id) => id,
            None => {
                let mut id = self.max_id.next();
                swap(&mut id, &mut self.max_id);
                id
            }
        }
    }
}

#[derive(Debug)]
pub enum Task {
    // requirements
    EnergyRequire(EnergyRequireTask),
    // tasks
    BasicHarvester(BasicHarvesterTask),
    Upgrade(UpgradeTask),
    Build(BuildTask),
}

pub trait TaskTrait {
    fn id(&self) -> TaskId;
}

impl TaskTrait for Task {
    fn id(&self) -> TaskId {
        todo!()
    }
}

// require energy
#[derive(Debug)]
pub struct EnergyRequireTask {
    id: TaskId,
    pub target: ObjectId<StructureSpawn>,
    pub energy: i32,
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
        self.tasks.insert(id, Task::EnergyRequire(task));
        match &self.tasks[&id] {
            Task::EnergyRequire(t) => t,
            _ => unreachable!(),
        }
    }
}

impl TaskTrait for EnergyRequireTask {
    fn id(&self) -> TaskId {
        self.id
    }
}

#[derive(Debug)]
pub struct BasicHarvesterTask {
    source: ObjectId<Source>,
    spawn: ObjectId<StructureSpawn>,
}

#[derive(Debug)]
pub struct UpgradeTask {
    spawn: ObjectId<StructureSpawn>,
    controller: ObjectId<StructureController>,
}

#[derive(Debug)]
pub struct BuildTask {
    spawn: ObjectId<StructureSpawn>,
    target: Option<ObjectId<ConstructionSite>>,
}
