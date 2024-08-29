mod eventhandler;
pub(crate) use eventhandler::{eventHandler, EventHandler};

use crate::util::Text;
use std::{collections::HashMap, marker::PhantomData};


#[derive(Clone)]
pub enum Node {
    Text(Text),
    Element(Element<()>),
}

#[derive(PartialEq)]
pub(crate) enum NodeKind {
    Text,
    Element(&'static str)
}

#[derive(Clone)]
pub struct Element<T: Tag> {t: PhantomData<fn()->T>,
    pub(crate) tag:   &'static str,
    pub(crate) props: Props,
}

#[derive(Clone)]
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

impl Props {
    pub(crate) const fn new() -> Self {
        Props {
            attributes:    None,
            eventhandlers: None,
            children:      Vec::new()
        }
    }
}

impl<T: Tag> Element<T> {
    pub(crate) const fn new() -> Self {
        Element {t: PhantomData,
            tag:   T::NAME,
            props: Props::new()
        }
    }

    pub(crate) fn with(props: Props) -> Self {
        Element {t: PhantomData,
            tag:   T::NAME,
            props
        }
    }

    pub(crate) fn into_node(self) -> Node {
        Node::Element(unsafe {std::mem::transmute(self)})
    }
}

impl Node {
    pub(crate) fn kind(&self) -> NodeKind {
        match self {
            Self::Element(e) => NodeKind::Element(e.tag),
            Self::Text(_)    => NodeKind::Text
        }
    }

    pub(crate) fn props(&self) -> Option<&Props> {
        match self {
            Self::Element(e) => Some(&e.props),
            Self::Text(_)    => None
        }
    }
    pub(crate) fn props_mut(&mut self) -> Option<&mut Props> {
        match self {
            Self::Element(e) => Some(&mut e.props),
            Self::Text(_)    => None
        }
    }
}

#[cfg(feature="DEBUG")]
const _: () = {
    impl std::fmt::Debug for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Text(text) => f.debug_tuple("TextElement")
                    .field(text)
                    .finish(),
                Self::Element(e) => {
                    let mut d = f.debug_struct(&format!("Element<{}>", e.tag));
                    if let Some(attrs) = &e.props.attributes {
                        d.field("attrs", attrs);
                    }
                    d.finish()
                }
            }
        }
    }
};
