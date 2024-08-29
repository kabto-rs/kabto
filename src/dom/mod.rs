pub(crate) struct DOM(
    web_sys::Element
);

impl std::ops::Deref for DOM {
    type Target = web_sys::Element;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<web_sys::Element> for DOM {
    fn from(value: web_sys::Element) -> Self {
        Self(value)
    }
}
impl From<web_sys::Text> for DOM {
    fn from(value: web_sys::Text) -> Self {
        use web_sys::wasm_bindgen::JsCast;
        Self(value.unchecked_into())
    }
}
