use crate::actor::{Actors, CreepSpawnTask};
use crate::task::Tasks;
use anyhow::anyhow;
use tracing::info;

pub fn plan(actors: &mut Actors, tasks: &mut Tasks) -> anyhow::Result<()> {
    info!("room planning");

    let has_spawn = tasks.iter::<CreepSpawnTask>().next().is_some();

    if !has_spawn {
        let (&default_room_id, _) = actors
            .room_actors
            .iter()
            .next()
            .ok_or(anyhow!("no room found"))?;
        _ = tasks.add_task(|id| {
            CreepSpawnTask::new_task(id, default_room_id, crate::actor::CreepClass::Worker)
        })?;
    }

    // TODO
    // let room = self.prototype.room().ok_or(anyhow!("room not found"))?;
    // let creeps = room.find(find::CREEPS, None);
    // let structure = match creeps.len() {
    //     0 => CreepStructure::new_harvest(spawn)?,
    //     1 => CreepStructure::new_builder(spawn)?,
    //     2 => CreepStructure::new_upgrader(spawn)?,
    //     _ => return Ok(()),
    // };

    Ok(())
}
