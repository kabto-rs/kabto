use web_sys::wasm_bindgen::closure::Closure;
use crate::{vdom::Node, window, JsCast, JSResult, Internals};
use crate::fiber::{Fiber, FiberNode, RcX};


pub(crate) fn schedule_callback(
    commit_root:          fn(&'static Internals),
    perform_unit_of_work: fn(Fiber, &'static Internals)->JSResult<Option<RcX<FiberNode>>>,
    internals:            &'static Internals,
) -> JSResult<()> {
    window().request_idle_callback(Closure::<dyn Fn(web_sys::IdleDeadline)->JSResult<()>>::new(
        move |deadline| {
            work_loop(deadline, commit_root, perform_unit_of_work, internals)
        }
    ).into_js_value().unchecked_ref())?;
    Ok(())
}

fn work_loop(
    deadline:             ::web_sys::IdleDeadline,
    commit_root:          fn(&'static Internals),
    perform_unit_of_work: fn(Fiber, &'static Internals)->JSResult<Option<RcX<FiberNode>>>,
    internals:            &'static Internals,
) -> JSResult<()> {
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
    window().request_idle_callback(Closure::<dyn Fn(web_sys::IdleDeadline)->JSResult<()>>::new(
        move |deadline| {
            work_loop(deadline, commit_root, perform_unit_of_work, internals)
        }
    ).into_js_value().unchecked_ref())?;
    Ok(())
}
