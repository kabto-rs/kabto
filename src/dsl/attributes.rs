use crate::vdom::{Text, Element, Tag};


macro_rules! keyvalue {
    (
        global {
            $( $name:ident $(for $attr_name:ident)? ),* $(,)?
        }
        $($tag:ident {
            $( $name2:ident $(for $attr_name2:ident)? ),* $(,)?
        })*
    ) => {
        impl<const T: Tag> Element<T> {$(
            keyvalue! {@ $name $(for $attr_name)?}
        )*}
        $(impl Element<{Tag::$tag}> {$(
            keyvalue! {@ $name2 $(for $attr_name2)?}
        )*})*
    };

    (@ $name:ident) => {
        pub fn $name(mut self, value: impl Into<Text>) -> Self {
            if self.attributes.is_none() {
                self.attributes = Some(Default::default())
            }
            unsafe {self.attributes.as_mut().unwrap_unchecked()}
                .insert(stringify!($name), value.into());
            self
        }
    };
    (@ $name:ident for $attr_name:ident) => {
        pub fn $name(mut self, value: impl Into<Text>) -> Self {
            if self.attributes.is_none() {
                self.attributes = Some(Default::default())
            }
            unsafe {self.attributes.as_mut().unwrap_unchecked()}
                .insert(stringify!($attr_name), value.into());
            self
        }
    };
} keyvalue! {
    global {
        accesskey,
        class,
        enterkeyhint,
        exportparts,
        id,
        is,
        itemid,
        itemprop,
        itemref,
        itemscope,
        itemtype,
        lang,
        nonce,
        part,
        role,
        slot,
        style,
        tabindex,
        title,
    }
    a {
        download_filename for download,
        href,
        hreflang,
        ping,
        rel,
    }
    area {
        alt,
        coords,
        download_filename for download,
        href,
        hreflang,
        ping,
        rel,
        shape,
        src
    }
    base {
        href
    }
    blockquote {
        cite
    }
}

macro_rules! boolean {
    (
        global {
            $( $name:ident $(for $attr_name:ident)? ),* $(,)?
        }
        $($tag:ident {
            $( $name2:ident $(for $attr_name2:ident)? ),* $(,)?
        })*
    ) => {
        impl<const T: Tag> Element<T> {$(
            boolean! {@ $name $(for $attr_name)?}
        )*}
        $(impl Element<{Tag::$tag}> {$(
            boolean! {@ $name2 $(for $attr_name2)?}
        )*})*
    };

    (@ $name:ident) => {
        pub fn $name(mut self) -> Self {
            if self.attributes.is_none() {
                self.attributes = Some(Default::default())
            }
            unsafe {self.attributes.as_mut().unwrap_unchecked()}
                .insert(stringify!($name), "".into());
            self
        }
    };
    (@ $name:ident for $attr_name:ident) => {
        pub fn $name(mut self) -> Self {
            if self.attributes.is_none() {
                self.attributes = Some(Default::default())
            }
            unsafe {self.attributes.as_mut().unwrap_unchecked()}
                .insert(stringify!($attr_name), "".into());
            self
        }
    };
} boolean! {
    global {
        autofocus,
        contenteditable,
        hidden,
        insert,
        popover,
    }
    a {
        download,
        type_ for type,
    }
    area {
        download,
    }
    audio {
        autoplay,
        controls,
        loops for loop,
        muted
    }
}

macro_rules! enumerated {
    (
        global {
            $($name:ident [$( $method:ident ($value:literal) )*] )*
        }
        $($tag:ident {
            $($name2:ident [$( $method2:ident ($value2:literal) )*] )*
        })*
    ) => {
        impl<const T: Tag> Element<T> {$(
            $(pub fn $method(mut self) -> Self {
                    if self.attributes.is_none() {
                        self.attributes = Some(Default::default())
                    }
                    unsafe {self.attributes.as_mut().unwrap_unchecked()}
                        .insert(stringify!($name), $value.into());
                    self
            })*
        )*}
        $(impl Element<{Tag::$tag}> {$(
            $(pub fn $method2(mut self) -> Self {
                if self.attributes.is_none() {
                    self.attributes = Some(Default::default())
                }
                unsafe {self.attributes.as_mut().unwrap_unchecked()}
                    .insert(stringify!($name2), $value2.into());
                self
        })*)*})*
    };
} enumerated! {
    global {
        autocapitalize [
            autocapitalize_off("off")
            autocapitalize_on("on")
            autocapitalize_words("words")
            autocapitalize_charactors("charactors")
        ]
        contenteditable [
            contenteditable_false("false")
            contenteditable_plaintext_only("plaintext-only")
        ]
        dir [
            dir_ltr("ltr")
            dir_rtl("rtl")
            dir_auto("auto")
        ]
        draggable [
            draggable_true("true")
            draggable_false("false")
        ]
        hidden [
            hidden_until_found("until-found")
        ]
        inputmode [
            inputmode_none("none")
            inputmode_text("text")
            inputmode_decimal("decimal")
            inputmode_numeric("numeric")
            inputmode_tel("tel")
            inputmode_search("search")
            inputmode_email("email")
            inputmode("url")
        ]
        spellcheck [
            spellcheck_true("true")
            spellcheck_false("false")
        ]
        translate [
            translate_yes("yes")
            translate_no("no")
        ]
    }
    a {
        referrerpolicy [
            referrerpolicy_no_referrer("no-referrer")
            referrerpolicy_no_referrer_when_downgrade("no-referrer-when-downgrade")
            referrerpolicy_origin("origin")
            referrerpolicy_origin_when_crossorigin("origin-when-cross-origin")
            referrerpolicy_same_origin("same-origin")
            referrerpolicy_strict_origin("strict-origin")
            referrerpolicy_strict_origin_when_cross_origin("strict-origin-when-cross-origin")
            referrerpolicy_unsafe_url("unsafe-url")
        ]
        target [
            target_self("_self")
            target_blank("_blank")
            target_parent("_parent")
            target_top("_top")
        ]
    }
    area {
        target [
            target_self("_self")
            target_blank("_blank")
            target_parent("_parent")
            target_top("_top")
        ]
    }
    audio {
        crossorigin [
            crossorigin_anonymous("anonymous")
            crossorigin_use_credentials("use-credentials")
        ]
        preload [
            preload_none("none")
            preload_metadata("metadata")
            preload_auto("auto")
        ]
    }
    base {
        target [
            target_self("_self")
            target_blank("_blank")
            target_parent("_parent")
            target_top("_top")
        ]
    }
}
