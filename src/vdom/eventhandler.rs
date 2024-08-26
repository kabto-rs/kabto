use std::future::Future;
use web_sys::wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;


pub(crate) struct eventHandler {
    class:   EventClass,
    handler: Box<dyn Fn(JsValue)>
}

pub enum EventClass {
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
    change,
    fullscreenchange,
    fullscreenerror,
    input,
    load,
    scroll,
    scrollend,
}


//////////////////////////////////////////////////////////////


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
            | Event::change
            | Event::fullscreenchange
            | Event::fullscreenerror
            | Event::input
            | Event::load
            | Event::scroll
            | Event::scrollend
            => EventClass::Event,
        }
    }
}

impl eventHandler {
    pub(crate) fn into_wasm_closure(self) -> Closure<dyn Fn(JsValue)> {
        Closure::wrap(self.handler)
    }
}


////////////////////////////////////////////////


pub trait EventHandler<Ev, __> {
    fn into_eventhandler(self) -> eventHandler;
}

const _: (/* with event */) = {
    pub trait EventObject: JsCast {
        const CLASS: EventClass;
    }
    const _: () = {
        impl EventObject for web_sys::Event {
            const CLASS: EventClass = EventClass::Event;
        }
        impl EventObject for web_sys::AnimationEvent {
            const CLASS: EventClass = EventClass::Animation;
        }
        impl EventObject for web_sys::CompositionEvent {
            const CLASS: EventClass = EventClass::Composition;
        }
        impl EventObject for web_sys::FocusEvent {
            const CLASS: EventClass = EventClass::Focus;
        }
        impl EventObject for web_sys::InputEvent {
            const CLASS: EventClass = EventClass::Input;
        }
        impl EventObject for web_sys::KeyboardEvent {
            const CLASS: EventClass = EventClass::Keyboard;
        }
        impl EventObject for web_sys::MouseEvent {
            const CLASS: EventClass = EventClass::Mouse;
        }
        impl EventObject for web_sys::PointerEvent {
            const CLASS: EventClass = EventClass::Pointer;
        }
        impl EventObject for web_sys::TouchEvent {
            const CLASS: EventClass = EventClass::Touch;
        }
        impl EventObject for web_sys::TransitionEvent {
            const CLASS: EventClass = EventClass::Transition;
        }
        impl EventObject for web_sys::WheelEvent {
            const CLASS: EventClass = EventClass::Wheel;
        }
    };

    impl<F, E> EventHandler<E, fn(E)> for F
    where
        F: Fn(E) + 'static,
        E: EventObject
    {
        fn into_eventhandler(self) -> eventHandler {
            eventHandler {   
                class:   E::CLASS,
                handler: Box::new(move |js_value| self(E::unchecked_from_js(js_value)))
            }
        }
    }

    impl<F, E> EventHandler<E, fn(E)->Result<(), JsValue>> for F
    where
        F: Fn(E)->Result<(), JsValue> + 'static,
        E: EventObject
    {
        fn into_eventhandler(self) -> eventHandler {
            eventHandler {
                class:   E::CLASS,
                handler: Box::new(move |js_value| {
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
        E:   EventObject
    {
        fn into_eventhandler(self) -> eventHandler {
            eventHandler {
                class:   E::CLASS,
                handler: Box::new(move |js_value| spawn_local(
                    self(E::unchecked_from_js(js_value))
                ))
            }
        }
    }

    impl<F, Fut, E> EventHandler<E, fn(E)->(Result<(), JsValue>,)> for F
    where
        F:   Fn(E) -> Fut + 'static,
        Fut: Future<Output = Result<(), JsValue>> + 'static,
        E:   EventObject
    {
        fn into_eventhandler(self) -> eventHandler {
            eventHandler {
                class:   E::CLASS,
                handler: Box::new(move |js_value| {
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
