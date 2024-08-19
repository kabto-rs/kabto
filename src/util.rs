use std::borrow::Cow;


pub struct Text(
    Cow<'static, str>
);
const _: () = {
    impl Into<Cow<'static, str>> for Text {
        fn into(self) -> Cow<'static, str> {
            self.0
        }
    }
    impl std::ops::Deref for Text {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
};
const _: () = {
    impl From<String> for Text {
        fn from(value: String) -> Self {
            Self(value.to_string().into())
        }
    }
    impl From<&'static str> for Text {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }

    macro_rules! integer_value {
        ($($t:ty)*) => {$(
            impl From<$t> for Text {
                fn from(value: $t) -> Self {
                    Self(value.to_string().into())
                }
            }
        )*};
    } integer_value! { u8 usize i32 }
};
