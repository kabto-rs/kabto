pub(crate) mod eventhandler;

use self::eventhandler::eventHandler;
use std::{borrow::Cow, collections::HashMap};
use web_sys::wasm_bindgen::{JsValue, JsCast};


pub enum Node {
    Text(Text),
    Element(Element<{Tag::ANY}>),
}

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
            Self(value.to_string().into())
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

pub struct Element<const TAG: Tag> {
    pub(crate) tag:           &'static str,
    pub(crate) attributes:    Option<Box<HashMap<&'static str, Text>>>,
    pub(crate) eventhandlers: Option<Box<HashMap<&'static str, eventHandler>>>,
    pub(crate) children:      Vec<Node>
}

macro_rules! typed_tag {
    ($($name:ident)*) => {
        mod tag {$(
            pub struct $name;
        )*}
    };
}

#[derive(std::marker::ConstParamTy, PartialEq, Eq)]
#[litenum::to]
pub enum Tag {
    ANY,

    a,
    abbr,
    address,
    area,
    article,
    aside,
    audio,
    b,
    base,
    bdi,
    blockquote,
    body,
    br,
    button,
    canvas,
    caption,
    circle,
    cite,
    code,
    col,
    colgroup,
    data,
    datalist,
    dd,
    del,
    details,
    dfn,
    dialog,
    div,
    dl,
    dt,
    em,
    embed,
    fencedframe,
    fieldset,
    figcaption,
    figure,
    footer,
    form,
    h1,
    h2,
    h3,
    h4,
    h5,
    h6,
    head,
    header,
    hgroup,
    hr,
    html,
    i,
    iframe,
    img,
    input,
    ins,
    kbd,
    label,
    legend,
    li,
    link,
    main,
    map,
    mark,
    menu,
    meta,
    meter,
    nav,
    noscript,
    object,
    ol,
    optgroup,
    option,
    output,
    p,
    path,
    picture,
    portal,
    pre,
    progress,
    q,
    rp,
    rt,
    ruby,
    s,
    samp,
    script,
    search,
    section,
    select,
    slot,
    small,
    source,
    span,
    strong,
    style,
    sub,
    summary,
    sup,
    svg,
    table,
    tbody,
    td,
    template,
    textarea,
    tfoot,
    th,
    thead,
    time,
    title,
    tr,
    track,
    u,
    ul,
    var,
    video,
    wbr,
}

impl<const TAG: Tag> Element<TAG> {
    pub(crate) const fn new() -> Self {
        Element {
            tag:           TAG.lit(),
            attributes:    None,
            eventhandlers: None,
            children:      Vec::new()
        }
    }

    pub(crate) fn into_node(self) -> Node {
        let this: Element<{Tag::ANY}> = unsafe {std::mem::transmute(self)};
        Node::Element(this)
    }
}

impl Node {
    pub fn render_to(self, container: &web_sys::Node) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        match self {
            Node::Text(text) => {
                let node = document.create_text_node(&text);
                container.append_child(&node)?;
            }
            Node::Element(Element { tag, attributes, eventhandlers, children }) => {
                let node = document.create_element(tag)?; {
                    if let Some(attributes) = attributes {                        
                        for (name, value) in *attributes {
                            node.set_attribute(name, &value)?;
                        }
                    }
                    if let Some(eventhandlers) = eventhandlers {                        
                        for (event, handler) in *eventhandlers {
                            let handler = handler.into_wasm_closure();
                            node.add_event_listener_with_callback(event, handler.as_ref().unchecked_ref())?;
                            handler.forget();
                        }
                    }
                    for child in children {
                        child.render_to(container)?;
                    }
                }
                container.append_child(&node)?;
            }
        }

        Ok(())
    }
}
