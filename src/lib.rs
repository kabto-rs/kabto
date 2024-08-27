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
mod fiber;
mod scheduler;
mod vdom;

pub use dsl::tag;

pub use ::web_sys::wasm_bindgen::{JsValue, JsCast};
pub mod event {pub use ::web_sys::{AnimationEvent, MouseEvent, PointerEvent, FocusEvent, CompositionEvent, KeyboardEvent, TouchEvent, TransitionEvent, WheelEvent, Event, UiEvent};}

pub type JSResult<T> = Result<T, JsValue>;

pub fn window() -> ::web_sys::Window {
    use web_sys::wasm_bindgen::UnwrapThrowExt;
    ::web_sys::window().expect_throw("`window` not found")
}

pub fn document() -> ::web_sys::Document {
    use web_sys::wasm_bindgen::UnwrapThrowExt;
    window().document().expect_throw("`document` not found")
}

pub trait Component: dsl::nodes::IntoNodes {}
impl<IN: dsl::nodes::IntoNodes> Component for IN {}

pub fn render(
    nodes: impl Component,
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
