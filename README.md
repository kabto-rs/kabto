```rs
use kabto::{Component, HTML, tag::*};

struct Meta {titile: String, description: String}
impl Component for Meta {
    fn render(self) -> impl HTML {
        (
            meta.charset("utf-8"),
            meta.name("viewport").content("width=device-width,initial-scale=1"),
            meta.http_equiv("Content-Type").content("text/html; charset=utf-8"),
            meta.http_equiv("Content-Language").content("ja"),
            meta.name("description").content(description),
            link.rel("icon").href("/blog/favicon.ico").sizes("any"),
            link.rel("icon").href("/blog/icon.svg").icon_type("image/svg+xml"),
        )
    }
}

struct App {
    meta_title:  String,
    description: String,
    title:       Option<String>,
    volume:      usize,
    categories:  Vec<String>,
    written_at:  Option<Date>,
} impl Component for App {
    fn render(self) -> impl HTML {
        let Self { meta_title, description, title, volume, categories, written_at } = self;
        let is_series_article = is_series_title(&title);

        (
            html.lang("en")(
                head.prefix("og: http://ogp.me/ns#"),
                Meta {
                    title: meta_title,
                    description: description.clone()
                },
                body(
                    header(
                        h1("kanarus")
                    ),
                    div.id("content")(
                        article(
                            h2(title),
                            div.id("metadata")(
                                div.id("tags")(categories.iter().map(|c|
                                    span.class("tag")(c)
                                )),
                                div.id("dates")(written_at.map(|wa| (
                                    svg.class("icon").view_box("0 0 32 32")(
                                        path.d("...")
                                    ),
                                    span.style("margin-left: 0.25em;")(wa)
                                )))
                            ),
                            p(is_series_article.then(
                                description
                            )),
                        )
                    ),
                    Sidebar {},
                )
            )
        )
    }
}
```
