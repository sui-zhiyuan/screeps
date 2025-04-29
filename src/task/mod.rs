mod basic_harvest_task;
mod build_task;
mod energy_require_task;
mod task_id;
mod tasks;
mod upgrade_task;

use crate::task::basic_harvest_task::BasicHarvestTask;
use crate::task::build_task::BuildTask;
use crate::task::upgrade_task::UpgradeTask;
pub use energy_require_task::EnergyRequireTask;
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

pub trait DownCast {
    fn cast(task: &Task) -> Option<&Self>;
    fn cast_mut(task: &mut Task) -> Option<&mut Self>;
}

#[macro_export]
macro_rules! impl_caster {
    ($t:ty , $arm: ident) => {
        impl crate::task::DownCast for $t {
            fn cast(task: &crate::task::Task) -> Option<&Self> {
                match task {
                    crate::task::Task::$arm(task) => Some(task),
                    _ => None,
                }
            }

             fn cast_mut(task: &mut crate::task::Task) -> Option<&mut Self> {
                match task {
                    crate::task::Task::$arm(task) => Some(task),
                    _ => None,
                }
            }
        }
    };
}
