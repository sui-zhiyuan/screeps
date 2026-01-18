mod id_manager;
mod enum_downcast;

pub(crate) use id_manager::{IdManager, NewIdResult, Tombstone};
pub(crate) use enum_downcast::{EnumDispatcher, EnumDowncast, enum_downcast};
