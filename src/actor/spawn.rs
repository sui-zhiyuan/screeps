use anyhow::{Result, anyhow};
use log::info;
use screeps::{Part, StructureSpawn, find, game};

pub(crate) fn run(spawn: StructureSpawn) -> Result<()> {
    let room = spawn.room().ok_or(anyhow!("room not found"))?;
    let creeps = room.find(find::CREEPS, None);
    if !creeps.is_empty() {
        return Ok(());
    }

    info!("creating working");

    let body = [Part::Carry, Part::Work, Part::Move, Part::Move];
    let cost = body.iter().map(|p| p.cost()).sum();

    if room.energy_available() < cost {
        info!("not enough energy");
        return Ok(());
    }

    let additional = 0;
    // create a unique name, spawn.
    let name_base = game::time();
    let name = format!("{}-{}", name_base, additional);

    spawn.spawn_creep(&body, &name)?;
    Ok(())
}
