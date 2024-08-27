use web_sys::wasm_bindgen::closure::Closure;
use crate::{vdom::Node, window, JsCast, JsValue};
use crate::fiber::{Fiber, Internals};


pub(crate) fn schedule_callback(
    commit_root: (/* todo */),
    perform_unit_of_work: (/* todo */),
    internals: &'static Internals,
) -> Result<(), JsValue> {
    window().request_idle_callback(Closure::<dyn Fn(web_sys::IdleDeadline)->Result<(), JsValue>>::new(
        move |deadline| {
            work_loop(deadline, commit_root, perform_unit_of_work, internals)
        }
    ).into_js_value().unchecked_ref())?;
    Ok(())
}

fn work_loop(
    deadline: web_sys::IdleDeadline,
    commit_root: (/* todo */),
    perform_unit_of_work: (/* todo */),
    internals: &'static Internals,
) -> Result<(), JsValue> {
    let mut should_yield = false;
    /*
    while (internals.nextUnitOfWork && !shouldYield) {
        internals.nextUnitOfWork = performUnitOfWork(
            internals.nextUnitOfWork,
            internals
        );
        shouldYield = deadline.timeRemaining() < 1;
    }
    */
    window().request_idle_callback(Closure::<dyn Fn(web_sys::IdleDeadline)->Result<(), JsValue>>::new(
        move |deadline| {
            work_loop(deadline, commit_root, perform_unit_of_work, internals)
        }
    ).into_js_value().unchecked_ref())?;
    Ok(())
}
