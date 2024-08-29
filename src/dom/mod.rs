use web_sys::wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::js_sys;


#[derive(Clone)]
pub(crate) enum DOM {
    Text(web_sys::Text),
    Element(web_sys::Element)
}

impl From<web_sys::Element> for DOM {
    fn from(element: web_sys::Element) -> Self {
        Self::Element(element)
    }
}

impl AsRef<web_sys::Node> for DOM {
    fn as_ref(&self) -> &web_sys::Node {
        match self {
            Self::Text(t)    => t.unchecked_ref(),
            Self::Element(e) => e.unchecked_ref()
        }
    }
}

impl DOM {
    pub(crate) fn append_child(&mut self, child: &DOM) {
        let Self::Element(element) = self else {return};
        element.append_child(child.as_ref()).unwrap_throw();
    }
    pub(crate) fn remove_child(&mut self, child: &DOM) {
        let Self::Element(element) = self else {return};
        element.remove_child(child.as_ref()).unwrap_throw();
    }

    pub(crate) fn set_attribute(&mut self, name: &str, value: &str) {
        let Self::Element(element) = self else {return};
        element.set_attribute(name, value).unwrap_throw();
    }
    pub(crate) fn remove_attribute(&mut self, name: &str) {
        let Self::Element(element) = self else {return};
        element.remove_attribute(name).unwrap_throw();
    }

    pub(crate) fn add_event_listener(&mut self, event: &str, listener: &js_sys::Function) {
        let Self::Element(element) = self else {return};
        element.add_event_listener_with_callback(event, listener).unwrap_throw();
    }
    pub(crate) fn remove_event_listener(&mut self, event: &str, listener: &js_sys::Function) {
        let Self::Element(element) = self else {return};
        element.remove_event_listener_with_callback(event, listener).unwrap_throw();
    }
}
