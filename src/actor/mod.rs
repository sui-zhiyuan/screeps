use log::error;
use screeps::game;

mod spawn;

pub(crate) fn run(){
    for s in game::spawns().values() {
        match spawn::run(s) {
            Ok(_) => (),
            Err(e) => {
                error!("spawn running error {}", e);
            }
        }
    }
}