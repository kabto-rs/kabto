use kabto::{document, JSResult, JsCast, tag, console_log, event::Event};

struct Props {
    value: String,
}

fn app(Props { value }: Props) -> impl kabto::Component {
    let update_value = |e: Event| {
        console_log!("`update_value` called with {e:?}");
        render_with(Props {
            value: e.target().unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>()
                .value()
        })
    };

    (
        tag::h1("Kabto app"),
        tag::div.id("app")(
            tag::p(
                "GitHub repo (currently private): ",
                tag::a
                    .href("https://github.com/ohkami-rs/kabto")
                    .target_blank()
                    (
                        "https://github.com/ohkami-rs/kabto"
                    )
            ),
            tag::div(
                tag::h2(format!("This is {value}")),
                tag::input
                    .value(value.clone())
                    .on_input(update_value),
            )
        )
    )
}

fn render_with(props: Props) -> JSResult<()> {
    let root = document().get_element_by_id("root").unwrap();
    kabto::render(app(props), root)
}

fn main() -> JSResult<()> {
    render_with(Props {
        value: String::from("Hello, kabto!")
    })
}
