mod effect;
use effect::Effect;

use crate::{dom::DOM, vdom::VDOM};


pub(crate) struct Context {
    document:      web_sys::Document,
    current_vdom:  VDOM,
    previous_vdom: Option<VDOM>,
    effects:       Vec<Effect>,
}
