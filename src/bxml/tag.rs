
use crate::bxml::parser::ParserResult;
use crate::bxml::Expression;
use crate::Parser;
use crate::bxml::JSonItem;
use std::collections::HashMap;

pub enum Tagtype{Single, Parent{ children: Vec<Box<Tag>>} }
impl Tagtype{
    pub fn new()->Self{ Tagtype::Parent{children:Vec::new() } }
}

pub struct Tag{ name: String, attributes: HashMap<String, Box<dyn JSonItem>> , tag_type: Tagtype}


impl Tag{
    pub fn new(name:String)->Self{
        return Tag{ name, attributes:HashMap::new(), tag_type: Tagtype::Single };
    }

    pub fn parse(data: &str)->ParserResult{
        let mut parser = Parser::new(data);

        let mut tags:Vec<Tag> = Vec::new();
        let mut errors:Vec<String> = Vec::new();
        loop{
            let mut init = parser.get_next();
            if let Expression::None = init{
                for error in parser.get_errors(){
                    errors.push(error.clone());
                }
                break;
            }

            if let Expression::OpeningTag{name, attributes} = init{
                let mut tag = Tag::new(name);
                for attribute in attributes{
                    if let Expression::Atribute{ name, value} = attribute{
                        tag.add_attribute(name, value);
                    }
                }
                tag.make_parent();
                tags.push(tag);
                continue;
            }

            if let Expression::SingleTag{name, attributes} = init{
                let mut tag = Tag::new(name);
                for attribute in attributes{
                    if let Expression::Atribute{ name, value} = attribute{
                        tag.add_attribute(name, value);
                    }
                }

                if tags.last_mut().is_some() && tags.last_mut().unwrap().is_parent(){
                    tags.last_mut().unwrap().add_child(tag);
                }else{
                    tags.push(tag);
                }
                continue;
            }

            if let Expression::ClosingTag{name} = init{
                if tags.len() > 1{
                    if tags.last().unwrap().name.eq(&name){
                        let last = tags.pop().unwrap();
                        tags.last_mut().unwrap().add_child(last);
                    }else{
                        errors.push(format!("unexpected closing tag for {}, try closing {} first", name, &tags.last().unwrap().name));
                    }
                }
            }
        }
        return ParserResult{ errors, tags};
    }

    pub fn make_parent(&mut self){ self.tag_type = Tagtype::new();}

    pub fn is_parent(&self)->bool{
        return matches!(self.tag_type, Tagtype::Parent{children: _});
    }

    pub fn add_attribute(&mut self, nam:String, value:Box<dyn JSonItem>){
        self.attributes.insert(nam, value); 
    }

    pub fn remove_attribute(&mut self, nam:&str){
        self.attributes.remove(nam); 
    }

    pub fn get_attributes(&self)->&HashMap<String, Box<dyn JSonItem>>{ return &self.attributes; }

    pub fn get_attributes_mut(&mut self)->&mut HashMap<String, Box<dyn JSonItem>>{ return &mut self.attributes; }

    pub fn add_child(&mut self, tag:Tag)->bool{
        if !self.is_parent(){ self.make_parent(); }
        if let Tagtype::Parent{ children} =  &mut self.tag_type{
            children.push(Box::new(tag));
            return true;
        }
        return false;
    }

    pub fn get_children(&self)->Option<&Vec<Box<Tag>>>{
        if let Tagtype::Parent{children} =  &self.tag_type{
            return Option::from(children);
        }
        return None;
    }

    pub fn get_children_mut(&mut self)->Option<&mut Vec<Box<Tag>>>{
        if let Tagtype::Parent{children} =  &mut self.tag_type{
            return Option::from(children);
        }
        return None;
    }

    fn format(&self, space: &mut String)->String{
        let mut builder = space.clone();
        builder.push_str("<");
        builder.push_str(self.name.as_ref());
        for attribute in &self.attributes{
            builder.push_str(format!(" {} = {}", attribute.0, attribute.1.to_string()).as_ref());
        }
        match &self.tag_type{
            Tagtype::Parent{children} => {
                builder.push('>');
                let mut init = format!("{}\t", space);
                for child in children{
                    builder.push_str(format!("\n{}", child.format(&mut init)).as_ref());
                }
                builder.push_str(format!("\n{}</{}>", space, self.name).as_ref());
            },
            _ =>{ builder.push_str("/>"); }
        }
        return builder;
    }

    pub fn to_string(&self)->String{ 
        return self.format(&mut String::from(""));
    }
}
