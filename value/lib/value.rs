#![allow(dead_code)]

mod display;
mod macros;

use display::{init_registry, FORMATTER_REGISTRY};
use macros::{impl_for, impl_methods};

use std::{
    any::TypeId,
    collections::HashMap,
    convert::From,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Float(f64),
    Number(usize),
    String(String),
    Static(&'static str),
    Array(Vec<Value>),
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
            Value::Array(_) => TypeId::of::<Vec<Value>>(),
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
    fn to_value(self) -> Value { Value::from(self) }
}

impl<T: ToValue> From<Vec<T>> for Value {
    fn from(vec: Vec<T>) -> Self { Value::Array(vec.into_iter().map(ToValue::to_value).collect()) }
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

impl_methods! {
    Bool => bool, as_bool, is_bool;
    Float => f64, as_float, is_float;
    Number => usize, as_number, is_number;
    String => String, as_string, is_string;
    Static => &'static str, as_str, is_str;
    Array => Vec<Value>, as_array, is_array;
    Slice => &'static [Value], as_slice, is_slice;
    Object => HashMap<String, Value>, as_object, is_object;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_creation() {
        assert_eq!(Value::Null, Value::Null);
        assert_eq!(Value::Bool(true), Value::from(true));
        assert_eq!(Value::Float(3.14), Value::from(3.14));
        assert_eq!(Value::Number(42), Value::from(42usize));
        assert_eq!(Value::String("hello".to_string()), Value::from("hello".to_string()));
        assert_eq!(Value::Static("static"), Value::from("static"));
        assert_eq!(Value::Array(vec![Value::Null]), Value::from(vec![Value::Null]));
        assert_eq!(Value::Slice(&[Value::Null]), Value::from(&[Value::Null][..]));

        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Null);
        assert_eq!(Value::Object(map.clone()), Value::from(map));
    }

    #[test]
    fn test_to_value() {
        assert_eq!(true.to_value(), Value::Bool(true));
        assert_eq!(3.14.to_value(), Value::Float(3.14));
        assert_eq!(42usize.to_value(), Value::Number(42));
        assert_eq!("hello".to_string().to_value(), Value::String("hello".to_string()));
    }

    #[test]
    fn test_from_option() {
        assert_eq!(Value::from(Some(42usize)), Value::Number(42));
        assert_eq!(Value::from(None::<usize>), Value::Null);
    }

    #[test]
    fn test_methods() {
        let bool_val = Value::Bool(true);
        assert!(bool_val.is_bool());
        assert_eq!(bool_val.as_bool(), Some(true));

        let float_val = Value::Float(3.14);
        assert!(float_val.is_float());
        assert_eq!(float_val.as_float(), Some(3.14));

        let num_val = Value::Number(42);
        assert!(num_val.is_number());
        assert_eq!(num_val.as_number(), Some(42));

        let string_val = Value::String("hello".to_string());
        assert!(string_val.is_string());
        assert_eq!(string_val.as_string(), Some("hello".to_string()));

        let static_val = Value::Static("static");
        assert!(static_val.is_str());
        assert_eq!(static_val.as_str(), Some("static"));

        let array_val = Value::Array(vec![Value::Null]);
        assert!(array_val.is_array());
        assert_eq!(array_val.as_array(), Some(vec![Value::Null]));

        let slice_val = Value::Slice(&[Value::Null]);
        assert!(slice_val.is_slice());
        assert_eq!(slice_val.as_slice(), Some(&[Value::Null][..]));

        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Null);
        let object_val = Value::Object(map.clone());
        assert!(object_val.is_object());
        assert_eq!(object_val.as_object(), Some(map));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Value::Null), "null");
        assert_eq!(format!("{}", Value::Bool(true)), "true");
        assert_eq!(format!("{}", Value::Float(3.14)), "3.14");
        assert_eq!(format!("{}", Value::Number(42)), "42");
        assert_eq!(format!("{}", Value::String("hello".to_string())), "\"hello\"");
        assert_eq!(format!("{}", Value::Static("static")), "\"static\"");
        assert_eq!(format!("{}", Value::Array(vec![Value::Null])), "[null]");
        assert_eq!(format!("{}", Value::Slice(&[Value::Null])), "[null]");

        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Null);
        assert_eq!(format!("{}", Value::Object(map)), "{\"key\": null}");
    }
}
