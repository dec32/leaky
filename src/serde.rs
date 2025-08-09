use serde::{Deserialize, Serialize};

use crate::Leak;

impl<'de, T> Deserialize<'de> for Leak<T>
where
    T: ?Sized,
    Box<T>: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let boxed = Box::<T>::deserialize(deserializer)?;
        Ok(Self(Box::leak(boxed)))
    }
}


impl<T> Serialize for Leak<T>
where
    T: Serialize + ?Sized
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        T::serialize(self.0, serializer)
    }
}