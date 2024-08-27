use crate::vdom::{Element, Node, Tag, Text};


pub trait IntoNodes {
    fn into_nodes(self) -> Nodes;
}

pub trait NodeCollection: std::marker::Tuple {
    fn collect(self) -> impl Iterator<Item = Nodes>;
}

pub enum Nodes {
    None,
    Some(Node),
    Many(Vec<Node>)
}
impl Nodes {
    fn join_into(self, collection: &mut Vec<Node>) {
        match self {
            Self::None    => (),
            Self::Some(n) => collection.push(n),
            Self::Many(m) => collection.extend(m)
        }
    }
}

impl<T: Tag> IntoNodes for Element<T> {
    fn into_nodes(self) -> Nodes {
        Nodes::Some(self.into_node())
    }
}
impl IntoNodes for Node {
    fn into_nodes(self) -> Nodes {
        Nodes::Some(self)
    }
}
impl<IN: IntoNodes> IntoNodes for Option<IN> {
    fn into_nodes(self) -> Nodes {
        match self {
            None    => Nodes::None,
            Some(n) => n.into_nodes()
        }
    }
}
impl<IN: IntoNodes> IntoNodes for Vec<IN> {
    fn into_nodes(self) -> Nodes {
        Nodes::Many(self.into_iter()
            .map(IntoNodes::into_nodes)
            .fold(Vec::new(), |mut vec, nodes| {nodes.join_into(&mut vec); vec})
        )
    }
}
impl<NC: NodeCollection> IntoNodes for NC {
    fn into_nodes(self) -> Nodes {
        let mut collection = Vec::new();
        for nodes in self.collect() {
            nodes.join_into(&mut collection)
        }
        Nodes::Many(collection)
    }
}
macro_rules! TextNode {
    ($($text:ty),*) => {$(
        impl IntoNodes for $text {
            fn into_nodes(self) -> Nodes {
                Nodes::Some(Node::Text(Text::from(self).into()))
            }
        }
    )*};
} TextNode! { &'static str, String, u8, usize, i32 }

impl<Children: NodeCollection> FnOnce<Children> for Node {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        let Node::Element(mut element) = self else {unreachable!()};
        for nodes in children.collect() {
            nodes.join_into(&mut element.props.children);
        }
        Node::Element(element)
    }
}
impl<T: Tag, Children: NodeCollection> FnOnce<Children> for Element<T> {
    type Output = Node;
    extern "rust-call" fn call_once(mut self, children: Children) -> Self::Output {
        for nodes in children.collect() {
            nodes.join_into(&mut self.props.children);
        }
        self.into_node()
    }
}

macro_rules! NodeCollection {
    ($($node:ident),*; $n:literal) => {
        impl<$($node: IntoNodes),*> NodeCollection for ($($node,)*) {
            fn collect(self) -> impl Iterator<Item = Nodes> {
                let ($($node,)*) = self;
                [$($node.into_nodes(),)*].into_iter()
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
