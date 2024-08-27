mod rcx;
pub(crate) use rcx::{RcX, WeakX};

use crate::internals::Internals;
use crate::{document, JSResult, JsCast, UnwrapThrowExt};
use crate::vdom::{Node, Props};
use ::web_sys::Node as DOM;


pub(crate) struct Fiber {
    root: RcX<FiberNode>
}

#[derive(Clone)]
pub(crate) struct FiberNode {
    pub(crate) kind:    Kind,
    pub(crate) props:   Props,
    pub(crate) dom:     Option<DOM>,
    pub(crate) parent:  Option<WeakX<FiberNode>>,
    pub(crate) sibling: Option<RcX<FiberNode>>,
    pub(crate) child:   Option<RcX<FiberNode>>,
}

#[derive(Clone)]
pub(crate) enum Kind {
    TEXT_ELEMENT,
    Element(&'static str)
}

impl Kind {
    fn of(node: &Node) -> Self {
        match node {
            Node::Element(e) => Self::Element(e.tag),
            Node::Text(_)    => Self::TEXT_ELEMENT
        }
    }
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
                if let Some(attributes) = &self.props.attributes {
                    for (name, value) in &**attributes {
                        element.set_attribute(name, &value)?;
                    }
                }
                if let Some(eventhandlers) = &self.props.eventhandlers {
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
    pub(crate) fn perform_unit_of_work(mut self, internals: &'static Internals) -> JSResult<Option<RcX<FiberNode>>> {
        let Fiber { root:this } = &mut self;

        if this.dom.is_none() {
            this.dom = Some(this.create_dom()?);
        }

        if let Some(parent) = &this.parent {
            parent.upgrade().expect_throw("invalid parent of fiber")
                .dom().append_child(this.dom())?;
        }

        let mut prev_sibling = None;
        for i in 0..this.props.children.len() {
            let next = this.props.children[i].clone();
            let next = RcX::new(FiberNode {
                kind:    Kind::of(&next),
                props:   next.props().cloned().unwrap_or_else(Props::new),
                dom:     None,
                parent:  Some(this.downgrade()),
                sibling: None,
                child:   None
            });

            prev_sibling = Some(next.clone());

            if i == 0 {
                this.child = Some(next)
            } else {
                prev_sibling.unwrap().sibling = Some(next)
            }
        }

        if this.child.is_some() {
            return Ok(Some(this.child.as_ref().unwrap().clone()))
        }

        let mut maybe_next = Some(this.clone());
        while let Some(next) = &maybe_next {
            if next.sibling.is_some() {
                return Ok(Some(next.sibling.as_ref().unwrap().clone()))
            }
            maybe_next = next.parent.as_ref().map(WeakX::upgrade).transpose()?
        }

        Ok(None)
    }
}
