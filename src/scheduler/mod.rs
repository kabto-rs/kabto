use crate::{window, JsCast, JSResult, Internals};
use crate::fiber::Fiber;
use ::web_sys::{IdleDeadline, wasm_bindgen::closure::Closure};


pub(crate) fn schedule_callback(
    commit_root:          fn(Internals),
    perform_unit_of_work: fn(Fiber, Internals)->JSResult<Option<Fiber>>,
    internals:            Internals,
) -> JSResult<()> {
    #[cfg(debug_assertions)] {
        web_sys::console::log_1(web_sys::Text::new_with_data(
            "`schedule_callback` called"
        )?.as_ref())
    }

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
    perform_unit_of_work: fn(Fiber, Internals)->JSResult<Option<Fiber>>,
    mut internals:        Internals,
) -> JSResult<()> {
    #[cfg(debug_assertions)] {
        web_sys::console::log_1(web_sys::Text::new_with_data(
            "`work_loop` called"
        )?.as_ref())
    }

    let mut should_yield = false;
    while internals.next_unit_of_work.is_some() && !should_yield {
        internals.next_unit_of_work = perform_unit_of_work(
            internals.next_unit_of_work.as_ref().unwrap().clone(),
            internals.clone()
        )?;
        should_yield = deadline.time_remaining() < 1.
    }

    window().request_idle_callback(Closure::<dyn Fn(web_sys::IdleDeadline)->JSResult<()>>::new(
        move |deadline| {
            work_loop(deadline, commit_root, perform_unit_of_work, internals.clone())
        }
    ).into_js_value().unchecked_ref())?;

    Ok(())
}
