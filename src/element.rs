pub struct Element {
    tag:      Tag,
    props:    Props,
    children: Vec<Element>,
}

#[litenum::to]
enum Tag {
    TEXT_ELEMENT,
    div,
}

struct Props {
    nodeValue: Option<String>,
}

impl Element {
    fn JS_render_to(self, container: &web_sys::Node) -> Result<(), web_sys::wasm_bindgen::JsValue> {
        let document = web_sys::Document::new()?;

        match self.tag {
            Tag::TEXT_ELEMENT => {
                let text_node = document.create_text_node("");
                container.append_child(&text_node)?;

                Ok(())
            }
            
            tag => {
                let dom = document.create_element(tag.lit())?;

                for child in self.children {
                    child.JS_render_to(&dom)?;
                }
                container.append_child(&dom)?;

                Ok(())
            }
        }
    }
}
