pub(crate) use crate::util::{RcX, WeakX};

use crate::internals::Internals;
use crate::{document, JSResult, JsCast, UnwrapThrowExt};
use crate::vdom::{Node as VDOM, Props};
use ::web_sys::Node as DOM;


#[derive(Clone)]
pub(crate) struct Fiber(RcX<FiberNode>);

#[derive(Clone)]
pub(crate) struct FiberNode {
    pub(crate) vdom:    VDOM,
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

    #[cfg(feature="DEBUG")]
    impl Drop for Fiber {
        fn drop(&mut self) {       
            #[cfg(feature="DEBUG")] {
                crate::console_log!(
                    "`Fiber` droped: remaining {}",
                    self.0.strong_count() - 1
                )
            }
        }
    }
    #[cfg(feature="DEBUG")]
    impl Drop for FiberNode {
        fn drop(&mut self) {       
            #[cfg(feature="DEBUG")] {
                crate::console_log!(
                    "`FiberNode` droped"
                )
            }
        }
    }
};

impl Fiber {
    pub(crate) fn child(&self) -> Option<Fiber> {
        self.child.clone().map(Fiber)
    }
    pub(crate) fn sibling(&self) -> Option<Fiber> {
        self.sibling.clone().map(Fiber)
    }
    pub(crate) fn parent(&self) -> Option<Fiber> {
        self.parent.clone()?.upgrade().ok().map(Fiber)
    }

    pub(crate) fn forget(self) {
        #[cfg(feature="DEBUG")] {
            crate::console_log!("`Fiber::forget` called")
        }

        std::mem::forget(self)
    }

    pub(crate) fn perform_unit_of_work(mut self, internals: Internals) -> JSResult<Option<Fiber>> {
        let Fiber(this) = &mut self;

        #[cfg(feature="DEBUG")] crate::console_log!(
            "`Fiber::perform_unit_of_work` by `{:?}`", this.vdom
        );

        if this.dom.is_none() {
            #[cfg(feature="DEBUG")] crate::console_log!(
                "`create_dom` by `{:?}`", this.vdom
            );

            this.dom = Some(this.create_dom()?);
        }

        let mut prev_sibling: Option<RcX<FiberNode>> = None;
        let children = &this.vdom.props().cloned()
            .unwrap_or_else(Props::new)
            .children;
        for i in 0..children.len() {
            let next = children[i].clone();
            let next = RcX::new(FiberNode {
                vdom:    next,
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
    pub(crate) fn dom(&self) -> &DOM {
        self.dom.as_ref().expect_throw("invalid `dom`")
    }

    fn create_dom(&self) -> JSResult<DOM> {
        match &self.vdom {
            VDOM::Text(text) => {
                let text = document().create_text_node(&text);
                Ok(text.unchecked_into())
            }
            VDOM::Element(e) => {
                let element = document().create_element(e.tag)?;
                if let Some(attributes) = &e.props.attributes {
                    for (name, value) in &**attributes {
                        element.set_attribute(name, value)?;
                    }
                }
                if let Some(eventhandlers) = &e.props.eventhandlers {
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
