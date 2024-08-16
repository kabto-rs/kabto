use kabto::element::*;

fn main() -> Result<(), web_sys::wasm_bindgen::JsValue> {
    let Some(root) = web_sys::window().unwrap().document().unwrap().get_element_by_id("root") else {
        return Ok(web_sys::console::log_1(&web_sys::Text::new_with_data("#root not found")?.into()));
    };

    let app = Element {
        tag:      Tag::div,
        props:    Props::new(),
        children: vec![
            Element {
                tag:      Tag::a,
                props:    Props::from([(Prop::href, "https://github.com")]),
                children: vec![
                    Element::text("link to GitHub")
                ],
            },
            Element::text("Hello, kabto!")
        ]
    };

    app.JS_render_to(&root)
}
