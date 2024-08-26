use crate::vdom::{Element, Tag};
use crate::vdom::eventhandler::{Event, EventHandler};


macro_rules! register_eventhandlers {
    ($($handler:ident: $event_name:ident $event_object:ty;)*) => {
        #[cfg(debug_assertions)]
        fn __assert_exaustive__(e: Event) {
            match e {$(| Event::$event_name => (),)*}
        }

        impl<const T: Tag> Element<T> {$(
            pub fn $handler<__>(mut self, f: impl EventHandler<$event_object, __>) -> Self {
                if self.eventhandlers.is_none() {
                    self.eventhandlers = Some(Default::default())
                }

                unsafe {self.eventhandlers.as_mut().unwrap_unchecked()}
                    .insert(Event::$event_name.lit(), f.into_eventhandler());

                self
            }
        )*}
    };
} register_eventhandlers! {
    on_animationcancel:    animationcancel    web_sys::AnimationEvent;
    on_animationend:       animationend       web_sys::AnimationEvent;
    on_animationiteration: animationiteration web_sys::AnimationEvent;
    on_animationstart:     animationstart     web_sys::AnimationEvent;

    on_auxclick:    auxclick    web_sys::MouseEvent;
    on_contextmenu: contextmenu web_sys::MouseEvent;
    on_dblclick:    dblclick    web_sys::MouseEvent;
    on_mousedown:   mousedown   web_sys::MouseEvent;
    on_mouseenter:  mouseenter  web_sys::MouseEvent;
    on_mouseleave:  mouseleave  web_sys::MouseEvent;
    on_mousemove:   mousemove   web_sys::MouseEvent;
    on_mouseout:    mouseout    web_sys::MouseEvent;
    on_mouseover:   mouseover   web_sys::MouseEvent;
    on_mouseup:     mouseup     web_sys::MouseEvent;

    on_click:              click              web_sys::PointerEvent;
    on_gotpointercapture:  gotpointercapture  web_sys::PointerEvent;
    on_lostpointercapture: lostpointercapture web_sys::PointerEvent;
    on_pointercancel:      pointercancel      web_sys::PointerEvent;
    on_pointerdown:        pointerdown        web_sys::PointerEvent;
    on_pointerenter:       pointerenter       web_sys::PointerEvent;
    on_pointerleave:       pointerleave       web_sys::PointerEvent;
    on_pointermove:        pointermove        web_sys::PointerEvent;
    on_pointerout:         pointerout         web_sys::PointerEvent;
    on_pointerover:        pointerover        web_sys::PointerEvent;
    on_pointerrawupdate:   pointerrawupdate   web_sys::PointerEvent;
    on_pointerup:          pointerup          web_sys::PointerEvent;

    on_beforeinput: beforeinput web_sys::InputEvent;

    on_blur:     blur     web_sys::FocusEvent;
    on_focus:    focus    web_sys::FocusEvent;
    on_focusin:  focusin  web_sys::FocusEvent;
    on_focusout: focusout web_sys::FocusEvent;

    on_compositionend:    compositionend    web_sys::CompositionEvent;
    on_compositionstart:  compositionstart  web_sys::CompositionEvent;
    on_compositionupdate: compositionupdate web_sys::CompositionEvent;

    on_keydown:  keydown  web_sys::KeyboardEvent;
    on_keypress: keypress web_sys::KeyboardEvent;
    on_keyup:    keyup    web_sys::KeyboardEvent;

    on_touchcancel: touchcancel web_sys::TouchEvent;
    on_touchend:    touchend    web_sys::TouchEvent;
    on_touchmove:   touchmove   web_sys::TouchEvent;
    on_touchstart:  touchstart  web_sys::TouchEvent;

    on_transitioncancel: transitioncancel web_sys::TransitionEvent;
    on_transitionend:    transitionend    web_sys::TransitionEvent;
    on_transitionrun:    transitionrun    web_sys::TransitionEvent;
    on_transitionstart:  transitionstart  web_sys::TransitionEvent;

    on_wheel: wheel web_sys::WheelEvent;

    on_beforematch:      beforematch      web_sys::Event;
    on_change:           change           web_sys::Event;
    on_fullscreenchange: fullscreenchange web_sys::Event;
    on_fullscreenerror:  fullscreenerror  web_sys::Event;
    on_input:            input            web_sys::Event;
    on_load:             load             web_sys::Event;
    on_scroll:           scroll           web_sys::Event;
    on_scrollend:        scrollend        web_sys::Event;
}
