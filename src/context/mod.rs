use crate::{dom::DOM, vdom::VDOM};


pub(crate) struct Context {
    current_vdom:  VDOM,
    previous_vdom: Option<VDOM>,
    effects:       Vec<Effect>,
}

pub(crate) enum Effect {
    CreateChild { it: VDOM, parent: DOM }
}
