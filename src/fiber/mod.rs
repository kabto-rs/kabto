use crate::{document, JSResult, JsCast, JsValue};
use crate::vdom::{eventHandler, Node, Text};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use ::web_sys::Node as DOM;


pub(crate) struct Fiber {
    pub(crate) kind:          Kind,
    pub(crate) dom:           Option<DOM>,
    pub(crate) parent:        Option<Weak<Fiber>>,
    pub(crate) attributes:    Option<Box<HashMap<&'static str, Text>>>,
    pub(crate) eventhandlers: Option<Box<HashMap<&'static str, eventHandler>>>,
}

pub(crate) enum Kind {
    TEXT_ELEMENT,
    Element(&'static str)
}

#[derive(Clone)]
pub(crate) struct Internals {
    next_unit_of_work: Option<(/* todo */)>,
    current_root:      Option<(/* todo */)>,
    wip_rot:           Option<(/* todo */)>,
    deletions:         Option<(/* todo */)>,
    wip_fiber:         Option<(/* todo */)>,
    hook_index:        Option<(/* todo */)>,
}

impl Fiber {
    fn create_dom(&self) -> JSResult<DOM> {
        match self.kind {
            Kind::TEXT_ELEMENT => {
                let text = document().create_text_node("");
                Ok(text.unchecked_into())
            }
            Kind::Element(tag) => {
                let element = document().create_element(tag)?;
                if let Some(attributes) = &self.attributes {
                    for (name, value) in &**attributes {
                        element.set_attribute(name, &value)?;
                    }
                }
                if let Some(eventhandlers) = &self.eventhandlers {
                    for (event, handler) in &**eventhandlers {
                        let handler = handler.clone().into_wasm_closure();
                        element.add_event_listener_with_callback(event, handler.into_js_value().unchecked_ref())?;
                    }
                }
                Ok(element.unchecked_into())
            }
        }
    }

    fn perform_unit_of_work(mut self, internals: Internals) -> JSResult<()> {
        if self.dom.is_none() {
            self.dom = Some(self.create_dom()?);
        }

        if let Some(parent) = &mut self.parent {
            // parent = 
        }

        Ok(())
    }
}
