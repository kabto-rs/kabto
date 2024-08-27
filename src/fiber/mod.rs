pub(crate) use crate::util::{RcX, WeakX};

use crate::internals::Internals;
use crate::{document, JSResult, JsCast, UnwrapThrowExt};
use crate::vdom::{Node, Props};
use ::web_sys::Node as DOM;


#[derive(Clone)]
pub(crate) struct Fiber(RcX<FiberNode>);

#[derive(Clone)]
pub(crate) struct FiberNode {
    pub(crate) kind:    Kind,
    pub(crate) props:   Props,
    pub(crate) dom:     Option<DOM>,
    pub(crate) parent:  Option<WeakX<FiberNode>>,
    pub(crate) sibling: Option<RcX<FiberNode>>,
    pub(crate) child:   Option<RcX<FiberNode>>,
}
const _: () = {
    /// SAFETY: single thread
    unsafe impl Send for FiberNode {}
    unsafe impl Sync for FiberNode {}

    impl std::ops::Deref for Fiber {
        type Target = FiberNode;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl From<FiberNode> for Fiber {
        fn from(node: FiberNode) -> Self {
            Self(RcX::new(node))
        }
    }

    #[cfg(debug_assertions)]
    impl Drop for Fiber {
        fn drop(&mut self) {       
            #[cfg(debug_assertions)] {
                crate::console_log!(
                    "`Fiber` droped: remaining {}",
                    self.0.strong_count() - 1
                )
            }
        }
    }
    #[cfg(debug_assertions)]
    impl Drop for FiberNode {
        fn drop(&mut self) {       
            #[cfg(debug_assertions)] {
                crate::console_log!(
                    "`FiberNode` droped"
                )
            }
        }
    }
};

#[derive(Clone, Debug)]
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

impl Fiber {
    pub(crate) fn forget(self) {
        #[cfg(debug_assertions)] {
            crate::console_log!("`Fiber::forget` called")
        }

        std::mem::forget(self)
    }

    pub(crate) fn perform_unit_of_work(mut self, internals: Internals) -> JSResult<Option<Fiber>> {
        let Fiber(this) = &mut self;

        #[cfg(debug_assertions)] crate::console_log!(
            "`Fiber::perform_unit_of_work` by `{:?}`",
            this.kind
        );

        if this.dom.is_none() {
            #[cfg(debug_assertions)] crate::console_log!(
                "`create_dom` by `{:?}`", this.kind
            );

            this.dom = Some(this.create_dom()?);
        }

        if let Some(parent) = &this.parent {
            #[cfg(debug_assertions)] crate::console_log!(
                "found parent of `{:?}`", this.kind
            );

            parent.upgrade()?.dom().append_child(this.dom())?;

            #[cfg(debug_assertions)] crate::console_log!(
                "succeed `{:?}`'s `append_child` to parent `{:?}`",
                this.kind,
                parent.upgrade()?.kind
            );
        }

        let mut prev_sibling: Option<RcX<FiberNode>> = None;
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

            {let next = next.clone();
                if i == 0 {
                    this.child = Some(next)
                } else {
                    prev_sibling.unwrap().sibling = Some(next)
                }
            }

            prev_sibling = Some(next);
        }

        if this.child.is_some() {
            return Ok(Some(Fiber(this.child.as_ref().unwrap().clone())))
        }

        let mut maybe_next = Some(this.clone());
        while let Some(next) = &maybe_next {
            if next.sibling.is_some() {
                return Ok(Some(Fiber(next.sibling.as_ref().unwrap().clone())))
            }
            maybe_next = next.parent.as_ref().map(WeakX::upgrade).transpose()?
        }

        Ok(None)
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
                        element.set_attribute(name, value)?;
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
