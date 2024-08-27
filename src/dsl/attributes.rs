use crate::vdom::{Element, Tag, tag};
use crate::util::Text;


macro_rules! keyvalue {
    (
        global {
            $( $name:ident $(for $attr_name:ident)? ),* $(,)?
        }
        $($tag:ident {
            $( $name2:ident $(for $attr_name2:literal)? ),* $(,)?
        })*
    ) => {
        impl<T: Tag> Element<T> {$(
            keyvalue! {@ $name $(for $attr_name)?}
        )*}
        $(impl Element<tag::$tag> {$(
            keyvalue! {@ $name2 $(for $attr_name2)?}
        )*})*
    };

    (@ $name:ident) => {
        pub fn $name(mut self, value: impl Into<Text>) -> Self {
            if self.props.attributes.is_none() {
                self.props.attributes = Some(Default::default())
            }
            unsafe {self.props.attributes.as_mut().unwrap_unchecked()}
                .insert(stringify!($name), value.into());
            self
        }
    };
    (@ $name:ident for $attr_name:literal) => {
        pub fn $name(mut self, value: impl Into<Text>) -> Self {
            if self.props.attributes.is_none() {
                self.props.attributes = Some(Default::default())
            }
            unsafe {self.props.attributes.as_mut().unwrap_unchecked()}
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
    ins {
        cite,
        datetime,
    }
    label {
        for_ for "for",
    }
    li {
        value
    }
    link {
        href,
        hreflang,
        imagesizes,
        imagesecret,
        integrity,
        media,
        rel,
        sizes,
        type_ for "type"
    }
    map {
        name
    }
    meta {
        content,
        name
    }
    meter {
        value,
        min,
        max,
        low,
        hight,
        optinum,
    }
    object {
        data,
        form,
        height,
        name,
        type_ for "type",
        usemap,
        width,
    }
    ol {
        start,
    }
    optgroup {
        label
    }
    option {
        label,
        value,
    }
    output {
        for_ for "for",
        form,
        name,
    }
    portal {
        src
    }
    progress {
        max,
        value,
    }
    q {
        cite
    }
    script {
        integrity,
        src,
    }
    select {
        autocomplete,
        form,
        name,
        size,
    }
    slot {
        name
    }
    source {
        type_ for "type",
        src,
        srcset,
        sizes,
        media,
        height,
        width,
    }
    style {
        media,
    }
    textarea {
        cols,
        dirname,
        form,
        maxlength,
        minlength,
        name,
        placeholder,
    }
    th {
        abbr,
        colspan,
        headers,
        rowspan,
    }
    time {
        datetime
    }
    track {
        label,
        src,
        srclang,
    }
    video {
        poster,
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
        impl<T: Tag> Element<T> {$(
            boolean! {@ $name $(for $attr_name)?}
        )*}
        $(impl Element<tag::$tag> {$(
            boolean! {@ $name2 $(for $attr_name2)?}
        )*})*
    };

    (@ $name:ident) => {
        pub fn $name(mut self) -> Self {
            if self.props.attributes.is_none() {
                self.props.attributes = Some(Default::default())
            }
            unsafe {self.props.attributes.as_mut().unwrap_unchecked()}
                .insert(stringify!($name), "".into());
            self
        }
    };
    (@ $name:ident for $attr_name:literal) => {
        pub fn $name(mut self) -> Self {
            if self.props.attributes.is_none() {
                self.props.attributes = Some(Default::default())
            }
            unsafe {self.props.attributes.as_mut().unwrap_unchecked()}
                .insert($attr_name, "".into());
            self
        }
    };
} boolean! {
    global {
        autofocus,
        contenteditable,
        hidden,
        inert,
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
    link {
        disabled,
    }
    ol {
        reversed,
    }
    optgroup {
        disabled
    }
    option {
        disabled,
        selected,
    }
    script {
        asynch for "async",
        defer,
        nomodule,
    }
    select {
        disabled,
        multiple,
        required,
    }
    template {
        shadowrootclonable,
        shadowrootdelegatesfocus,
    }
    textarea {
        disabled,
        readonly,
        required,
        rows,
    }
    track {
        default
    }
    video {
        autoplay,
        controls,
        loops for "loop",
        muted,
        playsinline,
        src,
        width
    }
}


macro_rules! enumerated {
    (
        global {
            $( $name:literal $(for $attr_name:literal)? [$( $method:ident ($value:literal) )*] )*
        }
        $($tag:ident {
            $( $name2:literal $(for $attr_name2:literal)? [$( $method2:ident ($value2:literal) )*] )*
        })*
    ) => {
        impl<T: Tag> Element<T> {$(
            $(pub fn $method(mut self) -> Self {
                if self.props.attributes.is_none() {
                    self.props.attributes = Some(Default::default())
                }
                unsafe {self.props.attributes.as_mut().unwrap_unchecked()}
                    .insert($name, $value.into());
                self
            })*
        )*}
        $(impl Element<tag::$tag> {$(
            $(pub fn $method2(mut self) -> Self {
                if self.props.attributes.is_none() {
                    self.props.attributes = Some(Default::default())
                }
                unsafe {self.props.attributes.as_mut().unwrap_unchecked()}
                    .insert($name2, $value2.into());
                self
        })*)*})*
    };
} enumerated! {
    global {
        "autocapitalize" [
            autocapitalize_off("off")
            autocapitalize_on("on")
            autocapitalize_words("words")
            autocapitalize_charactors("charactors")
        ]
        "contenteditable" [
            contenteditable_false("false")
            contenteditable_plaintext_only("plaintext-only")
        ]
        "dir" [
            dir_ltr("ltr")
            dir_rtl("rtl")
            dir_auto("auto")
        ]
        "draggable" [
            draggable_true("true")
            draggable_false("false")
        ]
        "hidden" [
            hidden_until_found("until-found")
        ]
        "inputmode" [
            inputmode_none("none")
            inputmode_text("text")
            inputmode_decimal("decimal")
            inputmode_numeric("numeric")
            inputmode_tel("tel")
            inputmode_search("search")
            inputmode_email("email")
            inputmode("url")
        ]
        "spellcheck" [
            spellcheck_true("true")
            spellcheck_false("false")
        ]
        "translate" [
            translate_yes("yes")
            translate_no("no")
        ]
        "virtualkeyboardpolicy" [
            virtualkeyboardpolicy_auto("auto")
            virtualkeyboardpolicy_manual("manual")
        ]
        "writingsuggestions" [
            writingsuggestions_true("true")
            writingsuggestions_false("false")
        ]
    }
    a {
        "referrerpolicy" [
            referrerpolicy_no_referrer("no-referrer")
            referrerpolicy_no_referrer_when_downgrade("no-referrer-when-downgrade")
            referrerpolicy_origin("origin")
            referrerpolicy_origin_when_crossorigin("origin-when-cross-origin")
            referrerpolicy_same_origin("same-origin")
            referrerpolicy_strict_origin("strict-origin")
            referrerpolicy_strict_origin_when_cross_origin("strict-origin-when-cross-origin")
            referrerpolicy_unsafe_url("unsafe-url")
        ]
        "target" [
            target_self("_self")
            target_blank("_blank")
            target_parent("_parent")
            target_top("_top")
        ]
    }
    area {
        "target" [
            target_self("_self")
            target_blank("_blank")
            target_parent("_parent")
            target_top("_top")
        ]
    }
    audio {
        "crossorigin" [
            crossorigin_anonymous("anonymous")
            crossorigin_use_credentials("use-credentials")
        ]
        "preload" [
            preload_none("none")
            preload_metadata("metadata")
            preload_auto("auto")
        ]
    }
    base {
        "target" [
            target_self("_self")
            target_blank("_blank")
            target_parent("_parent")
            target_top("_top")
        ]
    }
    button {
        "formenctype" [
            formenctype_urlencoded("application/x-www-form-urlencoded")
            formenctype_multipart("multipart/form-data")
        ]
        "formmethod" [
            formmethod_post("post")
            formmethod_get("get")
            formmethod_dialog("dialog")
        ]
        "formtarget" [
            formtarget_self("_self")
            formtarget_blank("_blank")
            formtarget_parent("_parent")
            formtarget_top("_top")
        ]
        "popovertargetaction" [
            popovertargetaction_hide("hide")
            popovertargetaction_show("show")
            popovertargetaction_toggle("toggle")
        ]
        "type" [
            type_submit("submit")
            type_reset("reset")
            type_button("button")
        ]
    }
    form {
        "autocomplete" [
            autocomplete_on("on")
            autocomplete_off("off")
        ]
        "enctype" [
            enctype_urlencoded("application/x-www-form-urlencoded")
            enctype_multipart("multipart/form-data")
        ]
        "method" [
            method_post("post")
            method_get("get")
            method_dialog("dialog")
        ]
        "target" [
            target_self("_self")
            target_blank("_blank")
            target_parent("_parent")
            target_top("_top")
        ]
    }
    iframe {
        "referrerpolicy" [
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
        "crossorigin" [
            crossorigin_anonymous("anonymous")
            crossorigin_use_credentials("use-credentials")
        ]
        "decoding" [
            decoding_sync("sync")
            decoding_async("async")
            decoding_auto("auto")
        ]
        "loading" [
            loading_eager("eager")
            loading_lazy("lazy")
        ]
        "referrerpolicy" [
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
        "formenctype" [
            formenctype_urlencoded("application/x-www-form-urlencoded")
            formenctype_multipart("multipart/form-data")
        ]
        "formmethod" [
            formmethod_post("post")
            formmethod_get("get")
            formmethod_dialog("dialog")
        ]
        "formtarget" [
            formtarget_self("_self")
            formtarget_blank("_blank")
            formtarget_parent("_parent")
            formtarget_top("_top")
        ]
        "popovertargetaction" [
            popovertargetaction_hide("hide")
            popovertargetaction_show("show")
            popovertargetaction_toggle("toggle")
        ]
        "type" [
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
    link {
        "as" [
            as_audio("audio")
            as_document("document")
            as_embed("embed")
            as_fetch("fetch")
            as_font("font")
            as_image("image")
            as_object("object")
            as_script("script")
            as_style("style")
            as_track("track")
            as_video("video")
            as_worker("worker")
        ]
        "crossorigin" [
            crossorigin_anonymous("anonymous")
            crossorigin_use_credentials("use-credentials")
        ]
        "fetchpriority" [
            fetchpriority_high("high")
            fetchpriority_low("low")
            fetchpriority_auto("auto")
        ]
        "referrerpolicy" [
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
    meta {
        "charset" [
            charset_utf8("utf-8")
        ]
        "http-equiv" [
            http_equiv_content_security_policy("content-security-policy")
            http_equiv_content_type("content-type")
            http_equiv_default_style("default-style")
            http_equiv_refresh("refresh")
        ]
    }
    ol {
        "type" [
            type_a("a")
            type_A("A")
            type_i("i")
            type_I("i")
            type_1("1")
        ]
    }
    portal {
        "referrerpolicy" [
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
    script {
        "crossorigin" [
            crossorigin_anonymous("anonymous")
            crossorigin_use_credentials("use-credentials")
        ]
        "referrerpolicy" [
            referrerpolicy_no_referrer("no-referrer")
            referrerpolicy_no_referrer_when_downgrade("no-referrer-when-downgrade")
            referrerpolicy_origin("origin")
            referrerpolicy_origin_when_crossorigin("origin-when-cross-origin")
            referrerpolicy_same_origin("same-origin")
            referrerpolicy_strict_origin("strict-origin")
            referrerpolicy_strict_origin_when_cross_origin("strict-origin-when-cross-origin")
            referrerpolicy_unsafe_url("unsafe-url")
        ]
        "type" [
            type_module("module")
            type_importmap("importmap")
        ]
    }
    template {
        "shadowrootmode" [
            shadowrootmode_open("open")
            shadowrootmode_closed("closed")
        ]
    }
    textarea {
        "autocomplete" [
            autocomplete_on("on")
            autocomplete_off("off")
        ]
        "wrap" [
            wrap_hard("hard")
            wrap_soft("soft")
        ]
    }
    th {
        "scope" [
            scope_row("row")
            scope_col("col")
            scope_rowgroup("rowgroup")
            scope_colgroup("colgroup")
        ]
    }
    track {
        "kind" [
            kind_subtitles("subtitles")
            kind_captions("captions")
            kind_descriptions("descriptions")
            kind_chapters("chapters")
            kind_metadata("metadata")
        ]
    }
    video {
        "crossorigin" [
            crossorigin_anonymous("anonymous")
            crossorigin_use_credentials("use-credentials")
        ]
        "preload" [
            preload_none("none")
            preload_metadata("metadata")
            preload_auto("auto")
        ]
    }
}
