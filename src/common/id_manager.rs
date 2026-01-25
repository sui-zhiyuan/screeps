use anyhow::{Result, anyhow};
use screeps::game;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;

const FREE_COOLDOWN_TIME: u32 = 300; // ticks
type KeyType = usize;

#[derive(Serialize, Deserialize, Clone)]
pub struct IdManager<T> {
    free_keys: BTreeSet<KeyType>,
    pending_keys: VecDeque<PendingKey>,
    next_id: KeyType,
    #[serde(skip)]
    phantom: PhantomData<T>,
}

impl<T> Default for IdManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct PendingKey {
    key: KeyType,
    free_time: u32,
}

impl<T> IdManager<T> {
    fn new() -> Self {
        IdManager {
            free_keys: BTreeSet::new(),
            pending_keys: VecDeque::new(),
            next_id: 0,
            phantom: PhantomData,
        }
    }
}

impl<T> IdManager<T>
where
    T: From<usize> + Into<usize> + Copy,
{
    pub fn alloc_id(&mut self) -> T {
        let curr_time = game::time();

        while let Some(pending) = self.pending_keys.front() {
            if pending.free_time + FREE_COOLDOWN_TIME > curr_time {
                break;
            }
            let pending = self.pending_keys.pop_front().unwrap();
            self.free_keys.insert(pending.key);
            while self.free_keys.remove(&(self.next_id - 1)) {
                self.next_id -= 1;
            }
        }

        if let Some(key) = self.free_keys.pop_first() {
            key.into()
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id.into()
        }
    }

    pub fn free_id(&mut self, id: T) {
        let pending = PendingKey {
            key: id.into(),
            free_time: game::time(),
        };
        self.pending_keys.push_back(pending);
    }
}

pub fn hash_map<TKIn, TKOut, TVIn, TVOut>(
    m: &HashMap<TKIn, TVIn>,
    mut f_key: impl FnMut(TKIn) -> TKOut,
    mut f_val: impl FnMut(&TVIn) -> TVOut,
) -> HashMap<TKOut, TVOut>
where
    TKIn: Copy,
    TKOut: Eq + Hash,
{
    let mut result = HashMap::with_capacity(m.len());
    for (k, v) in m.iter() {
        let new_key = f_key(*k);
        let new_val = f_val(v);
        result.insert(new_key, new_val);
    }
    result
}

pub fn hash_map_key<TKIn, TKOut, TVIn, TVOut>(
    m: &HashMap<TKIn, TVIn>,
    mut f_key: impl FnMut(&TKIn) -> TKOut,
    mut f_val: impl FnMut(TKOut, &TVIn) -> Option<TVOut>,
) -> HashMap<TKOut, TVOut>
where
    TKOut: Copy + Eq + Hash,
{
    let mut result = HashMap::with_capacity(m.len());
    for (k, v) in m.iter() {
        let new_key = f_key(k);
        let Some(new_val) = f_val(new_key, v) else {
            continue;
        };
        result.insert(new_key, new_val);
    }
    result
}

// TODO how to make alloc_id and free_id not accessible out of implementation struct?
pub trait IdManager1 {
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

pub fn hash_to_vec<TKey, TValue>(
    mut hash: HashMap<TKey, TValue>,
    mut f_key: impl FnMut(usize) -> TKey,
) -> Result<Vec<TValue>>
where
    TKey: Eq + Hash,
{
    let mut result = Vec::with_capacity(hash.len());
    for id in 0..hash.len() {
        let key = f_key(id);
        let value = hash.remove(&key).ok_or(anyhow!("Missing key"))?;
        result.push(value);
    }
    Ok(result)
}

pub fn vec_to_hash<TKey, TValue>(
    vec: Vec<TValue>,
    mut f_key: impl FnMut(usize) -> TKey,
) -> Result<HashMap<TKey, TValue>>
where
    TKey: Eq + Hash,
{
    let mut result = HashMap::with_capacity(vec.len());
    for (id, value) in vec.into_iter().enumerate() {
        let key = f_key(id);
        result.insert(key, value);
    }
    Ok(result)
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
