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


pub trait IntoNode {
    fn into_node(self) -> Node;
}

pub trait NodeCollection: std::marker::Tuple {
    const N: usize;
    fn collect(self) -> [Node; Self::N];
}


//////////////////////////////////////////////


impl IntoNode for Node {
    fn into_node(self) -> Node {self}
}

impl<Children: NodeCollection> FnOnce<Children> for Node
where [(); Children::N]:
{
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        let Node::Element(mut element) = self else {unreachable!()};
        element.children.extend(children.collect());
        Node::Element(element)
    }
}


//////////////////////////////////////////////


pub struct div;
