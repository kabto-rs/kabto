use crate::vdom::{Element, Tag};
use crate::vdom::eventhandler::{EventHandler};


macro_rules! register_eventhandlers {
    (
        global {$(
            $handler:ident: $event_name:ident $event_object:ident;
        )*}
        $($tag:ident {$(
            $handler2:ident: $event_name2:ident $event_object2:ident;
        )*})*
    ) => {
        impl<const T: Tag> Element<T> {$(
            pub fn $handler<__>(mut self, f: impl EventHandler<web_sys::$event_object, __>) -> Self {
                if self.eventhandlers.is_none() {
                    self.eventhandlers = Some(Default::default())
                }

                unsafe {self.eventhandlers.as_mut().unwrap_unchecked()}
                    .insert(stringify!($event_name), f.into_eventhandler());

                self
            }
        )*}
        $(impl Element<{Tag::$tag}> {$(
            pub fn $handler2<__>(mut self, f: impl EventHandler<web_sys::$event_object2, __>) -> Self {
                if self.eventhandlers.is_none() {
                    self.eventhandlers = Some(Default::default())
                }

                unsafe {self.eventhandlers.as_mut().unwrap_unchecked()}
                    .insert(stringify!($event_name2), f.into_eventhandler());

                self
            }
        )*})*
    };
} register_eventhandlers! {
    global {
        on_animationcancel:    animationcancel    AnimationEvent;
        on_animationend:       animationend       AnimationEvent;
        on_animationiteration: animationiteration AnimationEvent;
        on_animationstart:     animationstart     AnimationEvent;

        on_auxclick:    auxclick    MouseEvent;
        on_contextmenu: contextmenu MouseEvent;
        on_dblclick:    dblclick    MouseEvent;
        on_mousedown:   mousedown   MouseEvent;
        on_mouseenter:  mouseenter  MouseEvent;
        on_mouseleave:  mouseleave  MouseEvent;
        on_mousemove:   mousemove   MouseEvent;
        on_mouseout:    mouseout    MouseEvent;
        on_mouseover:   mouseover   MouseEvent;
        on_mouseup:     mouseup     MouseEvent;

        on_click:              click              PointerEvent;
        on_gotpointercapture:  gotpointercapture  PointerEvent;
        on_lostpointercapture: lostpointercapture PointerEvent;
        on_pointercancel:      pointercancel      PointerEvent;
        on_pointerdown:        pointerdown        PointerEvent;
        on_pointerenter:       pointerenter       PointerEvent;
        on_pointerleave:       pointerleave       PointerEvent;
        on_pointermove:        pointermove        PointerEvent;
        on_pointerout:         pointerout         PointerEvent;
        on_pointerover:        pointerover        PointerEvent;
        on_pointerrawupdate:   pointerrawupdate   PointerEvent;
        on_pointerup:          pointerup          PointerEvent;

        on_beforeinput: beforeinput InputEvent;

        on_blur:     blur     FocusEvent;
        on_focus:    focus    FocusEvent;
        on_focusin:  focusin  FocusEvent;
        on_focusout: focusout FocusEvent;

        on_compositionend:    compositionend    CompositionEvent;
        on_compositionstart:  compositionstart  CompositionEvent;
        on_compositionupdate: compositionupdate CompositionEvent;

        on_keydown:  keydown  KeyboardEvent;
        on_keypress: keypress KeyboardEvent;
        on_keyup:    keyup    KeyboardEvent;

        on_touchcancel: touchcancel TouchEvent;
        on_touchend:    touchend    TouchEvent;
        on_touchmove:   touchmove   TouchEvent;
        on_touchstart:  touchstart  TouchEvent;

        on_transitioncancel: transitioncancel TransitionEvent;
        on_transitionend:    transitionend    TransitionEvent;
        on_transitionrun:    transitionrun    TransitionEvent;
        on_transitionstart:  transitionstart  TransitionEvent;

        on_wheel: wheel WheelEvent;

        on_beforematch:      beforematch      Event;
        on_change:           change           Event;
        on_fullscreenchange: fullscreenchange Event;
        on_fullscreenerror:  fullscreenerror  Event;
        on_input:            input            Event;
        on_load:             load             Event;
        on_scroll:           scroll           Event;
        on_scrollend:        scrollend        Event;
    }
    body {
        on_afterprint:   afterprint   Event;
        on_beforeprint:  beforeprint  Event;
        on_beforeunload: beforeunload Event;
        on_offline:      offline      Event;
        on_online:       online       Event;

        on_resize:       resize       UiEvent;
    }
}
