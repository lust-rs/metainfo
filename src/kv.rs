use paste::paste;
use std::borrow::Cow;
use std::sync::Arc;

const DEFAULT_CAPACITY: usize = 10; // maybe enough for most cases?

macro_rules! set_impl {
    ($name:ident) => {
        paste! {
            pub fn [<set_ $name>]<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
                &mut self,
                key: K,
                value: V,
            ) {
                let kv = KV::new(key, value);
                if self.$name.is_none() {
                    self.$name = Some(Vec::with_capacity(DEFAULT_CAPACITY));
                }
                self.$name.as_mut().unwrap().push(Arc::new(kv));
            }
        }
    };
}

macro_rules! del_impl {
    ($name:ident) => {
        paste! {
            pub fn [<del_ $name>]<K: AsRef<str>>(&mut self, key: K) {
                let key = key.as_ref();
                if let Some(v) = self.$name.as_mut() {
                    if let Some(index) = v.iter().position(|k| k.key == key) {
                        v.remove(index);
                    }
                }
            }
        }
    };
}

macro_rules! get_impl {
    ($name:ident) => {
        paste! {
            pub fn [<get_ $name>]<K: AsRef<str>>(&self, key: K) -> Option<Cow<'static, str>> {
                let key = key.as_ref();
                match self.$name.as_ref() {
                    Some(v) => {
                        let kv = v.iter().find(|&kv| kv.key == key);
                        kv.map(|kv| kv.value.clone())
                    }
                    None => None,
                }
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct KV {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
}

impl KV {
    pub fn new<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(key: K, value: V) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    persistent: Option<Vec<Arc<KV>>>,
    transient: Option<Vec<Arc<KV>>>,
    stale: Option<Vec<Arc<KV>>>,
}

impl Node {
    set_impl!(persistent);
    set_impl!(transient);
    set_impl!(stale);

    del_impl!(persistent);
    del_impl!(transient);
    del_impl!(stale);

    get_impl!(persistent);
    get_impl!(transient);
    get_impl!(stale);
}

impl Default for Node {
    fn default() -> Self {
        Self {
            persistent: None,
            transient: None,
            stale: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_stale() {
        let mut node = Node::default();
        node.set_stale("key", "value");
        println!("{:?}", node);
    }
}
