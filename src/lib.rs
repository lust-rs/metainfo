use fxhash::FxHashMap;
use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// `MetaInfo` is used to passthrough information between components and even client-server.
/// It supports two types of info: typed map and string k-v.
/// It is designed to be tree-like, which means you can share a `MetaInfo` with multiple children.
/// Note: only the current scope is mutable.
///
/// Examples:
/// ```rust
/// use metainfo::MetaInfo;
/// use std::sync::Arc;
///
/// fn test() {
///     let mut m1 = MetaInfo::new();
///     m1.insert::<i8>(2);
///     assert_eq!(*m1.get::<i8>().unwrap(), 2);
///
///     let mut m2 = MetaInfo::from(Arc::new(m1));
///     assert_eq!(*m2.get::<i8>().unwrap(), 2);
///
///     m2.insert::<i8>(4);
///     assert_eq!(*m2.get::<i8>().unwrap(), 4);
///
///     m2.remove::<i8>();
///     assert_eq!(*m2.get::<i8>().unwrap(), 2);
/// }
/// ```
pub struct MetaInfo {
    /// Parent is read-only, if we can't find the specified key in the current,
    /// we search it in the parent scope.
    parent: Option<Arc<MetaInfo>>,
    tmap: FxHashMap<TypeId, Box<dyn Any + Send + Sync>>,
    smap: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl MetaInfo {
    /// Creates an empty `MetaInfo`.
    #[inline]
    pub fn new() -> MetaInfo {
        MetaInfo {
            parent: None,
            tmap: FxHashMap::default(),
            smap: HashMap::default(),
        }
    }

    /// Creates an `MetaInfo` with the parent given.
    /// When the info is not found in the current scope, `MetaInfo` will try to get from parent.
    #[inline]
    pub fn from(parent: Arc<MetaInfo>) -> MetaInfo {
        MetaInfo {
            parent: Some(parent),
            tmap: FxHashMap::default(),
            smap: HashMap::default(),
        }
    }

    /// Insert a type into this `MetaInfo`.
    pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) {
        self.tmap.insert(TypeId::of::<T>(), Box::new(val));
    }

    /// Insert a string k-v into this `MetaInfo`.
    pub fn insert_string(&mut self, key: Cow<'static, str>, val: Cow<'static, str>) {
        self.smap.insert(key, val);
    }

    /// Check if `MetaInfo` contains entry
    pub fn contains<T: 'static>(&self) -> bool {
        if self.tmap.contains_key(&TypeId::of::<T>()) {
            true
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().contains::<T>()
        } else {
            false
        }
    }

    /// Check if `MetaInfo` contains the given string k-v
    pub fn contains_string(&self, key: &str) -> bool {
        if self.smap.contains_key(key) {
            true
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().contains_string(key)
        } else {
            false
        }
    }

    /// Get a reference to a type previously inserted on this `MetaInfo`.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        let t = self
            .tmap
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref());
        if t.is_some() {
            return t;
        }
        if self.parent.is_some() {
            return self.parent.as_ref().unwrap().get::<T>();
        }
        None
    }

    /// Remove a type from this `MetaInfo` and return it.
    /// Can only remove the type in the current scope.
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.tmap
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    /// Get a reference to a string k-v previously inserted on this `MetaInfo`.
    pub fn get_string(&self, key: &str) -> Option<Cow<'static, str>> {
        let t = self.smap.get(key);
        if let Some(t) = t {
            return Some(t.clone());
        }
        if self.parent.is_some() {
            return self.parent.as_ref().unwrap().get_string(key);
        }
        None
    }

    /// Remove a string k-v from this `MetaInfo` and return it.
    /// Can only remove the type in the current scope.
    pub fn remove_string(&mut self, key: &str) -> Option<Cow<'static, str>> {
        self.smap.remove(key)
    }

    /// Clear the `MetaInfo` of all inserted MetaInfo.
    #[inline]
    pub fn clear(&mut self) {
        self.tmap.clear();
        self.smap.clear();
    }

    /// Extends self with the items from another `MetaInfo`.
    /// Only extend the items in the current scope.
    pub fn extend(&mut self, other: MetaInfo) {
        self.tmap.extend(other.tmap);
        self.smap.extend(other.smap);
    }
}

impl Default for MetaInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for MetaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MetaInfo").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let mut map = MetaInfo::new();

        map.insert::<i8>(123);
        assert!(map.get::<i8>().is_some());

        map.remove::<i8>();
        assert!(map.get::<i8>().is_none());

        map.insert::<i8>(123);

        let mut m2 = MetaInfo::from(Arc::new(map));

        m2.remove::<i8>();
        assert!(m2.get::<i8>().is_some());
    }

    #[test]
    fn test_clear() {
        let mut map = MetaInfo::new();

        map.insert::<i8>(8);
        map.insert::<i16>(16);
        map.insert::<i32>(32);

        assert!(map.contains::<i8>());
        assert!(map.contains::<i16>());
        assert!(map.contains::<i32>());

        map.clear();

        assert!(!map.contains::<i8>());
        assert!(!map.contains::<i16>());
        assert!(!map.contains::<i32>());

        map.insert::<i8>(10);
        assert_eq!(*map.get::<i8>().unwrap(), 10);
    }

    #[test]
    fn test_integers() {
        let mut map = MetaInfo::new();

        map.insert::<i8>(8);
        map.insert::<i16>(16);
        map.insert::<i32>(32);
        map.insert::<i64>(64);
        map.insert::<i128>(128);
        map.insert::<u8>(8);
        map.insert::<u16>(16);
        map.insert::<u32>(32);
        map.insert::<u64>(64);
        map.insert::<u128>(128);
        assert!(map.get::<i8>().is_some());
        assert!(map.get::<i16>().is_some());
        assert!(map.get::<i32>().is_some());
        assert!(map.get::<i64>().is_some());
        assert!(map.get::<i128>().is_some());
        assert!(map.get::<u8>().is_some());
        assert!(map.get::<u16>().is_some());
        assert!(map.get::<u32>().is_some());
        assert!(map.get::<u64>().is_some());
        assert!(map.get::<u128>().is_some());

        let m2 = MetaInfo::from(Arc::new(map));
        assert!(m2.get::<i8>().is_some());
        assert!(m2.get::<i16>().is_some());
        assert!(m2.get::<i32>().is_some());
        assert!(m2.get::<i64>().is_some());
        assert!(m2.get::<i128>().is_some());
        assert!(m2.get::<u8>().is_some());
        assert!(m2.get::<u16>().is_some());
        assert!(m2.get::<u32>().is_some());
        assert!(m2.get::<u64>().is_some());
        assert!(m2.get::<u128>().is_some());
    }

    #[test]
    fn test_composition() {
        struct Magi<T>(pub T);

        struct Madoka {
            pub god: bool,
        }

        struct Homura {
            pub attempts: usize,
        }

        struct Mami {
            pub guns: usize,
        }

        let mut map = MetaInfo::new();

        map.insert(Magi(Madoka { god: false }));
        map.insert(Magi(Homura { attempts: 0 }));
        map.insert(Magi(Mami { guns: 999 }));

        assert!(!map.get::<Magi<Madoka>>().unwrap().0.god);
        assert_eq!(0, map.get::<Magi<Homura>>().unwrap().0.attempts);
        assert_eq!(999, map.get::<Magi<Mami>>().unwrap().0.guns);
    }

    #[test]
    fn test_metainfo() {
        #[derive(Debug, PartialEq)]
        struct MyType(i32);

        let mut metainfo = MetaInfo::new();

        metainfo.insert(5i32);
        metainfo.insert(MyType(10));

        assert_eq!(metainfo.get(), Some(&5i32));

        assert_eq!(metainfo.remove::<i32>(), Some(5i32));
        assert!(metainfo.get::<i32>().is_none());

        assert_eq!(metainfo.get::<bool>(), None);
        assert_eq!(metainfo.get(), Some(&MyType(10)));
    }

    #[test]
    fn test_extend() {
        #[derive(Debug, PartialEq)]
        struct MyType(i32);

        let mut metainfo = MetaInfo::new();

        metainfo.insert(5i32);
        metainfo.insert(MyType(10));

        let mut other = MetaInfo::new();

        other.insert(15i32);
        other.insert(20u8);

        metainfo.extend(other);

        assert_eq!(metainfo.get(), Some(&15i32));

        assert_eq!(metainfo.remove::<i32>(), Some(15i32));
        assert!(metainfo.get::<i32>().is_none());

        assert_eq!(metainfo.get::<bool>(), None);
        assert_eq!(metainfo.get(), Some(&MyType(10)));

        assert_eq!(metainfo.get(), Some(&20u8));
    }
}
