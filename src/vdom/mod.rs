pub(crate) mod eventhandler;

use crate::util::Text;
use self::eventhandler::eventHandler;
use std::{borrow::Cow, collections::HashMap};
use web_sys::wasm_bindgen::{JsValue, JsCast};


pub enum Node {
    Text(Cow<'static, str>),
    Element(Element),
}

pub struct Element {
    pub(crate) tag:           &'static str,
    pub(crate) attributes:    Option<Box<HashMap<&'static str, Text>>>,
    pub(crate) eventhandlers: Option<Box<HashMap<&'static str, eventHandler>>>,
    pub(crate) children:      Vec<Node>
}

impl Node {
    pub(crate) const fn new_element(tag: &'static str) -> Self {
        Self::Element(Element {
            tag,
            attributes:    None,
            eventhandlers: None,
            children:      Vec::new()
        })
    }
}

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
                    if let Some(attributes) = attributes {                        
                        for (name, value) in *attributes {
                            node.set_attribute(name, &value)?;
                        }
                    }
                    if let Some(eventhandlers) = eventhandlers {                        
                        for (event, handler) in *eventhandlers {
                            let handler = handler.into_wasm_closure();
                            node.add_event_listener_with_callback(event, handler.as_ref().unchecked_ref())?;
                            handler.forget();
                        }
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
