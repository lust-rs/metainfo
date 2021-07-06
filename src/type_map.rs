use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

pub(crate) type AnyObject = Box<dyn Any + Send + Sync>;

#[derive(Debug, Default)]
pub struct TypeMap {
    inner: HashMap<TypeId, AnyObject, BuildHasherDefault<IdentHash>>,
}

impl TypeMap {
    #[inline]
    pub fn insert<T: Send + Sync + 'static>(&mut self, t: T) {
        self.inner.insert(TypeId::of::<T>(), Box::new(t));
    }

    #[inline]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    #[inline]
    pub fn contains<T: 'static>(&self) -> bool {
        self.inner.contains_key(&TypeId::of::<T>())
    }

    #[inline]
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.inner
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline]
    pub fn extend(&mut self, other: TypeMap) {
        self.inner.extend(other.inner)
    }
}

pub struct IdentHash(u64);

impl Hasher for IdentHash {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u8(*byte);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.0 = (self.0 << 8) | (i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

impl Default for IdentHash {
    fn default() -> IdentHash {
        IdentHash(0)
    }
}
