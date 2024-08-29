use super::Context;
use crate::{dom::DOM, JsResult, JsCast};
use crate::vdom::{VNode, VDOM};


pub(crate) enum Effect {
    CreateChild { new_child: VDOM, parent: DOM },
    Delete(DOM),
    UpdateContent { target: DOM, old: VDOM, new: VDOM },
}

impl Effect {
    pub(crate) fn run(self, ctx: &Context) -> JsResult<()> {
        Ok(match self {
            Effect::CreateChild { new_child, parent } => {
                let dom = new_child.create_dom(ctx)?;
                parent.append_child(&dom)?;
            }
            Effect::Delete(dom) => {
                dom.remove()
            }
            Effect::UpdateContent { target, old, new } => {
                #[cfg(debug_assertions)] {
                    assert_eq!(old.kind(), new.kind())
                }
                match (&*old, &*new) {
                    (VNode::Text(_), VNode::Text(new)) => {
                        target.replace_with_with_str_1(new)?;
                    }
                    (VNode::Element(old), VNode::Element(new)) => {
                        for (name, _) in old.attributes().into_iter().flatten() {
                            target.remove_attribute(name)?;
                        }
                        for (event, listener) in old.eventhandlers().into_iter().flatten() {
                            target.remove_event_listener_with_callback(event, listener)?;
                        }

                        for (name, value) in new.attributes().into_iter().flatten() {
                            target.set_attribute(name, value)?;
                        }
                        for (event, listener) in new.eventhandlers().into_iter().flatten() {
                            target.add_event_listener_with_callback(event, listener)?;
                        }
                    }
                    _ => unreachable!()
                }
            }
        })
    }
}
