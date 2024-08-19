pub(crate) mod eventhandler;

use self::eventhandler::EventHandler;
use std::{borrow::Cow, collections::HashMap};
use web_sys::wasm_bindgen::{JsValue, JsCast};


pub enum Node {
    Text(Cow<'static, str>),
    Element(Element),
}

pub struct Element {
    pub(crate) tag:           &'static str,
    pub(crate) attributes:    HashMap<&'static str, AttributeValue>,
    pub(crate) eventhandlers: HashMap<&'static str, EventHandler>,
    pub(crate) children:      Vec<Node>
}

pub struct AttributeValue(
    Cow<'static, str>
); const _: () = {
    impl From<String> for AttributeValue {
        fn from(value: String) -> Self {
            Self(value.to_string().into())
        }
    }
    impl From<&'static str> for AttributeValue {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }

    macro_rules! integer_value {
        ($($t:ty)*) => {$(
            impl From<$t> for AttributeValue {
                fn from(value: $t) -> Self {
                    Self(value.to_string().into())
                }
            }
        )*};
    } integer_value! { u8 usize i32 }
};

impl Node {
    pub fn csr(self, container: &web_sys::Node) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        match self {
            Node::Text(text) => {
                let node = document.create_text_node(&text);
                container.append_child(&node)?;
            }
            Node::Element(Element { tag, attributes, eventhandlers, children }) => {
                let node = document.create_element(tag)?; {
                    for (name, value) in attributes {
                        node.set_attribute(name, &value.0)?;
                    }
                    for (event, handler) in eventhandlers {
                        let handler = handler.into_wasm_closure();
                        node.add_event_listener_with_callback(event, handler.as_ref().unchecked_ref())?;
                        handler.forget();
                    }
                    for child in children {
                        child.csr(container)?;
                    }
                }
                container.append_child(&node)?;
            }
        }

        Ok(())
    }
}
