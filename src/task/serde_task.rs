use crate::task::{Task, Tasks};
use anyhow::bail;
use serde::ser::Error;
use serde::{Deserialize, Serialize, Serializer};
use tracing::warn;

#[derive(Deserialize)]
#[serde(try_from = "Vec<Task>")]
pub struct TaskSerializePhantom();

impl Serialize for TaskSerializePhantom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let guard = Tasks::get_guard()
            .map_err(|e| Error::custom(format!("load guard failed with: {}", e)))?;
        let tasks = guard
            .as_ref()
            .ok_or_else(|| Error::custom("task not initialize"))?;
        tasks.0.serialize(serializer)
    }
}

impl TryFrom<Vec<Task>> for TaskSerializePhantom {
    type Error = anyhow::Error;

    fn try_from(value: Vec<Task>) -> Result<Self, Self::Error> {
        let mut guard = Tasks::get_guard()?;
        if guard.as_mut().is_some() {
            bail!("task already initialized")
        }
        *guard = Some(Tasks(value));
        Ok(TaskSerializePhantom())
    }
}

impl Default for TaskSerializePhantom {
    fn default() -> Self {
        let mut guard = Tasks::get_guard().unwrap();
        if guard.as_mut().is_some() {
            warn!("already initialized");
            return TaskSerializePhantom();
        }

        *guard = Some(Tasks(Vec::new()));
        TaskSerializePhantom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::CreepClass;
    use crate::memory::Memory;
    use crate::task::CreepSpawnTask;
    use screeps::RoomName;

    #[test]
    fn test_initialize() {
        let value = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{}}"#;
        let memory: Memory = serde_json::from_str(value).unwrap();
        Tasks::with(|v| {
            assert_eq!(v.0.len(), 0);

            v.0.push(Task::NoTask);
            Ok(())
        })
        .unwrap();

        let json_memory = serde_json::to_string(&memory).unwrap();
        let result = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{},"tasks":[{"t":"NoTask"}]}"#;
        assert_eq!(result, json_memory);
    }

    #[test]
    fn test_serialize() {
        let value = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{},"tasks":[{"t":"NoTask"}]}"#;
        let memory: Memory = serde_json::from_str(value).unwrap();
        Tasks::with(|v| {
            assert_eq!(v.0.len(), 1);
            assert_eq!(Task::NoTask, v.0[0]);

            v.0.pop();
            v.0.push(CreepSpawnTask::new_task(
                RoomName::new("E1N3").unwrap(),
                CreepClass::Worker,
            ));
            Ok(())
        })
        .unwrap();

        let json_memory = serde_json::to_string(&memory).unwrap();
        let result = r#"{"rooms":{},"spawns":{},"creeps":{},"flags":{},"tasks":[{"t":"CreepSpawn","room":"E1N3","creep_class":"Worker"}]}"#;
        assert_eq!(result, json_memory);
    }
}
