use log::error;
use screeps::game;

mod creep_worker;
mod spawn;

pub(crate) fn run() {
    for s in game::spawns().values() {
        match spawn::run(&s) {
            Ok(_) => (),
            Err(e) => {
                error!("spawn running error {}", e);
            }
        }
    }

    for c in game::creeps().values() {
        if let Err(e) = creep_worker::run(&c) {
            error!("creep run error {}", e);
        }
    }
}
