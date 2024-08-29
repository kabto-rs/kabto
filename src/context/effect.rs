use super::Context;
use crate::{dom::DOM, JSResult, JsCast};
use crate::vdom::{VNode, VDOM};


pub(crate) enum Effect {
    CreateChild { new_child: VDOM, parent: DOM },
    Delete(DOM),
    UpdateContent { target: DOM, old: VDOM, new: VDOM },
}

impl Effect {
    pub(crate) fn run(self, ctx: &Context) -> JSResult<()> {
        Ok(match self {
            Effect::CreateChild { new_child, parent } => {
                match &*new_child {
                    VNode::Text(text) => {
                        let node = ctx.document.create_text_node(text);
                        parent.append_child(node.unchecked_ref())?;
                    }
                    VNode::Element(element) => {
                        let node = ctx.document.create_element(element.tag())?;
                        for (name, value) in element.attributes().into_iter().flatten() {
                            node.set_attribute(name, value)?;
                        }
                        for (event, listener) in element.eventhandlers().into_iter().flatten() {
                            node.add_event_listener_with_callback(event, listener)?;
                        }
                        parent.append_child(node.unchecked_ref())?;
                    }
                }
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
