pub trait EnumDowncast<Task>: Sized {
    fn enum_downcast(from: Task) -> Option<Self>;
    fn enum_downcast_ref(from: &Task) -> Option<&Self>;
    fn enum_downcast_mut(from: &mut Task) -> Option<&mut Self>;
}

pub trait EnumDispatcher: Sized {
    fn downcast<T: EnumDowncast<Self>>(self) -> Option<T> {
        T::enum_downcast(self)
    }
    fn downcast_ref<T: EnumDowncast<Self>>(&self) -> Option<&T> {
        T::enum_downcast_ref(self)
    }
    fn downcast_mut<T: EnumDowncast<Self>>(&mut self) -> Option<&mut T> {
        T::enum_downcast_mut(self)
    }
}

macro_rules! enum_downcast {
    ($component:ident, $variant:ident, $ty: ident) => {
        impl EnumDowncast<$component> for $ty {
            fn enum_downcast(from: $component) -> Option<Self> {
                match from {
                    $component::$variant(item) => Some(item),
                    _ => None,
                }
            }
            fn enum_downcast_ref(from: &$component) -> Option<&Self> {
                match from {
                    $component::$variant(item) => Some(item),
                    _ => None,
                }
            }
            fn enum_downcast_mut(from: &mut $component) -> Option<&mut Self> {
                match from {
                    $component::$variant(item) => Some(item),
                    _ => None,
                }
            }
        }
    }
}

pub(crate) use enum_downcast; 