use kabto::{document, JSResult, JsCast, tag, console_log, event::Event};
// 
// struct Props {
//     value: String,
// }
// 
// fn app(Props { value }: Props) -> impl kabto::Component {
//     let update_value = |e: Event| {
//         console_log!("`update_value` called with {e:?}");
//         render_with(Props {
//             value: e.target().unwrap()
//                 .unchecked_into::<web_sys::HtmlInputElement>()
//                 .value()
//         })
//     };
// 
//     (
//         tag::h1("Kabto app"),
//         tag::div.id("app")(
//             tag::p(
//                 "GitHub repo (currently private): ",
//                 tag::a
//                     .href("https://github.com/ohkami-rs/kabto")
//                     .target_blank()
//                     (
//                         "https://github.com/ohkami-rs/kabto"
//                     )
//             ),
//             tag::div(
//                 tag::h2(format!("This is {value}")),
//                 tag::input
//                     .value(value.clone())
//                     .on_input(update_value),
//             )
//         )
//     )
// }
// 
// fn render_with(props: Props) -> JSResult<()> {
//     let root = document().get_element_by_id("root").unwrap();
//     kabto::render(app(props), root)
// }

fn main() -> JSResult<()> {
    // render_with(Props {
    //     value: String::from("Hello, kabto!")
    // })

    use web_sys::{Element, Node, wasm_bindgen::JsCast};

    let root: Element = document().get_element_by_id("root").unwrap();
    console_log!("root = {root:?}");
    root.set_inner_html(r#"<p>
        <a href="https://x.com/kanarus" target="_blank">
            X profile
        </a>
    </p>"#);
    //root.set_attribute(name, value)

    let node: Node = root.unchecked_into();
    console_log!("node = {node:?}");
    node.append_child({
        let child = document().create_element("p")?;
        child.set_inner_html(r#"
            <a href="https://github.com/kana-rus" target="_blank">
                GitHub profile
            </a>
        "#);
        child
    }.as_ref())?;
    //node.set

    let element: Element = node.unchecked_into();
    console_log!("element = {element:?}");
    element.append_child({
        let child = document().create_element("p")?;
        child.set_inner_html(r#"
            <a href="https://github.com/ohkami-rs" target="_blank">
                ohkami-rs home
            </a>
        "#);
        child
    }.as_ref())?;

    let text: web_sys::Text = document().create_text_node("This is text!");
    console_log!("text = {text:?}");
    element.append_child({
        let child = document().create_element("p")?;
        child.append_child(&text)?;
        child
    }.as_ref())?;

    let text: &web_sys::Element = text.unchecked_ref();
    console_log!("text as Element = {text:?}");
    console_log!("text's child_element_count = {}", text.child_element_count());

    text.remove();

    Ok(())
}
