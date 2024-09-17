use std::borrow::Cow;

pub type Str = &'static str;

pub struct Json<T>(pub T);

pub struct Text<'a>(pub Cow<'a, str>);

pub struct Bytes<'a>(pub Cow<'a, [u8]>);
