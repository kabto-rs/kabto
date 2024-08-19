mod eventhandler;

use self::eventhandler::EventHandler;
use std::{borrow::Cow, collections::HashMap};
use web_sys::wasm_bindgen::{JsValue, JsCast};


pub enum Element {
    Text(Cow<'static, str>),
    Tag {
        name:          &'static str,
        attributes:    HashMap<&'static str, Cow<'static, str>>,
        eventhandlers: HashMap<&'static str, EventHandler>,
        children:      Vec<Element>
    }
}

impl Element {
    pub fn CSR(self, container: &web_sys::Node) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        match self {
            Element::Text(text) => {
                let node = document.create_text_node(&text);
                container.append_child(&node)?;
            }
            Element::Tag { name, attributes, eventhandlers, children } => {
                let node = document.create_element(name)?; {
                    for (name, value) in attributes {
                        node.set_attribute(name, &value)?;
                    }
                    for (event, handler) in eventhandlers {
                        let handler = handler.into_wasm_closure();
                        node.add_event_listener_with_callback(event, handler.as_ref().unchecked_ref())?;
                        handler.forget();
                    }
                    for child in children {
                        child.CSR(container)?;
                    }
                }
                container.append_child(&node)?;
            }
        }

        Ok(())
    }
}
