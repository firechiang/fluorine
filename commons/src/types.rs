use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]// remove type to json string
pub enum ProtoType<'a> {
    Str(&'a str),
    String(String),
    Bool(bool)
}

impl fmt::Display for ProtoType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProtoType::Str(str) => write!(f,"{}",str),
            ProtoType::String(string) => write!(f, "{}", string),
            ProtoType::Bool(bool) => write!(f, "{}", bool),
        }
    }
}

pub trait FromToNumber<T> {
    
}