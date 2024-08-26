use web_sys::wasm_bindgen::closure::Closure;
use crate::{fiber::Fiber, vdom::Node, window, JsCast, JsValue};


pub(crate) fn schedule_callback(
    commit_root: (/* todo */),
    perform_unit_of_work: (/* todo */),
    internals: Internals,
) -> Result<(), JsValue> {
    window().request_idle_callback(Closure::<dyn Fn(web_sys::IdleDeadline)->Result<(), JsValue>>::new(
        move |deadline| {
            work_loop(deadline, commit_root, perform_unit_of_work, internals.clone())
        }
    ).into_js_value().unchecked_ref())?;
    Ok(())
}

fn work_loop(
    deadline: web_sys::IdleDeadline,
    commit_root: (/* todo */),
    perform_unit_of_work: (/* todo */),
    internals: Internals,
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
            work_loop(deadline, commit_root, perform_unit_of_work, internals.clone())
        }
    ).into_js_value().unchecked_ref())?;
    Ok(())
}

fn perform_unit_of_work(fiber: Fiber, internals: Internals) {
    if fiber.dom.is_none() {
        
    }
}

#[derive(Clone)]
pub(crate) struct Internals {
    next_unit_of_work: Option<(/* todo */)>,
    current_root:      Option<(/* todo */)>,
    wip_rot:           Option<(/* todo */)>,
    deletions:         Option<(/* todo */)>,
    wip_fiber:         Option<(/* todo */)>,
    hook_index:        Option<(/* todo */)>,
}
