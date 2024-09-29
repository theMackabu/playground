#[doc(hidden)]
#[macro_export]
macro_rules! _impl_from_for_value {
    ($($t:ty => $variant:ident),+) => {
        $(
            impl From<$t> for Value {
                fn from(item: $t) -> Self {
                    Value::$variant(item.into())
                }
            }
        )+

        #[cfg(not(feature = "serde"))]
        impl From<&'static [Value]> for Value {
            fn from(item: &'static [Value]) -> Self {
                Value::Slice(item)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_value_methods {
    ($($variant:ident => $type:ty, $as_method:ident, $is_method:ident);+ $(;)?) => {
        impl Value {
            $(
                pub fn $is_method(&self) -> bool {
                    matches!(self, Value::$variant(_))
                }

                pub fn $as_method(&self) -> Option<$type> {
                    match self {
                        Value::$variant(v) => Some(v.clone()),
                        _ => None,
                    }
                }
            )+

            #[cfg(not(feature = "serde"))]
            pub fn as_slice(&self) -> Option<&'static [Value]> {
                match self {
                    Value::Slice(v) => Some(v),
                    _ => None,
                }
            }

            #[cfg(not(feature = "serde"))]
            pub fn is_slice(&self) -> bool {
                matches!(self, Value::Slice(_))
            }

            pub fn to_value(self) -> Value {
                self
            }

            pub fn is_null(&self) -> bool {
                matches!(self, Value::Null)
            }
        }
    };
}

#[doc(inline)]
pub use _impl_from_for_value as impl_for;

#[doc(inline)]
pub use _impl_value_methods as impl_methods;
