use std::borrow::Cow;

pub trait Backward {
    // We don't think backward persistent makes sense.
    fn get_backward_transient<K: Into<Cow<'static, str>>>(
        &self,
        key: K,
    ) -> Option<Cow<'static, str>>;
    fn get_backward_downstream<K: Into<Cow<'static, str>>>(
        &self,
        key: K,
    ) -> Option<Cow<'static, str>>;

    fn set_backward_transient<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        &mut self,
        key: K,
        value: V,
    );
    fn set_backward_downstream<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        &mut self,
        key: K,
        value: V,
    );

    fn del_backward_transient<K: Into<Cow<'static, str>>>(&mut self, key: K);
    fn del_backward_downstream<K: Into<Cow<'static, str>>>(&mut self, key: K);
}
