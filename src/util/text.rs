use std::{borrow::Cow, rc::Rc};


#[derive(Clone)]
pub struct Text(
    Rc<Cow<'static, str>>
);

impl std::ops::Deref for Text {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(Rc::new(value.into()))
    }
}

impl From<&'static str> for Text {
    fn from(value: &'static str) -> Self {
        Self(Rc::new(value.into()))
    }
}

impl From<bool> for Text {
    fn from(value: bool) -> Self {
        Self(Rc::new((if value {"true"} else {"false"}).into()))
    }
}

macro_rules! integer_value {
    ($($t:ty)*) => {$(
        impl From<$t> for Text {
            fn from(value: $t) -> Self {
                Self(Rc::new(value.to_string().into()))
            }
        }
    )*};
} integer_value! { u8 usize i32 }

#[cfg(feature="DEBUG")]
const _: () = {
    impl std::fmt::Debug for Text {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", &*self.0)
        }
    }
};
