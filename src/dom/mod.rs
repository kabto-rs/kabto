#[derive(Clone)]
pub(crate) struct DOM(
    web_sys::Element
);

impl std::ops::Deref for DOM {
    type Target = web_sys::Element;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<web_sys::Node> for DOM {
    fn as_ref(&self) -> &web_sys::Node {
        use web_sys::wasm_bindgen::JsCast;
        self.0.unchecked_ref()
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
