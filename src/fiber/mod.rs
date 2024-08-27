mod rcx;

use self::rcx::{RcX, WeakX};
use crate::{document, JSResult, JsCast, UnwrapThrowExt};
use crate::vdom::{eventHandler, Text};
use std::collections::HashMap;
use ::web_sys::Node as DOM;


pub(crate) struct Fiber {
    root: RcX<FiberNode>
}

pub(crate) struct FiberNode {
    pub(crate) kind:          Kind,
    pub(crate) dom:           Option<DOM>,
    pub(crate) parent:        Option<WeakX<FiberNode>>,
    pub(crate) sibling:       Option<RcX<FiberNode>>,
    pub(crate) child:         Option<RcX<FiberNode>>,
    /* props */
    pub(crate) attributes:    Option<Box<HashMap<&'static str, Text>>>,
    pub(crate) eventhandlers: Option<Box<HashMap<&'static str, eventHandler>>>,
    pub(crate) children:      Vec<RcX<FiberNode>>,
}

#[derive(Clone)]
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

impl FiberNode {
    fn dom(&self) -> &DOM {
        self.dom.as_ref().expect_throw("invalid `dom`")
    }

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
}

impl Fiber {
    fn perform_unit_of_work(&mut self, internals: Internals) -> JSResult<()> {
        let Fiber { root:this } = self;

        if this.dom.is_none() {
            this.dom = Some(this.create_dom()?);
        }

        if let Some(parent) = &this.parent {
            parent.upgrade().expect_throw("invalid parent of fiber")
                .dom().append_child(this.dom())?;
        }

        // let mut prev_sibling = None;
        for i in 0..this.children.len() {
            let mut next = this.children[i].clone();
            next.parent = Some(this.downgrade());


        }

        Ok(())
    }
}
