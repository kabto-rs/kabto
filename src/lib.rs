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
    sync_unsafe_cell,
)]

mod context;
mod dsl;
mod vdom;
mod dom;
mod util;

pub use dsl::{tag, component::Component};

pub use ::web_sys::{console, Text, Element};
pub use ::web_sys::wasm_bindgen::{JsValue, JsCast, UnwrapThrowExt};
pub mod event {pub use ::web_sys::{AnimationEvent, MouseEvent, PointerEvent, FocusEvent, CompositionEvent, KeyboardEvent, TouchEvent, TransitionEvent, WheelEvent, Event, UiEvent};}

pub type JsResult<T> = Result<T, JsValue>;

pub fn window() -> ::web_sys::Window {
    use web_sys::wasm_bindgen::UnwrapThrowExt;
    ::web_sys::window().expect_throw("`window` not found")
}

pub fn document() -> ::web_sys::Document {
    use web_sys::wasm_bindgen::UnwrapThrowExt;
    window().document().expect_throw("`document` not found")
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        $crate::console::log_1(&$crate::Text::new_with_data(
            &format_args!($($t)*).to_string()
        ).unwrap())
    };
}

// pub fn render(
//     component: impl Component,
//     root:      impl Into<web_sys::Element>
// ) -> JsResult<()> {
//     use fiber::{Fiber, FiberNode};
//     use vdom::{Node, Element, Props};
// 
//     let mut internals = Internals::get();
// 
//     let root = Fiber::from(FiberNode {
//         vdom: Node::Element(Element::with(Props {
//             attributes:    None,
//             eventhandlers: None,
//             children:      component.into_nodes().into()
//         })),
//         dom: Some(root.into().into()),
//         parent:    None,
//         sibling:   None,
//         child:     None,
//         effect:    None,
//         alternate: internals.current_root.clone(),
//     });
// 
//     internals.next_unit_of_work = Some(root.clone());
//     internals.wip_root          = Some(root.clone());
//     internals.flush_sync()?;
// 
//     root.forget();
//     Ok({
//         #[cfg(feature="DEBUG")] {
//             console_log!("`render` finished")
//         }
//     })
// }
