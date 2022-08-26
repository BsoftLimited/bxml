mod tag;
mod lexer;
mod parser;
mod json;

pub use json::JSonObject;
pub use json::JSonArray;
pub use json::JSon;
pub use json::JSonItem;

pub use tag::{Tag};
pub use lexer::{Token, Lexer};
pub use parser::{ Parser, Expression};

pub struct XmlResult{ tags: Vec<Box<Tag>>,  errors: Vec<String> }
impl XmlResult{
    fn new ()->Self{ XmlResult{ tags:Vec::new(), errors:Vec::new() } }
}