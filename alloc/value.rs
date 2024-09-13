#![allow(dead_code)]

mod display;
mod macros;

use self::display::{init_registry, FORMATTER_REGISTRY};
use self::macros::{impl_for, impl_list, impl_methods};

use std::{
    any::TypeId,
    collections::HashMap,
    convert::From,
    fmt::{self, Debug, Display, Formatter},
    iter::FromIterator,
};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Float(f64),
    Number(usize),
    String(String),
    Static(&'static str),
    Array(crate::List<Value>),
    Slice(&'static [Value]),
    Object(HashMap<String, Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let registry = FORMATTER_REGISTRY.get_or_init(init_registry);
        let type_id = match self {
            Value::Null => TypeId::of::<()>(),
            Value::Bool(_) => TypeId::of::<bool>(),
            Value::Float(_) => TypeId::of::<f64>(),
            Value::Number(_) => TypeId::of::<usize>(),
            Value::String(_) => TypeId::of::<String>(),
            Value::Static(_) => TypeId::of::<&'static str>(),
            Value::Array(_) => TypeId::of::<crate::List<Value>>(),
            Value::Slice(_) => TypeId::of::<&'static [Value]>(),
            Value::Object(_) => TypeId::of::<HashMap<String, Value>>(),
        };

        if let Some(formatter) = registry.get(type_id) {
            let value: &dyn std::any::Any = match self {
                Value::Null => &(),
                Value::Bool(b) => b,
                Value::Float(n) => n,
                Value::Number(n) => n,
                Value::String(s) => s,
                Value::Static(s) => s,
                Value::Array(a) => a,
                Value::Slice(s) => s,
                Value::Object(o) => o,
            };
            formatter.format(value, f)
        } else {
            Err(fmt::Error)
        }
    }
}

pub trait ToValue {
    fn to_value(self) -> Value;
}

impl<T: Into<Value>> ToValue for T
where
    T: Clone,
    Value: From<T>,
{
    fn to_value(self) -> Value {
        Value::from(self)
    }
}

impl<T: ToValue> From<crate::List<T>> for Value {
    fn from(list: crate::List<T>) -> Self {
        Value::Array(list.into_iter().map(ToValue::to_value).collect())
    }
}

impl_list! {
    bool => Bool,
    f64 => Float,
    usize => Number,
    String => String,
    &'a str => String,
    crate::List<Value> => Array,
    HashMap<String, Value> => Object
}

impl_for! {
    bool => Bool,
    f64 => Float,
    usize => Number,
    String => String,
    &'static str => Static,
    &'static [Value] => Slice,
    HashMap<String, Value> => Object
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

impl FromIterator<Value> for crate::List<Value> {
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
        let mut list = crate::List::new();
        for value in iter {
            list.push(value);
        }
        list
    }
}

impl_methods! {
    Bool => bool, as_bool, is_bool;
    Float => f64, as_float, is_float;
    Number => usize, as_number, is_number;
    String => String, as_string, is_string;
    Static => &'static str, as_str, is_str;
    Array => crate::List<Value>, as_array, is_array;
    Slice => &'static [Value], as_slice, is_slice;
    Object => HashMap<String, Value>, as_object, is_object;
}
