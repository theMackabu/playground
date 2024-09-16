use crate::Value;

use std::{
    any::TypeId,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    sync::OnceLock,
};

pub trait ValueFormatter: Send + Sync {
    fn format(&self, value: &dyn std::any::Any, f: &mut Formatter) -> fmt::Result;
}

pub struct FormatterRegistry {
    formatters: HashMap<TypeId, Box<dyn ValueFormatter>>,
}

impl FormatterRegistry {
    fn new() -> Self { Self { formatters: HashMap::new() } }

    fn register<T: 'static>(&mut self, formatter: Box<dyn ValueFormatter>) { self.formatters.insert(TypeId::of::<T>(), formatter); }

    pub fn get(&self, type_id: TypeId) -> Option<&dyn ValueFormatter> { self.formatters.get(&type_id).map(|boxed| boxed.as_ref()) }
}

pub static FORMATTER_REGISTRY: OnceLock<FormatterRegistry> = OnceLock::new();

pub fn init_registry() -> FormatterRegistry {
    let mut registry = FormatterRegistry::new();

    registry.register::<()>(Box::new(NullFormatter));
    registry.register::<bool>(Box::new(SimpleFormatter));
    registry.register::<usize>(Box::new(SimpleFormatter));
    registry.register::<f64>(Box::new(SimpleFormatter));
    registry.register::<String>(Box::new(StringFormatter));
    registry.register::<&'static str>(Box::new(StringFormatter));
    registry.register::<Vec<Value>>(Box::new(SequenceFormatter::new("[", "]")));
    registry.register::<&'static [Value]>(Box::new(SequenceFormatter::new("[", "]")));
    registry.register::<HashMap<String, Value>>(Box::new(MapFormatter));

    registry
}

struct NullFormatter;
impl ValueFormatter for NullFormatter {
    fn format(&self, _: &dyn std::any::Any, f: &mut Formatter) -> fmt::Result { f.write_str("null") }
}

struct SimpleFormatter;
impl ValueFormatter for SimpleFormatter {
    fn format(&self, value: &dyn std::any::Any, f: &mut Formatter) -> fmt::Result {
        if let Some(v) = value.downcast_ref::<bool>() {
            v.fmt(f)
        } else if let Some(v) = value.downcast_ref::<usize>() {
            v.fmt(f)
        } else if let Some(v) = value.downcast_ref::<f64>() {
            v.fmt(f)
        } else {
            Err(fmt::Error)
        }
    }
}

struct StringFormatter;
impl ValueFormatter for StringFormatter {
    fn format(&self, value: &dyn std::any::Any, f: &mut Formatter) -> fmt::Result {
        if let Some(s) = value.downcast_ref::<String>() {
            write!(f, "\"{}\"", s)
        } else if let Some(s) = value.downcast_ref::<&'static str>() {
            write!(f, "\"{}\"", s)
        } else {
            Err(fmt::Error)
        }
    }
}

struct SequenceFormatter {
    start: &'static str,
    end: &'static str,
}

impl SequenceFormatter {
    fn new(start: &'static str, end: &'static str) -> Self { Self { start, end } }
}

impl ValueFormatter for SequenceFormatter {
    fn format(&self, value: &dyn std::any::Any, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.start)?;
        let mut first = true;

        if let Some(vec) = value.downcast_ref::<Vec<Value>>() {
            for item in vec.iter() {
                if !first {
                    f.write_str(", ")?;
                }
                item.fmt(f)?;
                first = false;
            }
        } else if let Some(slice) = value.downcast_ref::<&'static [Value]>() {
            for item in *slice {
                if !first {
                    f.write_str(", ")?;
                }
                item.fmt(f)?;
                first = false;
            }
        } else {
            return Err(fmt::Error);
        }

        f.write_str(self.end)
    }
}

struct MapFormatter;
impl ValueFormatter for MapFormatter {
    fn format(&self, value: &dyn std::any::Any, f: &mut Formatter) -> fmt::Result {
        if let Some(map) = value.downcast_ref::<HashMap<String, Value>>() {
            f.write_str("{")?;
            let mut first = true;
            for (key, value) in map {
                if !first {
                    f.write_str(", ")?;
                }
                write!(f, "\"{}\": {}", key, value)?;
                first = false;
            }
            f.write_str("}")
        } else {
            Err(fmt::Error)
        }
    }
}
