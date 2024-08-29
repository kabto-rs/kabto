mod eventhandler;
pub(crate) use eventhandler::EventHandler;

mod text;
pub(crate) use text::VText;

use crate::util::RcX;
use std::{collections::HashMap, marker::PhantomData};
use web_sys::js_sys::Function;


#[derive(Clone)]
pub(crate) struct VDOM(VNode);

#[derive(Clone)]
pub struct VNode(RcX<VElement<()>>);

pub struct VElement<T: Tag> {t: PhantomData<fn()->T>,
    kind:     Kind,
    props:    Option<Props>,
    value:    Option<VText>,
    children: Option<Vec<VNode>>,
}

pub(crate) enum Kind {
    Tag(&'static str),
    Text
}

pub(crate) struct Props {
    attributes:    Box<HashMap<&'static str, VText>>,
    eventhandlers: Box<HashMap<&'static str, Function>>,
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

impl Props {
    pub(crate) fn new() -> Self {
        Props {
            attributes:    Box::default(),
            eventhandlers: Box::default(),
        }
    }
}

impl VElement<()> {
    pub(crate) fn new_text(text: impl Into<VText>) -> Self {
        VElement {t: PhantomData,
            kind:     Kind::Text,
            props:    None,
            value:    Some(text.into()),
            children: None
        }
    }
}

impl<T: Tag> VElement<T> {
    pub(crate) const fn new_tag() -> Self {
        VElement {t: PhantomData,
            kind:     Kind::Tag(T::NAME),
            props:    None,
            value:    None,
            children: None
        }
    }

    pub(crate) fn into_node(self) -> VNode {
        VNode(RcX::new(unsafe {std::mem::transmute(self)}))
    }

    pub(crate) fn kind(&self) -> &Kind {
        &self.kind
    }

    pub(crate) fn as_text(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub(crate) fn children(&self) -> Option<&Vec<VNode>> {
        self.children.as_ref()
    }
    pub(crate) fn children_mut(&mut self) -> &mut Vec<VNode> {
        if self.children.is_none() {
            self.children = Some(Vec::new())
        }
        unsafe {self.children.as_mut().unwrap_unchecked()}
    }

    pub(crate) fn attributes(&self) -> Option<&HashMap<&'static str, VText>> {
        self.props.as_ref().map(|p| &*p.attributes)
    }
    pub(crate) fn attributes_mut(&mut self) -> &mut HashMap<&'static str, VText> {
        if self.props.is_none() {
            self.props = Some(Props::new())
        }
        unsafe {&mut self.props.as_mut().unwrap_unchecked().attributes}
    }

    pub(crate) fn eventhandlers(&self) -> Option<&HashMap<&'static str, Function>> {
        self.props.as_ref().map(|p| &*p.eventhandlers)
    }
    pub(crate) fn eventhandlers_mut(&mut self) -> &mut HashMap<&'static str, Function> {
        if self.props.is_none() {
            self.props = Some(Props::new())
        }
        unsafe {&mut self.props.as_mut().unwrap_unchecked().eventhandlers}
    }
}
