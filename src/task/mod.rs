mod basic_harvest_task;
mod build_task;
mod energy_require_task;
mod task_id;
mod tasks;
mod upgrade_task;

use crate::task::basic_harvest_task::BasicHarvestTask;
use crate::task::build_task::BuildTask;
use crate::task::energy_require_task::EnergyRequireTask;
use crate::task::upgrade_task::UpgradeTask;
use enum_dispatch::enum_dispatch;
pub use task_id::TaskId;
pub use tasks::Tasks;

#[derive(Debug)]
#[enum_dispatch(TaskTrait)]
pub enum Task {
    // requirements
    EnergyRequire(EnergyRequireTask),
    // tasks
    BasicHarvest(BasicHarvestTask),
    Upgrade(UpgradeTask),
    Build(BuildTask),
}

#[enum_dispatch]
pub trait TaskTrait {
    fn task_id(&self) -> TaskId;
}

pub trait Caster<T> {
    fn cast(&self) -> Option<&T>;
}

#[macro_export]
macro_rules! impl_caster {
    ($t:ty , $arm: ident) => {
        impl crate::task::Caster<$t> for crate::task::Task {
            fn cast(&self) -> Option<&$t> {
                match self {
                    crate::task::Task::$arm(task) => Some(task),
                    _ => None,
                }
            }
        }
    };
}
