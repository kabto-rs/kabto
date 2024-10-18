use super::{Context, Effect};
use crate::JsResult;
use crate::dom::DOM;
use crate::vdom::{VDOM, VNode};
use crate::util::{RcX, WeakX};


#[derive(Clone)]
pub(super) struct Fiber(RcX<FiberNode>);

#[derive(Clone)]
struct FiberNode {
    vdom:    VDOM,
    dom:     Option<DOM>,
    parent:  Option<WeakX<FiberNode>>,
    sibling: Option<RcX<FiberNode>>,
    child:   Option<RcX<FiberNode>>,
}

impl Fiber {
    pub fn new(vdom: VDOM) -> Self {
        Self(FiberNode::rcx_from(VNode::from(vdom)))
    }
}
impl FiberNode {
    fn rcx_from(vnode: VNode) -> RcX<FiberNode> {
        let mut node = RcX::new(FiberNode {
            vdom:    vnode.clone().into(),
            dom:     None,
            parent:  None,
            child:   None,
            sibling: None,
        });

        let mut prev_sibling: RcX<FiberNode> = node.clone();
        for child_vnode in vnode.children().into_iter().flatten() {
            let mut child_fnode = FiberNode::rcx_from(child_vnode.clone());
            if node.child.is_none() {
                node.child = Some(child_fnode.clone());
                child_fnode.parent = Some(node.downgrade());
            } else {
                prev_sibling.sibling = Some(child_fnode.clone());
                prev_sibling = child_fnode;
            }
        }

        node
    }
}

impl Fiber {
    pub(super) fn traverse(&mut self, ctx: &Context) -> JsResult<Vec<super::Effect>> {
        let mut effects = vec![];

        let mut prev_target = ctx.previous_fiber.as_ref().map(|f| f.0.clone());
        let mut target  = &mut self.0;
        {
            if target.dom.is_none() {
                target.dom = Some(target.vdom.create_dom(ctx)?)
            }


        }

        Ok(effects)
    }
}
