use crate::{window, JsCast, JSResult, Internals};
use crate::fiber::{Fiber, FiberNode, RcX};
use ::web_sys::{IdleDeadline, wasm_bindgen::closure::Closure};


pub(crate) fn schedule_callback(
    commit_root:          fn(Internals),
    perform_unit_of_work: fn(FiberNode, Internals)->JSResult<Option<RcX<FiberNode>>>,
    internals:            Internals,
) -> JSResult<()> {
    window().request_idle_callback(Closure::<dyn Fn(web_sys::IdleDeadline)->JSResult<()>>::new(
        move |deadline| {
            work_loop(deadline, commit_root, perform_unit_of_work, internals.clone())
        }
    ).into_js_value().unchecked_ref())?;
    Ok(())
}

fn work_loop(
    deadline:             IdleDeadline,
    commit_root:          fn(Internals),
    perform_unit_of_work: fn(FiberNode, Internals)->JSResult<Option<RcX<FiberNode>>>,
    internals:            Internals,
) -> JSResult<()> {
    let mut should_yield = false;
    while internals.next_unit_of_work.is_some() && !should_yield {

    }
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
            work_loop(deadline, commit_root, perform_unit_of_work, internals.clone())
        }
    ).into_js_value().unchecked_ref())?;
    Ok(())
}
