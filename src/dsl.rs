use crate::element;
use internal::Tag;


pub type div = Tag<{element::Tag::div}>;
pub type h1  = Tag<{element::Tag::h1}>;
pub type h2  = Tag<{element::Tag::h2}>;
pub type p   = Tag<{element::Tag::p}>;
pub type a   = Tag<{element::Tag::a}>;

mod internal {
    pub struct Tag<const T: crate::element::Tag>;

    pub struct Element(crate::element::Element);

    macro_rules! props {
        ($($name:ident)*) => {
            impl<const T: crate::element::Tag> Tag<T> {$(
                pub fn $name(mut self, value: ) -> Element {

                }
            )*}
            impl Element {$(
                pub fn $name(mut self, value: ) -> Element {

                }
            )*}
        };
    } props! {

    }
}
