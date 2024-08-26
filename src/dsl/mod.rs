mod nodes;
mod attributes;
mod eventhandlers;

use crate::vdom::{Element, Tag};


macro_rules! tag {
    ($($name:ident),* $(,)?) => {
        #[cfg(debug_assertions)]
        fn __assert_exaustive__(tag: Tag) {
            match tag {
                $( Tag::$name | )* Tag::ANY => ()
            }
        }

        $(
            #[allow(non_upper_case_globals)]
            pub const $name: Element<{Tag::$name}> = Element::new();
        )*
    };
} tag! {
    a,
    abbr,
    address,
    area,
    article,
    aside,
    audio,
    b,
    base,
    bdi,
    blockquote,
    body,
    br,
    button,
    canvas,
    caption,
    circle,
    cite,
    code,
    col,
    colgroup,
    data,
    datalist,
    dd,
    del,
    details,
    dfn,
    dialog,
    div,
    dl,
    dt,
    em,
    embed,
    fencedframe,
    dieldset,
    figcaption,
    figure,
    footer,
    form,
    h1,
    h2,
    h3,
    h4,
    h5,
    h6,
    head,
    header,
    hgroup,
    hr,
    html,
    i,
    iframe,
    img,
    input,
    ins,
    kbd,
    label,
    legend,
    li,
    link,
    main,
    map,
    mark,
    menu,
    meta,
    meter,
    nav,
    noscript,
    object,
    ol,
    optgroup,
    option,
    output,
    p,
    path,
    picture,
    portal,
    pre,
    progress,
    q,
    rp,
    rt,
    ruby,
    s,
    samp,
    script,
    search,
    section,
    select,
    slot,
    small,
    source,
    span,
    strong,
    style,
    sub,
    summary,
    sup,
    svg,
    table,
    tbody,
    td,
    template,
    textarea,
    tfoot,
    th,
    thead,
    time,
    title,
    tr,
    track,
    u,
    ul,
    var,
    video,
    wbr,
}
