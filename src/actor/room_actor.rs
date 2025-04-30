use crate::actor::Actor;
use crate::actor::creep_actor::CreepClass;
use crate::memory::Memory;
use crate::task::Tasks;
use anyhow::anyhow;
use log::info;
use screeps::{Room, SharedCreepProperties, find};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

impl Actor for Room {
    fn plan(&self, memory: &mut Memory, tasks: &mut Tasks) -> anyhow::Result<()> {
        let controller = self.controller().ok_or(anyhow!("controller not found"))?;
        let level = controller.level();

        let creeps = self
            .find(find::CREEPS, None)
            .into_iter()
            .filter(|v| v.my())
            .collect::<Vec<_>>();

        let mut creep_by_class = creeps.iter().map(|v| memory.creeps[&v.name()].class).fold(
            HashMap::new(),
            |mut acc, class| {
                acc.entry(class).and_modify(|e| *e += 1).or_insert(1);
                acc
            },
        );

        let mut required_creep = None;
        for &(target_creep, count) in ROOM_CREEPS.iter() {
            let entry = creep_by_class.entry(target_creep).or_insert(0);
            if *entry >= count {
                *entry -= count;
                continue;
            }
            required_creep = Some(target_creep);
            break;
        }

        let Some(required_creep) = required_creep else {
            info!("no creeps needed");
            return Ok(());
        };

        tasks.new_creep_spawn(self, required_creep);
        Ok(())
    }

    fn run(&self, _memory: &mut Memory) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct RoomMemory();

static ROOM_CREEPS: LazyLock<Vec<(CreepClass, i32)>> =
    LazyLock::new(|| [(CreepClass::Worker, 5)].into_iter().collect());
