use std::future::Future;
use web_sys::wasm_bindgen::{JsValue, closure::Closure};
use wasm_bindgen_futures::spawn_local;


pub(crate) struct EventHandler {
    class:   EventClass,
    handler: Box<dyn Fn(EventObject)>
}

enum EventClass {
    Animation,
    Mouse,
    Pointer,
    Input,
    Focus,
    Composition,
    Keyboard,
    Touch,
    Transition,
    Wheel,
    Event
}

pub enum EventObject {
    Animation(web_sys::AnimationEvent),
    Composition(web_sys::CompositionEvent),
    Error(web_sys::ErrorEvent),
    Event(web_sys::Event),
    Focus(web_sys::FocusEvent),
    Input(web_sys::InputEvent),
    Keyboard(web_sys::KeyboardEvent),
    Mouse(web_sys::MouseEvent),
    Pointer(web_sys::PointerEvent),
    Progress(web_sys::ProgressEvent),
    Touch(web_sys::TouchEvent),
    Transition(web_sys::TransitionEvent),
    Ui(web_sys::UiEvent),
    Wheel(web_sys::WheelEvent),
    Submit(web_sys::SubmitEvent),
}

#[litenum::to]
pub enum Event {
    // AnimationEvent
    animationcancel,
    animationend,
    animationiteration,
    animationstart,

    // MouseEvent
    auxclick,
    contextmenu,
    dblclick,
    mousedown,
    mouseenter,
    mouseleave,
    mousemove,
    mouseout,
    mouseover,
    mouseup,

    // PointerEvent
    click,
    gotpointercapture,
    lostpointercapture,
    pointercancel,
    pointerdown,
    pointerenter,
    pointerleave,
    pointermove,
    pointerout,
    pointerover,
    pointerrawupdate,
    pointerup,

    // InputEvent
    beforeinput,

    // FocusEvent
    blur,
    focus,
    focusin,
    focusout,

    // CompositionEvent
    compositionend,
    compositionstart,
    compositionupdate,

    // KeyboardEvent
    keydown,
    keypress,
    keyup,

    // TouchEvent
    touchcancel,
    touchend,
    touchmove,
    touchstart,

    // TransitionEvent
    transitioncancel,
    transitionend,
    transitionrun,
    transitionstart,

    // WheelEvent
    wheel,

    // Event
    beforematch,
    fullscreenchange,
    fullscreenerror,
    input,
    scroll,
    scrollend,
}


impl Event {
    fn class(self) -> EventClass {
        match self {
            | Event::animationcancel
            | Event::animationend
            | Event::animationiteration
            | Event::animationstart
            => EventClass::Animation,

            | Event::auxclick
            | Event::contextmenu
            | Event::dblclick
            | Event::mousedown
            | Event::mouseenter
            | Event::mouseleave
            | Event::mousemove
            | Event::mouseout
            | Event::mouseover
            | Event::mouseup
            => EventClass::Mouse,

            | Event::click
            | Event::gotpointercapture
            | Event::lostpointercapture
            | Event::pointercancel
            | Event::pointerdown
            | Event::pointerenter
            | Event::pointerleave
            | Event::pointermove
            | Event::pointerout
            | Event::pointerover
            | Event::pointerrawupdate
            | Event::pointerup
            => EventClass::Pointer,

            | Event::beforeinput
            => EventClass::Input,

            | Event::blur
            | Event::focus
            | Event::focusin
            | Event::focusout
            => EventClass::Focus,

            | Event::compositionend
            | Event::compositionstart
            | Event::compositionupdate
            => EventClass::Composition,

            | Event::keydown
            | Event::keypress
            | Event::keyup
            => EventClass::Keyboard,

            | Event::touchcancel
            | Event::touchend
            | Event::touchmove
            | Event::touchstart
            => EventClass::Touch,

            | Event::transitioncancel
            | Event::transitionend
            | Event::transitionrun
            | Event::transitionstart
            => EventClass::Transition,

            | Event::wheel
            => EventClass::Wheel,

            | Event::beforematch
            | Event::fullscreenchange
            | Event::fullscreenerror
            | Event::input
            | Event::scroll
            | Event::scrollend
            => EventClass::Event,
        }
    }
}

impl EventHandler {
    pub(crate) fn into_wasm_closure(self) -> Closure<dyn Fn(JsValue)> {
        Closure::wrap(Box::new(move |js_value| (self.handler)(match self.class {
            EventClass::Animation   => EventObject::Animation(js_value.into()),
            EventClass::Mouse       => EventObject::Mouse(js_value.into()),
            EventClass::Pointer     => EventObject::Pointer(js_value.into()),
            EventClass::Input       => EventObject::Input(js_value.into()),
            EventClass::Focus       => EventObject::Focus(js_value.into()),
            EventClass::Composition => EventObject::Composition(js_value.into()),
            EventClass::Keyboard    => EventObject::Keyboard(js_value.into()),
            EventClass::Touch       => EventObject::Touch(js_value.into()),
            EventClass::Transition  => EventObject::Transition(js_value.into()),
            EventClass::Wheel       => EventObject::Wheel(js_value.into()),
            EventClass::Event       => EventObject::Event(js_value.into())
        })))
    }
}


////////////////////////////////////////////////


pub trait IntoEventHandler<T> {
    fn into_event_handler(self) -> EventHandler;
}

const _: (/* without event */) = {
    impl<F> IntoEventHandler<fn()> for F
    where
        F: Fn() + 'static
    {
        fn into_event_handler(self) -> EventHandler {
            EventHandler {class:EventClass::Event,
                handler: Box::new(move |_| self())
            }
        }
    }
    
    impl<F> IntoEventHandler<fn()->Result<(), JsValue>> for F
    where
        F: Fn()->Result<(), JsValue> + 'static
    {
        fn into_event_handler(self) -> EventHandler {
            EventHandler {class:EventClass::Event,
                handler: Box::new(move |_| {
                    if let Err(err) = self() {
                        web_sys::console::log_1(&err)
                    }
                })
            }
        }
    }
    
    impl<F, Fut> IntoEventHandler<fn()->(Fut, ())> for F
    where
        F:   Fn() -> Fut + 'static,
        Fut: Future<Output = ()> + 'static
    {
        fn into_event_handler(self) -> EventHandler {
            EventHandler {class:EventClass::Event,
                handler: Box::new(move |_| spawn_local(
                    self()
                ))
            }
        }
    }
    
    impl<F, Fut> IntoEventHandler<fn()->(Fut, Result<(), JsValue>)> for F
    where
        F:   Fn() -> Fut + 'static,
        Fut: Future<Output = Result<(), JsValue>> + 'static
    {
        fn into_event_handler(self) -> EventHandler {
            EventHandler {class:EventClass::Event,
                handler: Box::new(move |_| {
                    let res = self();
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


