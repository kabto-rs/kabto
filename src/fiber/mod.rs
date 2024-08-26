use crate::vdom::Node;


pub(crate) struct Fiber {
    pub(crate) dom:    Option<Node>,
    pub(crate) parent: Option<Node>,
}
