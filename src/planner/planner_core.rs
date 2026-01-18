use crate::actor::{ActorTrait, Actors, CreepSpawnTask, RoomActor};
use crate::task::Tasks;
use anyhow::anyhow;
use tracing::info;

pub fn plan(actors: &mut Actors, tasks: &mut Tasks) -> anyhow::Result<()> {
    info!("room planning");

    let has_spawn = tasks.iter::<CreepSpawnTask>().next().is_some();

    if !has_spawn {
        let default_room = actors
            .iter::<RoomActor>()
            .next()
            .ok_or(anyhow!("no room found"))?;
        let spawn_task =
            CreepSpawnTask::new_task(default_room.name(), crate::actor::CreepClass::Worker);
        tasks.add_task(spawn_task);
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
