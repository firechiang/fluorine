
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::header::HeaderMap;
use serde_json::value::Value;
use std::fmt;

#[derive(Serialize,Deserialize)]
pub struct Data {
    jsonrpc:String,
    id:u32,
    method:String,
    params:Vec<()>
}



fn main() {
}




