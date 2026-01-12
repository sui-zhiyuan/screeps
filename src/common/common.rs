pub trait EnumDowncast<Task>: Sized {
    fn enum_downcast(from: Task) -> Option<Self>;
    fn enum_downcast_ref(from: &Task) -> Option<&Self>;
    fn enum_downcast_mut(from: &mut Task) -> Option<&mut Self>;
}

