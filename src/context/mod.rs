mod effect;
use effect::Effect;

use crate::vdom::VDOM;


pub(crate) struct Context {
    current_vdom:  Option<VDOM>,
    previous_vdom: Option<VDOM>,
    document:      web_sys::Document,
    effects:       Vec<Effect>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            current_vdom:  None,
            previous_vdom: None,
            document:      crate::document(),
            effects:       Vec::new()
        }
    }

    pub(crate) fn document(&self) -> &web_sys::Document {
        &self.document
    }

    pub(crate) fn insert(&mut self, vdom: VDOM) {
        std::mem::swap(&mut self.current_vdom, &mut self.previous_vdom);
        self.current_vdom = Some(vdom)
    }
}
