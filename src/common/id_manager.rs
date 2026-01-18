use screeps::game;
use serde::{Deserialize, Serialize};

const FREE_COOLDOWN_TIME: u32 = 300; // ticks

// TODO how to make alloc_id and free_id not accessible out of implementation struct?
pub trait IdManager {
    fn get_pointer(&mut self) -> &mut Option<(usize, usize)>;
    fn get_tombstone(&mut self, index: usize) -> &mut Tombstone;

    fn alloc_id(&mut self) -> NewIdResult {
        let curr_time = game::time();
        let &mut Some((head, tail)) = self.get_pointer() else {
            return NewIdResult::NewId;
        };

        let tombstone = self.get_tombstone(head);

        if tombstone.free_time + FREE_COOLDOWN_TIME <= curr_time {
            return NewIdResult::NewId;
        }

        let free_index = head;
        if head == tail {
            *self.get_pointer() = None;
        } else {
            let next_free = tombstone.next_free.expect("Invalid next_free pointer");
            self.get_pointer().expect("Already checked for Some").0 = next_free;
        }

        NewIdResult::ReusedId(free_index)
    }

    fn free_id(&mut self, id: usize) -> Tombstone {
        let tombstone = Tombstone {
            free_time: game::time(),
            next_free: None,
        };

        let &mut Some((_, tail)) = self.get_pointer() else {
            *self.get_pointer() = Some((id, id));
            return tombstone;
        };

        let tail_tombstone = self.get_tombstone(tail);
        tail_tombstone.next_free = Some(id);
        self.get_pointer().expect("Already checked for Some").1 = id;

        tombstone
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Tombstone {
    free_time: u32,
    next_free: Option<usize>,
}

pub enum NewIdResult {
    NewId,
    ReusedId(usize),
}
