mod effect;
use effect::Effect;

use crate::vdom::VDOM;


pub(crate) struct Context {
    current_fiber:  Option<VDOM>,
    previous_fiber: Option<VDOM>,
    document:       web_sys::Document,
    effects:        Vec<Effect>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            current_fiber:  None,
            previous_fiber: None,
            document:      crate::document(),
            effects:       Vec::new()
        }
    }

    pub(crate) fn document(&self) -> &web_sys::Document {
        &self.document
    }

    pub(crate) fn insert(&mut self, vdom: VDOM) {
        std::mem::swap(&mut self.current_fiber, &mut self.previous_fiber);
        self.current_fiber = Some(vdom)
    }

    pub(crate) fn commit(&self) {
        match (&self.current_fiber, &self.previous_fiber) {
            (None, _) => (),
            (Some(curr), None) => {
                // just build DOM
            }
            (Some(curr), Some(prev)) => {
                // detect diffs and apply updates
            }
        }
    }
}
