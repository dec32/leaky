#![doc = include_str!("../README.md")]
use std::{
    cmp::Ordering,
    ffi::{CStr, CString, OsStr, OsString},
    fmt::{self},
    hash::{Hash, Hasher},
    ops::Deref,
    path::{Path, PathBuf},
};

#[cfg(feature = "serde")]
mod serde;
mod str;

// -----------------------------------------------------------------------------
// Core types and implementations
// -----------------------------------------------------------------------------

/// An wrapper around `&'static T` that provides additional functionality.
///
/// This type encapsulates a `'static` reference to heap-allocated data, offering
/// a clear semantic distinction for leaked values. For a detailed explanation of
/// this type's purpose and how it addresses common deserialization challenges,
/// please refer to the [crate-level documentation](crate).
pub struct Leak<T: ?Sized + 'static>(&'static T);

impl<T> Leak<T> {
    /// Creates a new `Leak<T>` by allocating `value` on the heap and leaking it.
    pub fn new(value: T) -> Self {
        Self(Box::leak(Box::new(value)))
    }
}

impl<T: ?Sized> Deref for Leak<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: ?Sized, R: ?Sized> AsRef<R> for Leak<T>
where
    for<'a> &'a T: AsRef<R>,
{
    fn as_ref(&self) -> &R {
        self.0.as_ref()
    }
}

impl<T: ?Sized> Copy for Leak<T> {}

impl<T: ?Sized> Clone for Leak<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: ?Sized + Eq> Eq for Leak<T> {}

impl<T: ?Sized + PartialEq> PartialEq for Leak<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        PartialEq::ne(&**self, &**other)
    }
}

impl<T: ?Sized + Ord> Ord for Leak<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<T: ?Sized + PartialOrd> PartialOrd for Leak<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        PartialOrd::lt(&**self, &**other)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        PartialOrd::le(&**self, &**other)
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        PartialOrd::ge(&**self, &**other)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        PartialOrd::gt(&**self, &**other)
    }
}

impl<T: ?Sized + Hash> Hash for Leak<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Leak<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for Leak<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

// -----------------------------------------------------------------------------
// Implementations of `Default` for `Leak<T>` where `T` is a slice type.
// -----------------------------------------------------------------------------

impl<T> Default for Leak<[T]> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Default for Leak<str> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Default for Leak<CStr> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Default for Leak<OsStr> {
    fn default() -> Self {
        Self(Default::default())
    }
}

// -----------------------------------------------------------------------------
// Leaker implementations
// -----------------------------------------------------------------------------

impl<T: ?Sized> From<Box<T>> for Leak<T> {
    fn from(value: Box<T>) -> Self {
        Self(Box::leak(value))
    }
}

impl<T> From<Vec<T>> for Leak<[T]> {
    fn from(value: Vec<T>) -> Self {
        Self(value.leak())
    }
}

impl From<String> for Leak<str> {
    fn from(value: String) -> Self {
        Self(value.leak())
    }
}

impl From<PathBuf> for Leak<Path> {
    fn from(value: PathBuf) -> Self {
        Self(Box::leak(value.into_boxed_path()))
    }
}

impl From<OsString> for Leak<OsStr> {
    fn from(value: OsString) -> Self {
        Self(Box::leak(value.into_boxed_os_str()))
    }
}

impl From<CString> for Leak<CStr> {
    fn from(value: CString) -> Self {
        Self(Box::leak(value.into_boxed_c_str()))
    }
}

// -----------------------------------------------------------------------------
// test
// -----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::Leak;
    use serde::{Deserialize, Serialize};
    use std::{
        ffi::{CStr, OsStr},
        mem,
        path::Path,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    struct LeakEverything(
        Leak<()>,
        Leak<u8>,
        Leak<[u8]>,
        Leak<str>,
        Leak<CStr>,
        Leak<OsStr>,
        Leak<Path>,
        Leak<[Leak<str>]>,
        Leak<Cake>,
        Leak<[Cake]>,
        Leak<[Leak<[Cake]>]>,
    );

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    struct Cake {
        id: String,
    }

    #[test]
    fn it_works() {
        as_ref();
        as_null();
    }

    fn as_ref() -> impl AsRef<Path> {
        let path: Leak<str> = String::from("/etc").into();
        path
    }

    fn as_null() {
        let none = Option::<Leak<str>>::None;
        assert_eq!(none, unsafe { mem::zeroed() })
    }
}
