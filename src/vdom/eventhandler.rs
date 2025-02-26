use crate::JSResult;
use std::{future::Future, rc::Rc};
use web_sys::wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::js_sys::Function;
use wasm_bindgen_futures::spawn_local;


#[derive(Clone)]
pub struct eventHandler {
    handler: Rc<dyn Fn(JsValue)>
}
impl Into<Function> for eventHandler {
    fn into(self) -> Function {
        self.into_wasm_closure().into_js_value().unchecked_into()
    }
}
impl eventHandler {
    pub(crate) fn into_wasm_closure(self) -> Closure<dyn Fn(JsValue)> {
        Closure::new(move |js_value| (&*self.handler)(js_value))
    }
}

pub trait EventHandler<Ev, __> {
    fn into_eventhandler(self) -> eventHandler;
}

const _: (/* with event */) = {
    impl<F, E> EventHandler<E, fn(E)> for F
    where
        F: Fn(E) + 'static,
        E: JsCast + Into<web_sys::Event>
    {
        fn into_eventhandler(self) -> eventHandler {
            eventHandler {
                handler: Rc::new(move |js_value| self(E::unchecked_from_js(js_value)))
            }
        }
    }

    impl<F, E> EventHandler<E, fn(E)->JSResult<()>> for F
    where
        F: Fn(E)->JSResult<()> + 'static,
        E: JsCast + Into<web_sys::Event>
    {
        fn into_eventhandler(self) -> eventHandler {
            eventHandler {
                handler: Rc::new(move |js_value| {
                    if let Err(err) = self(E::unchecked_from_js(js_value)) {
                        web_sys::console::log_1(&err)
                    }
                })
            }
        }
    }

    impl<F, Fut, E> EventHandler<E, fn(E)->((),)> for F
    where
        F:   Fn(E) -> Fut + 'static,
        Fut: Future<Output = ()> + 'static,
        E:   JsCast + Into<web_sys::Event>
    {
        fn into_eventhandler(self) -> eventHandler {
            eventHandler {
                handler: Rc::new(move |js_value| spawn_local(
                    self(E::unchecked_from_js(js_value))
                ))
            }
        }
    }

    impl<F, Fut, E> EventHandler<E, fn(E)->(JSResult<()>,)> for F
    where
        F:   Fn(E) -> Fut + 'static,
        Fut: Future<Output = JSResult<()>> + 'static,
        E:   JsCast + Into<web_sys::Event>
    {
        fn into_eventhandler(self) -> eventHandler {
            eventHandler {
                handler: Rc::new(move |js_value| {
                    let res = self(E::unchecked_from_js(js_value));
                    spawn_local(async {
                        if let Err(err) = res.await {
                            web_sys::console::log_1(&err)
                        }
                    })
                })
            }
        }
    }
};
