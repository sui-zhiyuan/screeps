use log::error;
use screeps::game;

mod creep_actor;
mod spawn_actor;

pub(crate) fn run() {
    for s in game::spawns().values() {
        match spawn_actor::run(&s) {
            Ok(_) => (),
            Err(e) => {
                error!("spawn running error {}", e);
            }
        }
    }

    for c in game::creeps().values() {
        if let Err(e) = creep_actor::run(&c) {
            error!("creep run error {}", e);
        }
    }
}
