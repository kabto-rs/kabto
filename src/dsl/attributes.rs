use crate::vdom::{Text, Element, Tag};


macro_rules! keyvalue {
    (
        global {
            $( $name:ident $(for $attr_name:ident)? ),* $(,)?
        }
        $($tag:ident {
            $( $name2:ident $(for $attr_name2:literal)? ),* $(,)?
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
    (@ $name:ident for $attr_name:literal) => {
        pub fn $name(mut self, value: impl Into<Text>) -> Self {
            if self.attributes.is_none() {
                self.attributes = Some(Default::default())
            }
            unsafe {self.attributes.as_mut().unwrap_unchecked()}
                .insert($attr_name, value.into());
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
        download_filename for "download",
        href,
        hreflang,
        ping,
        rel,
        type_ for "type",
    }
    area {
        alt,
        coords,
        download_filename for "download",
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
    button {
        form,
        formaction,
        formtarget,
        name,
        popovertarget,
        value
    }
    canvas {
        height,
        width
    }
    col {
        span
    }
    colgroup {
        span
    }
    data {
        value
    }
    del {
        cite,
        datetime
    }
    embed {
        height,
        src,
        type_ for "type",
        width
    }
    fencedframe {
        allow,
        height,
        width
    }
    fieldset {
        form,
        name
    }
    form {
        accept_charset for "accept-charset",
        name,
        rel,
        action
    }
    iframe {
        allow,
        height,
        loading,
        name,
        sandbox,
        src,
        srcdoc,
        width
    }
    img {
        alt,
        elementtiming,
        height,
        sizes,
        src,
        srcset,
        width,
        usemap,
    }
    input {
        accept,
        alt,
        autocomplete,
        capture,
        dirname,
        form,
        formaction,
        height,
        list,
        max,
        maxlength,
        min,
        minlength,
        name,
        pattern,
        placeholder,
        popovertarget,
        size,
        src,
        step,
        value,
        width,
    }
}


macro_rules! boolean {
    (
        global {
            $( $name:ident $(for $attr_name:literal)? ),* $(,)?
        }
        $($tag:ident {
            $( $name2:ident $(for $attr_name2:literal)? ),* $(,)?
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
    (@ $name:ident for $attr_name:literal) => {
        pub fn $name(mut self) -> Self {
            if self.attributes.is_none() {
                self.attributes = Some(Default::default())
            }
            unsafe {self.attributes.as_mut().unwrap_unchecked()}
                .insert($attr_name, "".into());
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
    }
    area {
        download,
    }
    audio {
        autoplay,
        controls,
        loops for "loop",
        muted
    }
    button {
        disabled,
        formnovalidate,
    }
    details {
        open
    }
    dialog {
        open
    }
    fieldset {
        disabled
    }
    form {
        novalidate
    }
    img {
        ismap
    }
    input {
        checked,
        disabled,
        formnovalidate,
        multiple,
        readonly,
        required,
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
    button {
        formenctype [
            formenctype_urlencoded("application/x-www-form-urlencoded")
            formenctype_multipart("multipart/form-data")
        ]
        formmethod [
            formmethod_post("post")
            formmethod_get("get")
            formmethod_dialog("dialog")
        ]
        formtarget [
            formtarget_self("_self")
            formtarget_blank("_blank")
            formtarget_parent("_parent")
            formtarget_top("_top")
        ]
        popovertargetaction [
            popovertargetaction_hide("hide")
            popovertargetaction_show("show")
            popovertargetaction_toggle("toggle")
        ]
        type [
            type_submit("submit")
            type_reset("reset")
            type_button("button")
        ]
    }
    form {
        autocomplete [
            autocomplete_on("on")
            autocomplete_off("off")
        ]
        enctype [
            enctype_urlencoded("application/x-www-form-urlencoded")
            enctype_multipart("multipart/form-data")
        ]
        method [
            method_post("post")
            method_get("get")
            method_dialog("dialog")
        ]
        target [
            target_self("_self")
            target_blank("_blank")
            target_parent("_parent")
            target_top("_top")
        ]
    }
    iframe {
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
    }
    img {
        crossorigin [
            crossorigin_anonymous("anonymous")
            crossorigin_use_credentials("use-credentials")
        ]
        decoding [
            decoding_sync("sync")
            decoding_async("async")
            decoding_auto("auto")
        ]
        loading [
            loading_eager("eager")
            loading_lazy("lazy")
        ]
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
    }
    input {
        formenctype [
            formenctype_urlencoded("application/x-www-form-urlencoded")
            formenctype_multipart("multipart/form-data")
        ]
        formmethod [
            formmethod_post("post")
            formmethod_get("get")
            formmethod_dialog("dialog")
        ]
        formtarget [
            formtarget_self("_self")
            formtarget_blank("_blank")
            formtarget_parent("_parent")
            formtarget_top("_top")
        ]
        popovertargetaction [
            popovertargetaction_hide("hide")
            popovertargetaction_show("show")
            popovertargetaction_toggle("toggle")
        ]
        type [
            type_button("button")
            type_checkbox("checkbox")
            type_color("color")
            type_date("date")
            type_datetime_local("datetime-load")
            type_email("email")
            type_file("file")
            type_hidden("hidden")
            type_image("image")
            type_month("month")
            type_number("number")
            type_password("password")
            type_radio("radio")
            type_range("range")
            type_search("search")
            type_submit("submit")
            type_tel("tel")
            type_text("text")
            type_time("time")
            type_url("url")
            type_week("week")
        ]
    }
}
