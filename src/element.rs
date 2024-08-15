pub struct Element {
    tag:   Tag,
    props: Props,
}

#[litenum::to]
enum Tag {
    TEXT_ELEMENT,
    div,
}

struct Props {
    value:    Option<String>,
    children: Vec<Element>,
}
