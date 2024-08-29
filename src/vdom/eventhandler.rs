use crate::JSResult;
use std::future::Future;
use web_sys::wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt};
use web_sys::js_sys::Function;
use wasm_bindgen_futures::spawn_local;


pub trait EventHandler<Ev, __>: Sized {
    fn into_closure(self) -> Closure<dyn Fn(JsValue)>;
    fn into_function(self) -> Function {
        self.into_closure().into_js_value().unchecked_into()
    }
}

const _: () = {
    impl<F, E> EventHandler<E, fn(E)> for F
    where
        F: Fn(E) + 'static,
        E: JsCast + Into<web_sys::Event>
    {
        fn into_closure(self) -> Closure<dyn Fn(JsValue)> {
            Closure::<dyn Fn(JsValue)>::new(move |js_value| {
                self(E::unchecked_from_js(js_value))
            })
        }
    }

    impl<F, E> EventHandler<E, fn(E)->JSResult<()>> for F
    where
        F: Fn(E)->JSResult<()> + 'static,
        E: JsCast + Into<web_sys::Event>
    {
        fn into_closure(self) -> Closure<dyn Fn(JsValue)> {
            Closure::<dyn Fn(JsValue)>::new(move |js_value| {
                self(E::unchecked_from_js(js_value)).unwrap_throw()
            })
        }
    }

    impl<F, Fut, E> EventHandler<E, fn(E)->((),)> for F
    where
        F:   Fn(E) -> Fut + 'static,
        Fut: Future<Output = ()> + 'static,
        E:   JsCast + Into<web_sys::Event>
    {
        fn into_closure(self) -> Closure<dyn Fn(JsValue)> {
            Closure::<dyn Fn(JsValue)>::new(move |js_value| {
                spawn_local(self(E::unchecked_from_js(js_value)))
            })
        }
    }

    impl<F, Fut, E> EventHandler<E, fn(E)->(JSResult<()>,)> for F
    where
        F:   Fn(E) -> Fut + 'static,
        Fut: Future<Output = JSResult<()>> + 'static,
        E:   JsCast + Into<web_sys::Event>
    {
        fn into_closure(self) -> Closure<dyn Fn(JsValue)> {
            Closure::<dyn Fn(JsValue)>::new(move |js_value| {
                let res = self(E::unchecked_from_js(js_value));
                spawn_local(async {res.await.unwrap_throw()})
            })
        }
    }
};
