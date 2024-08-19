// use crate::element;
// use internal::Tag;
// 
// 
// pub type div = Tag<{element::Tag::div}>;
// pub type h1  = Tag<{element::Tag::h1}>;
// pub type h2  = Tag<{element::Tag::h2}>;
// pub type p   = Tag<{element::Tag::p}>;
// pub type a   = Tag<{element::Tag::a}>;
// 
// mod internal {
//     pub struct Tag<const T: crate::element::Tag>;
// 
//     pub struct Element(crate::element::Element);
// 
//     macro_rules! attributes {
//         ($($name:ident)*) => {
//             impl<const T: crate::element::Tag> Tag<T> {$(
//                 pub fn $name(mut self, value: ) -> Element {
// 
//                 }
//             )*}
//             impl Element {$(
//                 pub fn $name(mut self, value: ) -> Element {
// 
//                 }
//             )*}
//         };
//     } attributes! {
// 
//     }
// }
// 


use crate::vdom::{Node, Element};
use crate::util::Text;


pub trait IntoNode {
    fn into_node(self) -> Option<Node>;
}

pub trait NodeCollection: std::marker::Tuple {
    const N: usize;
    fn collect(self) -> [Option<Node>; Self::N];
}


//////////////////////////////////////////////


impl IntoNode for Node {
    fn into_node(self) -> Option<Node> {
        Some(self)
    }
}
impl<IN: IntoNode> IntoNode for Option<IN> {
    fn into_node(self) -> Option<Node> {
        self?.into_node()
    }
}
macro_rules! TextNode {
    ($($text:ty),*) => {$(
        impl IntoNode for $text {
            fn into_node(self) -> Option<Node> {
                Some(Node::Text(Text::from(self).into()))
            }
        }
    )*};
} TextNode! { &'static str, String, u8, usize, i32 }

impl<Children: NodeCollection> FnOnce<Children> for Node
where [(); Children::N]:
{
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        let Node::Element(mut element) = self else {unreachable!()};
        element.children.extend(children.collect().into_iter().filter_map(|it|it));
        Node::Element(element)
    }
}

macro_rules! NodeCollection {
    ($($node:ident),*; $n:literal) => {
        impl<$($node: IntoNode),*> NodeCollection for ($($node,)*) {
            const N: usize = $n;
            fn collect(self) -> [Option<Node>; Self::N] {
                let ($($node,)*) = self;
                [$($node.into_node(),)*]
            }
        }
    };
}
NodeCollection! { ; 0}
NodeCollection! { N1; 1 }
NodeCollection! { N1, N2; 2 }
NodeCollection! { N1, N2, N3; 3 }
NodeCollection! { N1, N2, N3, N4; 4 }
NodeCollection! { N1, N2, N3, N4, N5; 5 }
NodeCollection! { N1, N2, N3, N4, N5, N6; 6 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7; 7 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8; 8 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9; 9 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10; 10 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11; 11 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12; 12 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13; 13 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14; 14 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15; 15 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16; 16 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17; 17 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18; 18 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19; 19 }
NodeCollection! { N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20; 20 }


//////////////////////////////////////////////


macro_rules! native_tags {
    ($($name:ident)*) => {$(
        #[allow(non_upper_case_globals)]
        pub const $name: Node = Node::new_element(stringify!($name));
    )*};
} native_tags! {
    /* main root */
    html

    /* document metadata */
    head
    link
    meta
    style
    title

    /* sectioning root */
    body

    /* content sectioning */
    article
    aside
    footer
    header
    h1
    h2
    h3
    h4
    h5
    h6
    main
    nav
    section

    /* text content */
    blockquote
    div
    li
    menu
    ol
    p
    pre
    ul

    /* inline text semantics */
    a
    code
    span
    strong

    /* image and multimedia */
    audio
    img
    video

    /* embedded content */
    iframe

    /* svg */
    svg
    path
    circle

    /* scripting */
    canvas
    script

    /* table content */
    caption
    col
    colgroup
    table
    tbody
    td
    tfoot
    th
    thread
    tr

    /* forms */
    button
    form
    input
    label
    textarea
}
