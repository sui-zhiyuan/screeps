use crate::actor::creep_actor::CreepMemory;
use anyhow::anyhow;
use screeps::{
    Creep, HasId, HasPosition, ObjectId, ResourceType, SharedCreepProperties, StructureController,
    StructureSpawn,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreepUpgraderMemory {
    spawn: ObjectId<StructureSpawn>,
    controller: ObjectId<StructureController>,
    state: State,
}

#[derive(Serialize, Deserialize, Debug)]
enum State {
    Loading,
    Upgrading,
}

impl CreepUpgraderMemory {
    #[allow(dead_code)]
    pub fn new_memory(spawn: &StructureSpawn, controller: &StructureController) -> CreepMemory {
        // CreepMemory::Upgrader(CreepUpgraderMemory {
        //     spawn: spawn.id(),
        //     controller: controller.id(),
        //     state: State::Loading,
        // })
        todo!()
    }
}

#[allow(dead_code)]
fn run(memory: &mut CreepUpgraderMemory, creep: &Creep) -> anyhow::Result<()> {
    if creep.spawning() {
        return Ok(());
    }

    if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
        let spawn = &memory.spawn.resolve().ok_or(anyhow!("spawn not found"))?;
        if !creep.pos().is_near_to(spawn.pos()) {
            creep.move_to(spawn)?;
            return Ok(());
        }
        if creep.store().get_free_capacity(Some(ResourceType::Energy))
            > spawn.store().get_used_capacity(Some(ResourceType::Energy)) as i32
        {
            return Ok(());
        }

        creep.withdraw(spawn, ResourceType::Energy, None)?;
        Ok(())
    } else {
        let controller = &memory
            .controller
            .resolve()
            .ok_or(anyhow!("controller not found"))?;
        if !creep.pos().is_near_to(controller.pos()) {
            creep.move_to(controller)?;
            return Ok(());
        }

        creep.upgrade_controller(controller)?;
        Ok(())
    }
}
