use std::{borrow::Cow, rc::Rc};


#[derive(Clone)]
pub struct VText(
    Rc<Cow<'static, str>>
);

impl std::ops::Deref for VText {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for VText {
    fn from(value: String) -> Self {
        Self(Rc::new(value.into()))
    }
}
impl From<&'static str> for VText {
    fn from(value: &'static str) -> Self {
        Self(Rc::new(value.into()))
    }
}
impl From<bool> for VText {
    fn from(value: bool) -> Self {
        Self(Rc::new((if value {"true"} else {"false"}).into()))
    }
}
macro_rules! from_integer {
    ($($t:ty)*) => {$(
        impl From<$t> for VText {
            fn from(value: $t) -> Self {
                Self(Rc::new(value.to_string().into()))
            }
        }
    )*};
} from_integer! { u8 usize i32 }

#[cfg(feature="DEBUG")]
const _: () = {
    impl std::fmt::Debug for VText {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", &*self.0)
        }
    }
};
