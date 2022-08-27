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
                for attr in tag.get_attributes(){
                    println!("attribute {} = {}", attr.0, attr.1.as_primitive().unwrap().as_string())
                }

                if let Some(children) = tag.get_children(){
                    for child in children{
                        println!("child: {}", child.get_name());
                    }
                }
            }
        },
        Err(error)=>{ print!("{}", error); }
    }
}
