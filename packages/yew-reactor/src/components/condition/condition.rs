use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};

pub trait AsBool {
    fn as_bool(&self) -> bool;

    fn as_inverted_bool(&self) -> bool {
        !self.as_bool()
    }
}

impl AsBool for () {
    fn as_bool(&self) -> bool {
        false
    }
}

impl AsBool for bool {
    fn as_bool(&self) -> bool {
        *self
    }
}

impl AsBool for String {
    fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    fn as_inverted_bool(&self) -> bool {
        self.is_empty()
    }
}

impl AsBool for &str {
    fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    fn as_inverted_bool(&self) -> bool {
        self.is_empty()
    }
}

impl<T> AsBool for HashSet<T> {
    fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    fn as_inverted_bool(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> AsBool for HashMap<K, V> {
    fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    fn as_inverted_bool(&self) -> bool {
        self.is_empty()
    }
}

impl<T> AsBool for BTreeSet<T> {
    fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    fn as_inverted_bool(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> AsBool for BTreeMap<K, V> {
    fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    fn as_inverted_bool(&self) -> bool {
        self.is_empty()
    }
}

impl<T> AsBool for [T] {
    fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    fn as_inverted_bool(&self) -> bool {
        self.is_empty()
    }
}

macro_rules! impl_as_bool_for_numeric {
    ($t:ty) => {
        impl AsBool for $t {
            fn as_bool(&self) -> bool {
                *self != (0 as $t)
            }

            fn as_inverted_bool(&self) -> bool {
                *self == (0 as $t)
            }
        }
    };
}

impl_as_bool_for_numeric!(i8);
impl_as_bool_for_numeric!(i16);
impl_as_bool_for_numeric!(i32);
impl_as_bool_for_numeric!(i64);
impl_as_bool_for_numeric!(i128);
impl_as_bool_for_numeric!(isize);
impl_as_bool_for_numeric!(u8);
impl_as_bool_for_numeric!(u16);
impl_as_bool_for_numeric!(u32);
impl_as_bool_for_numeric!(u64);
impl_as_bool_for_numeric!(u128);
impl_as_bool_for_numeric!(usize);
impl_as_bool_for_numeric!(f32);
impl_as_bool_for_numeric!(f64);

impl<T: AsBool> AsBool for Option<T> {
    fn as_bool(&self) -> bool {
        match self {
            Some(v) => v.as_bool(),
            None => false,
        }
    }
}

impl<T: AsBool, E> AsBool for Result<T, E> {
    fn as_bool(&self) -> bool {
        match self {
            Ok(v) => v.as_bool(),
            Err(_) => false,
        }
    }
}

impl<T: AsBool> AsBool for &T {
    fn as_bool(&self) -> bool {
        AsBool::as_bool(*self)
    }
}
