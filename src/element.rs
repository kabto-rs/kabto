use crate::util::IndexMap;


pub struct Element {
    pub tag:      Tag,
    pub props:    Props,
    pub children: Vec<Element>,
}
const _: () = {
    impl Element {
        pub fn text(text: impl Into<String>) -> Self {
            Self {
                tag:      Tag::TEXT_ELEMENT,
                props:    Props::from([(Prop::nodeValue, text.into())]),
                children: vec![],
            }
        }
    }
};

#[litenum::to]
pub enum Tag {
    TEXT_ELEMENT,
    div,
    h1,
    h2,
    p,
    a
}

pub struct Props(
    IndexMap<Prop, String>
);
#[litenum::to]
#[derive(Clone, Copy)]
pub enum Prop {
    nodeValue,
    href,
}
impl crate::util::Index for Prop {
    const MAX: u8 = 2;
    fn n(self) -> u8 {self as _}
}
const _: () = {
    impl Props {
        pub fn empty() -> Self {
            Self(IndexMap::with_capacity(0))
        }

        pub fn new() -> Self {
            Self(IndexMap::with_capacity(2))
        }

        pub fn get(&self, key: Prop) -> Option<&str> {
            self.0.get(key).map(String::as_str)
        }

        pub fn set(&mut self, key: Prop, value: impl Into<String>) {
            self.0.set(key, value.into())
        }

        pub fn iter(&self) -> impl Iterator<Item = &(Prop, String)> {
            self.0.iter()
        }
    }

    impl<const N: usize, V: Into<String>> From<[(Prop, V); N]> for Props {
        fn from(vec: [(Prop, V); N]) -> Self {
            let mut map = IndexMap::with_capacity(vec.len());
            for (k, v) in vec {
                map.set(k, v.into())
            }
            Self(map)
        }
    }
};

impl Element {
    pub fn JS_render_to(self, container: &web_sys::Node) -> Result<(), web_sys::wasm_bindgen::JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        match self.tag {
            Tag::TEXT_ELEMENT => {
                let text_node = document.create_text_node(self.props.get(Prop::nodeValue).unwrap_or(""));
                container.append_child(&text_node)?;

                Ok(())
            }

            tag => {
                let dom = document.create_element(tag.lit())?;
                for (prop, v) in self.props.iter() {
                    dom.set_attribute(prop.lit(), v)?;
                }

                for child in self.children {
                    child.JS_render_to(&dom)?;
                }

                container.append_child(&dom)?;

                Ok(())
            }
        }
    }
}
