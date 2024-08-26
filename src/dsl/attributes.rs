use crate::vdom::{Text, Element, Tag};


macro_rules! boolean {
    ($($name:ident)*) => {
        impl<const T: Tag> Element<T> {$(
            pub fn $name(mut self) -> Self {
                if self.attributes.is_none() {
                    self.attributes = Some(Default::default())
                }

                unsafe {self.attributes.as_mut().unwrap_unchecked()}
                    .insert(stringify!($name), "".into());

                self
            }
        )*}
    };
} boolean! {
    autofocus
    contenteditable
    hidden
    insert
    popover
}

macro_rules! enumerated {
    ($($name:ident: $( $method:ident ($value:literal) )|* ),*) => {
        impl<const T: Tag> Element<T> {$(
            $(
                pub fn $method(mut self) -> Self {
                    if self.attributes.is_none() {
                        self.attributes = Some(Default::default())
                    }

                    unsafe {self.attributes.as_mut().unwrap_unchecked()}
                        .insert(stringify!($name), $value.into());

                    self
                }
            )*
        )*}
    };
} enumerated! {
    autocapitalize:  autocapitalize_off("off") | autocapitalize_on("on") | autocapitalize_words("words") | autocapitalize_charactors("charactors"),
    contenteditable: contenteditable_false("false") | contenteditable_plaintext("plaintext-only"),
    dir:             dir_ltr("ltr") | dir_rtl("rtl") | dir_auto("auto"),
    draggable:       draggable_true("true") | draggable_false("false"),
    hidden:          hidden_untilfound("until-found"),
    inputmode:       inputmode_none("none") | inputmode_text("text") | inputmode_decimal("decimal") | inputmode_numeric("numeric") | inputmode_tel("tel") | inputmode_search("search") | inputmode_email("email") | inputmode("url"),
    spellcheck:      spellcheck_true("true") | spellcheck_false("false"),
    translate:       translate_yes("yes") | translate_no("no")
}

macro_rules! other {
    ($($name:ident)*) => {
        impl<const T: Tag> Element<T> {$(
            pub fn $name(mut self, value: impl Into<Text>) -> Self {
                if self.attributes.is_none() {
                    self.attributes = Some(Default::default())
                }

                unsafe {self.attributes.as_mut().unwrap_unchecked()}
                    .insert(stringify!($name), value.into());

                self
            }
        )*}
    };
} other! {
    accesskey
    class
    enterkeyhint
    exportparts
    id
    is
    itemid
    itemprop
    itemref
    itemscope
    itemtype
    lang
    nonce
    part
    role
    slot
    style
    tabindex
    title
}
