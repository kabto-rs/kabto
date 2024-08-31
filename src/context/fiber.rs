use crate::vdom::VNode;
use crate::{dom::DOM, vdom::VDOM};
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

impl From<VDOM> for Fiber {
    fn from(vdom: VDOM) -> Self {
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
        for child_vnode in vnode.children().cloned().into_iter().flatten() {
            let mut child_fnode = FiberNode::rcx_from(child_vnode);
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
