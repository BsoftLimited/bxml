use crate::bxml::json::JSonNone;
use crate::bxml::json::JSonPrimitive;
use crate::bxml::Tag;
use crate::bxml::JSonArray;
use crate::bxml::JSonObject;
use crate::bxml::JSonItem;
use crate::bxml::lexer::Token;
use crate::bxml::Lexer;

fn variant_equal(a: &Token, b: &Token)->bool{
    return std::mem::discriminant(a) == std::mem::discriminant(b);
}

pub struct ParserResult{ pub tags: Vec<Tag>, pub errors: Vec<String>}


pub enum Expression{
    SingleTag{name:String, attributes:Vec<Expression>},
    OpeningTag{name:String, attributes:Vec<Expression>},
    ClosingTag{ name:String}, Atribute{ name:String, value:Box<dyn JSonItem>}, None}

pub struct Parser{ lexer:Box<Lexer>, errors: Vec<String> , current: Token}

impl Parser{
    pub fn new(data:&str)->Self{
        let mut parser = Parser{
            lexer: Box::new(Lexer::new(data)), errors: Vec::new() , current: Token::None };
        parser.next_token();
        return parser;
    }

    pub fn get_next(&mut self)->Expression{
        while !matches!(self.current, Token::None){
            if matches!(self.current, Token::OpenAngleBracket){
                self.pop_token();
                return self.get_tag();
            }
            let token = self.pop_token();
            self.errors.push(format!("Unexpected token: {:?} expecting a opening angle bracket(<)", token));
        }
        return Expression::None;
    }
    
    fn next_token(&mut self)->bool{
        while self.lexer.has_next(){
            match self.lexer.get_next_token(){
                Err(error) =>{ self.errors.push(String::from(error)); }
                Ok(token) =>{
                    self.current = token;
                    return true;
                }
            }
        }
        self.current = Token::None;
        return false;
    }

    fn pop_token(&mut self)->Token{
        let init = self.current.clone();
        self.next_token();
        return init;
    }

    fn unwrap(token:Token)->String{
        let mut init = String::new();
        if let Token::Name(value) = token{
            init = value;
        }else if let Token::String(value) = token{
            init = value;
        } 
        init
    }

    fn get_name(&mut self)->Token{
        while !matches!(self.current, Token::None){
            if matches!(self.current, Token::Name(_)){
                return self.pop_token();
            }else{
                self.errors.push(format!("Unexpected token: {:?} expecting a name", &self.current));
            }
            self.next_token();
        }
        return Token::None;
    }

    fn get_value(&mut self)->Box<dyn JSonItem>{
        while !matches!(self.current, Token::None){
            if matches!(self.current, Token::String(_)) || matches!(self.current, Token::Boolean(_)) || matches!(self.current, Token::Number(_)){
                let init = self.pop_token();
                if let Token::String(value) = &init{
                    return Box::new(JSonPrimitive::from(value));
                }else if let Token::Boolean(value) = &init{
                    return Box::new(JSonPrimitive::from(value));
                }else if let Token::Number(value) = &init{
                    return Box::new(JSonPrimitive::from(value));
                }
            }if matches!(self.current, Token::OpenCurlyBracket){
                self.next_token();
                return self.get_object();
            }if matches!(self.current, Token::OpenSquareBracket){
                self.next_token();
                return self.get_array();
            }else{
                self.errors.push(format!("Unexpected token: {:?} expecting an string literal or boolean value", &self.current));
                self.next_token();
            }
        }
        return Box::new(JSonNone);
    }

    fn find(&mut self, token: Token)->Token{
        while !matches!(self.current, Token::None){
            if variant_equal(&self.current, &token){
                return self.pop_token();
            }else{
                self.errors.push(format!("Unexpected token: {:?} expecting a token of type {:?}", &self.current, token));
            }
            self.next_token();
        }
        return Token::None;
    }

    fn check_attribute(&mut self, name: Token)->Expression{
        let mut proceed = false;
        while !matches!(self.current, Token::None){
            if matches!(self.current, Token::Equals){
                if !proceed{
                    proceed = true;
                }else{
                    self.errors.push(format!("Unexpected token: {:?} expecting an attribute value", self.current));
                }
            }else if matches!(self.current, Token::String(_)) || matches!(self.current, Token::Number(_)) || matches!(self.current, Token::Boolean(_)) || matches!(self.current, Token::OpenCurlyBracket) || matches!(self.current, Token::OpenSquareBracket){
                if !proceed{ self.errors.push(format!("Add an equals(=) sign beore the attribute value")); }

                return Expression::Atribute{ name: Parser::unwrap(name), value: self.get_value()};
            }else{
                let post = if proceed { "attribute value" }else{ "equals(=) sign" };
                self.errors.push(format!("Unexpected token: {:?} expecting an {}", self.current, post ));
                if matches!(self.current, Token::ClosingAngleBracket) || matches!(self.current, Token::ForwardSlash){
                    break;
                }
            }
            self.next_token();
        }
        return Expression::Atribute{ name: Parser::unwrap(name), value: Box::new(JSonPrimitive::new())};
    }

    fn get_closing(&mut self)->Expression{
        let name = self.get_name();
        if !matches!(self.current, Token::ClosingAngleBracket) {
            self.errors.push(format!("unexpect token: > expecting after {:?}", &name));
        }else{
            self.pop_token();
        }
        return Expression::ClosingTag{ name: Parser::unwrap(name) };
    }

    fn get_opening(&mut self)->Expression{
        let name = self.get_name();
        let mut attributes: Vec<Expression> = Vec::new();
        while matches!(self.current, Token::Name(_)){
            let value = self.pop_token();
            attributes.push(self.check_attribute(value));   
        }

        while !matches!(self.current, Token::None){
            if matches!(self.current, Token::ClosingAngleBracket) || matches!(self.current, Token::ForwardSlash){
                let init = self.pop_token();
                if let Token::ClosingAngleBracket = init{
                    return Expression::OpeningTag{ name: Parser::unwrap(name), attributes};
                }else if let Token::ForwardSlash = init{
                    if !matches!(self.current, Token::ClosingAngleBracket){
                        self.errors.push(format!("unexpect token: > expecting after / but got {:?}", self.current));
                    }else{
                        self.pop_token();
                    }
                    return Expression::SingleTag{ name: Parser::unwrap(name), attributes };
                }
            }else{
                self.errors.push(format!("unexpected token: / or > or name token but got {:?}", self.current));
            }
            self.next_token();
        }
        return Expression::None;
    }

    fn get_tag(&mut self)->Expression{
        if matches!(self.current, Token::ForwardSlash){
            self.pop_token();
            return self.get_closing();
        }
        return self.get_opening();
    }

    fn get_object(&mut self)->Box<dyn JSonItem>{
        let mut init_object = JSonObject::new();
        let mut key: Option<String> = None;
        let mut complete = false;
        loop{
            if matches!(self.current, Token::String(_)) || matches!(self.current, Token::SemiColumn) || matches!(self.current, Token::Coma){
                let token = self.pop_token();
                if let Token::SemiColumn = &token{
                    if key.is_some(){
                        init_object.add( key.as_ref().unwrap(), self.get_value());
                        complete = true;
                    }else{
                        self.errors.push(format!("Unexpected semicolumn expecting a json key"));
                    }
                }else if let Token::Coma = &token{
                    if !complete {
                        if key.is_some(){
                            self.errors.push(format!("Unexpected token: {:?} exprcting semicolumn(:)", self.current));    
                        }else if !complete && key.is_none(){
                            self.errors.push(format!("Unexpected token: {:?} exprcting string token", self.current));
                        }
                    }
                    key = None;
                    complete = false;
                }else if let Token::String(value) = &token{
                    if key.is_some() && !complete{
                        self.errors.push(format!("Unexpected token: {:?} expecting a semi column(:)", &key));
                    }else if complete{
                        self.errors.push(format!("Unexpected token: {:?} expecting a semi coma(,)", &key));
                    }   
                    key = Some(value.clone());
                }
            } else if matches!(self.current, Token::ClosingCurlyBracket) || matches!(self.current, Token::None){
                self.next_token();
                break;
            } else{
                let token = self.pop_token();
                self.errors.push(format!("Unexpected token: {:?}", token));
            }
        }
        return Box::new(init_object);
    }

    fn get_array(&mut self)->Box<dyn JSonItem>{
        let mut array = JSonArray::new();

        while !matches!(self.current, Token::None) && !matches!(self.current, Token::ClosingSquareBracket){
            array.add(self.get_value());
            if matches!(self.current, Token::ClosingSquareBracket){
                self.next_token();
                break;
            } if matches!(self.current, Token::Coma){
                self.next_token();
            }else{
                self.errors.push(format!("Unexpected token: {:?}, expected coma(,) or closing square bracket(])", self.current));
            }
        }
        return Box::new(array);
    }

    pub fn has_errors(&self)-> bool{ self.errors.len() > 0 }
    pub fn get_errors(&self)-> &Vec<String>{ &self.errors }
    pub fn display_errors(&self){
        for error in &self.errors{
            println!("{}", error);
        }
    }
}
