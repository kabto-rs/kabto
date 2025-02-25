pub(crate) use crate::util::{RcX, WeakX};
use crate::internals::Internals;
use crate::{document, JSResult, UnwrapThrowExt};
use crate::vdom::{Node as VDOM, NodeKind, Props};
use crate::dom::DOM;


#[derive(Clone)]
pub(crate) struct Fiber(RcX<FiberNode>);

#[derive(Clone)]
pub(crate) struct FiberNode {
    pub(crate) vdom:      VDOM,
    pub(crate) dom:       Option<DOM>,
    pub(crate) parent:    Option<WeakX<FiberNode>>,
    pub(crate) sibling:   Option<RcX<FiberNode>>,
    pub(crate) child:     Option<RcX<FiberNode>>,
    pub(crate) alternate: Option<Fiber>,
    pub(crate) effect:    Option<Effect>
}

#[derive(Clone, Debug)]
pub(crate) enum Effect {
    Update,
    Create,
    Delete
}

#[cfg(debug_assertions)]
const _: () = {
    impl std::fmt::Debug for Fiber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }

    impl std::fmt::Debug for FiberNode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("FiberNode")
                .field("vdom", &self.vdom)
                .field("dom", &self.dom)
                //.field("parent", &self.parent.as_ref().map(|p| p.upgrade()))
                .field("sibling", &self.sibling)
                .field("child", &self.child)
                .field("alternate", &self.alternate)
                .field("affect", &self.effect)
                .finish()
        }
    }
};

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
    impl std::ops::DerefMut for Fiber {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
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

    fn reconcile_children(wip_fiber: &mut RcX<FiberNode>, children: &Vec<VDOM>, mut internals: Internals) {
        #[cfg(feature="DEBUG")] {
            crate::console_log!("`reconcile_children` called");
            crate::console_log!("wip_fiber = {wip_fiber:#?}");
        }

        let mut index = 0;
        let mut prev_sibling: Option<RcX<FiberNode>> = None;
        let mut old_fiber = wip_fiber.alternate.as_ref().map(|f| f.child()).flatten();

        while index < children.len() || old_fiber.is_some() {
            let next = children.get(index).cloned();

            let mut new_fiber = None;
            /*
                due to the `while` condition, here at least one
                of `next`, `old_fiber` is Some.
            */
            let same_kind =
                next.as_ref().map(|n| n.kind()) ==
                old_fiber.as_ref().map(|o| o.kind());
            match (same_kind, &next, &mut old_fiber) {
                | (_, None, None)
                | (true, None, _)
                | (true, _ , None)
                => unreachable!("at least one of `next`, `old_fiber` is Some"),

                (true, Some(next), Some(old)) => {
                    #[cfg(feature="DEBUG")] {
                        crate::console_log!("reconcile::update");
                        crate::console_log!("update: old  = {old:#?}");
                        crate::console_log!("update: next = {next:#?}");
                    }

                    let new_vdom = {
                        let mut new_vdom = old.vdom.clone();
                        if let Some(text) = new_vdom.text_mut() {
                            if let Some(new_text) = next.text() {
                                *text = new_text.clone()
                            }
                        }
                        if let Some(props) = new_vdom.props_mut() {
                            if let Some(next_props) = next.props() {
                                *props = next_props.clone()
                            }
                        }
                        new_vdom
                    };
                    new_fiber = Some(RcX::new(FiberNode {
                        vdom:      new_vdom,
                        dom:       old.dom.clone(),
                        parent:    Some(wip_fiber.downgrade()),
                        alternate: Some(old.clone()),
                        effect:    Some(Effect::Update),
                        sibling:   None,
                        child:     None
                    }))
                }

                (false, None, Some(old)) => {
                    #[cfg(feature="DEBUG")] {
                        crate::console_log!("reconcile::delete");
                        crate::console_log!("delete: old  = {old:#?}");
                    }

                    old.effect = Some(Effect::Delete);
                    internals.deletions.push(old.clone());
                }

                (false, Some(next), _) => {
                    #[cfg(feature="DEBUG")] {
                        crate::console_log!("reconcile::create");
                        crate::console_log!("create: next = {next:#?}");
                    }

                    new_fiber = Some(RcX::new(FiberNode {
                        vdom:      next.clone(),
                        dom:       None,
                        parent:    Some(wip_fiber.downgrade()),
                        alternate: None,
                        effect:    Some(Effect::Create),
                        sibling:   None,
                        child:     None
                    }))
                }
            }

            if old_fiber.is_some() {
                old_fiber = old_fiber.unwrap().sibling()
            }

            {let new_fiber = new_fiber.clone();
                if index == 0 {
                    wip_fiber.child = new_fiber
                } else {
                    prev_sibling.unwrap().sibling = new_fiber
                }
            }

            prev_sibling = new_fiber;

            index += 1
        }
    }
}

impl FiberNode {
    pub(crate) fn dom(&self) -> &DOM {
        self.dom.as_ref().expect_throw("invalid `dom`")
    }
    pub(crate) fn dom_mut(&mut self) -> &mut DOM {
        self.dom.as_mut().expect_throw("invalid `dom`")
    }

    fn kind(&self) -> NodeKind {
        self.vdom.kind()
    }

    fn create_dom(&self) -> JSResult<DOM> {
        match &self.vdom {
            VDOM::Text(text) => {
                let text = document().create_text_node(&text);
                Ok(DOM::Text(text))
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
                        element.add_event_listener_with_callback(event, handler)?;
                    }
                }
                Ok(DOM::Element(element))
            }
        }
    }
}
