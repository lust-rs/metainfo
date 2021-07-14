use fxhash::FxHashMap;
use std::any::{Any, TypeId};

pub(crate) type AnyObject = Box<dyn Any + Send + Sync>;

#[derive(Debug, Default)]
pub struct TypeMap {
    inner: FxHashMap<TypeId, AnyObject>,
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
