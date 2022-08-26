mod bxml;

use crate::bxml::Tag;
use std::io::Read;

use crate::bxml::{Parser};

fn main() {
    match &mut std::fs::File::open("./test.bxml"){
        Ok(file) =>{
            let  mut content = String::new();
            if let Result::Err(error) = file.read_to_string(&mut content){
                print!("{}", error);
            }

            let result = Tag::parse(content.as_ref());
            for error in result.errors{
                println!("{}", error);
            }
            for tag in result.tags{
                println!("{}", tag.to_string());
            }
        },
        Err(error)=>{ print!("{}", error); }
    }
}
