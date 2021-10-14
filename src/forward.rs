use std::borrow::Cow;

pub trait Forward {
    fn get_persistent<K: Into<Cow<'static, str>>>(&self, key: K) -> Option<Cow<'static, str>>;
    fn get_transient<K: Into<Cow<'static, str>>>(&self, key: K) -> Option<Cow<'static, str>>;
    fn get_upstream<K: Into<Cow<'static, str>>>(&self, key: K) -> Option<Cow<'static, str>>;

    fn set_persistent<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        &mut self,
        key: K,
        value: V,
    );
    fn set_transient<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        &mut self,
        key: K,
        value: V,
    );
    fn set_upstream<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        &mut self,
        key: K,
        value: V,
    );

    fn del_persistent<K: Into<Cow<'static, str>>>(&mut self, key: K);
    fn del_transient<K: Into<Cow<'static, str>>>(&mut self, key: K);
    fn del_upstream<K: Into<Cow<'static, str>>>(&mut self, key: K);
}
