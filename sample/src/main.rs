use kabto::{tag, document};

fn app() -> impl kabto::Component {
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
            )
        )
    )
}

fn main() {
    let root = document().get_element_by_id("root").unwrap();
    kabto::render(app(), root).unwrap()
}

