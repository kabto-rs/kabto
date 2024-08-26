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
pub use dsl::nodes::{IntoNodes, NodeCollection};

pub use ::web_sys::{AnimationEvent, MouseEvent, PointerEvent, FocusEvent, CompositionEvent, KeyboardEvent, TouchEvent, TransitionEvent, WheelEvent, Event, UiEvent};
pub use ::web_sys::wasm_bindgen::{JsValue, JsCast};
