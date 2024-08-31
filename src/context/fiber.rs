use crate::{dom::DOM, vdom::VDOM};
use crate::util::{RcX, WeakX};


pub(super) struct Fiber(RcX<FiberNode>);

struct FiberNode {
    vdom:    VDOM,
    dom:     Option<DOM>,
    parent:  WeakX<FiberNode>,
    sibling: RcX<FiberNode>,
    child:   RcX<FiberNode>,
}


