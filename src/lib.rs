#![allow(
    non_snake_case,
    non_camel_case_types,
    incomplete_features,
)]

#![feature(
    adt_const_params,
    tuple_trait,
    fn_traits,
    unboxed_closures,
)]

mod dsl;
mod vdom;

pub use dsl::tag;
pub use dsl::nodes::IntoNodes;

pub use ::web_sys::{AnimationEvent, MouseEvent, PointerEvent, FocusEvent, CompositionEvent, KeyboardEvent, TouchEvent, TransitionEvent, WheelEvent, Event, UiEvent};
pub use ::web_sys::wasm_bindgen::{JsValue, JsCast};


pub fn window() -> ::web_sys::Window {
    use web_sys::wasm_bindgen::UnwrapThrowExt;
    ::web_sys::window().expect_throw("`window` not found")
}

pub fn document() -> ::web_sys::Document {
    use web_sys::wasm_bindgen::UnwrapThrowExt;
    window().document().expect_throw("`document` not found")
}

pub fn render(
    nodes: impl IntoNodes,
    root:  &web_sys::Node
) -> Result<(), JsValue> {
    use dsl::nodes::Nodes;

    match nodes.into_nodes() {
        Nodes::None        => Ok(()),
        Nodes::Some(node)  => node.render_to(root),
        Nodes::Many(nodes) => {
            for node in nodes {node.render_to(root)?}
            Ok(())
        }
    }
}
