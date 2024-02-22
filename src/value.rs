use std::fmt;

#[derive(Debug, Clone)]
pub enum JsonValue {
    JsonObject(Vec<(String, Box<JsonValue>)>),
    JsonArray(Vec<Box<JsonValue>>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonValue::JsonObject(ref obj) => {
                write!(f, "{{")?;
                for (i, (ref key, ref value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            },
            JsonValue::JsonArray(ref arr) => {
                write!(f, "[")?;
                for (i, ref value) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            },
            JsonValue::String(ref s) => write!(f, "\"{}\"", s),
            JsonValue::Number(ref n) => write!(f, "{}", n),
            JsonValue::Boolean(ref b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
        }
    }
}

pub trait AstDisplay {
    fn ast_fmt(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result;
}

impl AstDisplay for JsonValue {
    fn ast_fmt(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        match *self {
            JsonValue::JsonObject(ref obj) => {
                writeln!(f, "{}JsonObject of size {} {{", " ".repeat(depth), obj.len())?;
                for (ref key, ref value) in obj {
                    write!(f, "{}Key: {}, Value: ", " ".repeat(depth+2), key)?;
                    value.ast_fmt(f, depth+4)?;
                }
                writeln!(f, "{}}}", " ".repeat(depth))
            },
            JsonValue::JsonArray(ref arr) => {
                writeln!(f, "{}JsonArray of size {} [", " ".repeat(depth), arr.len())?;
                for value in arr {
                    write!(f, "{}", " ".repeat(depth+2))?;
                    value.ast_fmt(f, depth+2)?;
                }
                writeln!(f, "{}]", " ".repeat(depth))
            },
            JsonValue::String(ref s) => writeln!(f, "{}String: \"{}\"", " ".repeat(depth), s),
            JsonValue::Number(ref n) => writeln!(f, "{}Number: {}", " ".repeat(depth), n),
            JsonValue::Boolean(ref b) => writeln!(f, "{}Boolean: {}", " ".repeat(depth), b),
            JsonValue::Null => writeln!(f, "{}Null", " ".repeat(depth)),
        }
    }
}

pub struct AstWrapper<'a>(pub &'a JsonValue);

impl<'a> fmt::Display for AstWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.ast_fmt(f, 0)
    }
}