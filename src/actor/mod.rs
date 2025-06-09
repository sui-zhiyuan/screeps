use anyhow::Result;
use enum_dispatch::enum_dispatch;
use screeps::{Creep, Room, game};
use tracing::error;

mod creep_actor;
mod creep_builder;
mod creep_harvester;
mod creep_upgrader;
mod room_actor;
mod spawn_actor;

use crate::context::Context;
pub use creep_actor::CreepMemory;
pub use room_actor::{RoomMemory, RoomMemorySerialize};
pub use spawn_actor::{CreepClass, CreepSpawnTask};

pub fn run(ctx: &Context) {
    let rooms = game::rooms().values();

    let actors = rooms.map(ActorDispatch::Room).collect::<Vec<_>>();

    for a in actors.iter() {
        if let Err(e) = a.plan(ctx) {
            error!("plan error on {} : {}", a.name(), e);
        };
    }

    for a in actors.iter() {
        if let Err(e) = a.assign(ctx) {
            error!("assign error on {} : {}", a.name(), e);
        };
    }

    for a in actors.iter() {
        if let Err(e) = a.run(ctx) {
            error!("run error on {} : {}", a.name(), e);
        };
    }

    for s in game::spawns().values() {
        if let Err(e) = spawn_actor::run(ctx, &s) {
            error!("spawn running error {}", e);
        }
    }

    for c in game::creeps().values() {
        if let Err(e) = creep_actor::run(ctx, &c) {
            error!("creep run error {}", e);
        }
    }
}

trait CreepMemoryTrait {
    fn run(&mut self, creep: &Creep) -> anyhow::Result<()>;
}

#[enum_dispatch]
trait Actor: Sized {
    fn name(&self) -> String;
    fn plan(&self, ctx: &Context) -> Result<()>;
    fn assign(&self, ctx: &Context) -> Result<()>;
    fn run(&self, ctx: &Context) -> Result<()>;
}

#[enum_dispatch(Actor)]
enum ActorDispatch {
    Room(Room),
}
