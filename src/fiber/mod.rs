pub(crate) use crate::util::{RcX, WeakX};

use crate::internals::Internals;
use crate::{document, JSResult, JsCast, UnwrapThrowExt};
use crate::vdom::{Node as VDOM, Props};
use ::web_sys::Node as DOM;


#[derive(Clone)]
pub(crate) struct Fiber(RcX<FiberNode>);

#[derive(Clone)]
pub(crate) struct FiberNode {
    pub(crate) vdom:      VDOM,
    pub(crate) dom:       Option<DOM>,
    pub(crate) parent:    Option<WeakX<FiberNode>>,
    pub(crate) sibling:   Option<RcX<FiberNode>>,
    pub(crate) child:     Option<RcX<FiberNode>>,
    pub(crate) alternate: Option<Fiber>
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

        Self::reconcile_children(
            this,
            &this.vdom.props().cloned()
                .unwrap_or_else(Props::new)
                .children,
            internals
        );

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

    fn reconcile_children(wip_fiber: &mut RcX<FiberNode>, children: &Vec<VDOM>, internals: Internals) {
        let mut index = 0;
        let mut prev_sibling: Option<RcX<FiberNode>> = None;
        let mut old_fiber = wip_fiber.alternate.as_ref().map(|f| f.child()).flatten();

        while index < children.len() || old_fiber.is_some() {
            let next = children.get(index).map(Clone::clone);
            let next = next.map(|next| RcX::new(FiberNode {
                vdom:      next,
                dom:       None,
                parent:    Some(wip_fiber.downgrade()),
                sibling:   None,
                child:     None,
                alternate: None
            }));

            match (&next, &old_fiber) {
                (Some(next), Some(old)) => {}
                (Some(next), None) => {}
                (None, Some(old)) => {}
                (None, None) => {}
            }

            if old_fiber.is_some() {
                old_fiber = old_fiber.unwrap().sibling()
            }


            {let next = next.clone();
                if index == 0 {
                    wip_fiber.child = next
                } else {
                    prev_sibling.unwrap().sibling = next
                }
            }

            prev_sibling = next;

            index += 1
        }
    }
}

impl FiberNode {
    pub(crate) fn dom(&self) -> &DOM {
        self.dom.as_ref().expect_throw("invalid `dom`")
    }

    fn kind(&self) -> &'static str {
        match &self.vdom {
            VDOM::Element(e) => e.tag,
            VDOM::Text(_)    => "TEXT_NODE"
        }
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
