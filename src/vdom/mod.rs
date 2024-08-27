pub(crate) mod eventhandler;

pub(crate) use self::eventhandler::eventHandler;
use std::{borrow::Cow, collections::HashMap, marker::PhantomData};
use web_sys::wasm_bindgen::{JsValue, JsCast};


pub enum Node {
    Text(Text),
    Element(Element<()>),
}

//#[derive(Clone)]
pub struct Text(
    Cow<'static, str>
);
const _: () = {
    impl Into<Cow<'static, str>> for Text {
        fn into(self) -> Cow<'static, str> {
            self.0
        }
    }
    impl std::ops::Deref for Text {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl From<String> for Text {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
    impl From<&'static str> for Text {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<bool> for Text {
        fn from(value: bool) -> Self {
            Self((if value {"true"} else {"false"}).into())
        }
    }
    macro_rules! integer_value {
        ($($t:ty)*) => {$(
            impl From<$t> for Text {
                fn from(value: $t) -> Self {
                    Self(value.to_string().into())
                }
            }
        )*};
    } integer_value! { u8 usize i32 }
};

pub struct Element<T: Tag> {t: PhantomData<fn()->T>,
    pub(crate) tag:   &'static str,
    pub(crate) props: Props,
}

pub struct Props {
    pub(crate) attributes:    Option<Box<HashMap<&'static str, Text>>>,
    pub(crate) eventhandlers: Option<Box<HashMap<&'static str, eventHandler>>>,
    pub(crate) children:      Vec<Node>,
}

macro_rules! typed_tag {
    ($($name:ident)*) => {
        pub trait Tag {
            const NAME: &'static str;
        }
        impl Tag for () {
            const NAME: &'static str = "";
        }
        pub(crate) mod tag {$(
            pub struct $name;
            impl super::Tag for $name {
                const NAME: &'static str = stringify!($name);
            }
        )*}
    };
} typed_tag! {
    a
    abbr
    address
    area
    article
    aside
    audio
    b
    base
    bdi
    blockquote
    body
    br
    button
    canvas
    caption
    circle
    cite
    code
    col
    colgroup
    data
    datalist
    dd
    del
    details
    dfn
    dialog
    div
    dl
    dt
    em
    embed
    fencedframe
    fieldset
    figcaption
    figure
    footer
    form
    h1
    h2
    h3
    h4
    h5
    h6
    head
    header
    hgroup
    hr
    html
    i
    iframe
    img
    input
    ins
    kbd
    label
    legend
    li
    link
    main
    map
    mark
    menu
    meta
    meter
    nav
    noscript
    object
    ol
    optgroup
    option
    output
    p
    path
    picture
    portal
    pre
    progress
    q
    rp
    rt
    ruby
    s
    samp
    script
    search
    section
    select
    slot
    small
    source
    span
    strong
    style
    sub
    summary
    sup
    svg
    table
    tbody
    td
    template
    textarea
    tfoot
    th
    thead
    time
    title
    tr
    track
    u
    ul
    var
    video
    wbr
}

impl<T: Tag> Element<T> {
    pub(crate) const fn new() -> Self {
        Element {t: PhantomData,
            tag:   T::NAME,
            props: Props {
                attributes:    None,
                eventhandlers: None,
                children:      Vec::new()
            }
        }
    }

    pub(crate) fn into_node(self) -> Node {
        Node::Element(unsafe {std::mem::transmute(self)})
    }
}

impl Node {
    pub fn render_to(self, container: &web_sys::Node) -> Result<(), JsValue> {
        let document = crate::document();

        match self {
            Node::Text(text) => {
                let node = document.create_text_node(&text);
                container.append_child(&node)?;
            }
            Node::Element(Element { t:_, tag, props }) => {
                let node = document.create_element(tag)?; {
                    if let Some(attributes) = props.attributes {                        
                        for (name, value) in *attributes {
                            node.set_attribute(name, &value)?;
                        }
                    }
                    if let Some(eventhandlers) = props.eventhandlers {                        
                        for (event, handler) in *eventhandlers {
                            let handler = handler.into_wasm_closure();
                            node.add_event_listener_with_callback(event, handler.into_js_value().unchecked_ref())?;
                        }
                    }
                    for child in props.children {
                        child.render_to(&node)?;
                    }
                }
                container.append_child(&node)?;
            }
        }

        Ok(())
    }
}
