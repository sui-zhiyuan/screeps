use gloo_utils::format::JsValueSerdeExt;
use screeps::{game, RoomName};
use serde::Deserialize;
use tracing::{error, info, warn};

pub fn test1() {
    let is_sim = is_simulator();
    info!(is_sim, "finish");

    let rn = RoomName::new("sim").unwrap();
    info!(%rn, "start");
}

#[derive(Deserialize)]
struct RoomStruct {
    name: String,
}

fn is_simulator() -> bool {
    let mut room = game::rooms().values();
    let Some(room) = room.next() else {
        warn!("no room found");
        return false;
    };
    let room: RoomStruct = match room.into_serde() {
        Ok(room) => room,
        Err(err) => {
            error!(%err , "parse room failed");
            return false;
        }
    };
    room.name == "sim"
}
