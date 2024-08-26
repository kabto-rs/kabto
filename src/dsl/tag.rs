macro_rules! native_tags {
    ($($name:ident)*) => {
        pub mod tag {
            use crate::vdom::{Element, Tag};
            $(
                #[allow(non_upper_case_globals)]
                pub const $name: Element<{Tag::$name}> = Element::new();
            )*
        }
    };
} native_tags! {
    /* main root */
    html

    /* document metadata */
    head
    link
    meta
    style
    title

    /* sectioning root */
    body

    /* content sectioning */
    article
    aside
    footer
    header
    h1
    h2
    h3
    h4
    h5
    h6
    main
    nav
    section

    /* text content */
    blockquote
    div
    li
    menu
    ol
    p
    pre
    ul

    /* inline text semantics */
    a
    code
    span
    strong

    /* image and multimedia */
    audio
    img
    video

    /* embedded content */
    iframe

    /* svg */
    svg
    path
    circle

    /* scripting */
    canvas
    script

    /* table content */
    caption
    col
    colgroup
    table
    tbody
    td
    tfoot
    th
    thread
    tr

    /* forms */
    button
    form
    input
    label
    textarea
}
