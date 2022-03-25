use std::fmt;

#[derive(Debug)]
enum Type {
    String(String),
    Bool(bool)
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::String(s) => write!(f, "{}", s),
            Type::Bool(b) => write!(f, "{}", b),
        }
    }
}