#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token{
	Name(String), String(String), Number(String), Boolean(String),
	ForwardSlash, OpenAngleBracket, ClosingAngleBracket, OpenSquareBracket, ClosingSquareBracket,
	OpenCurlyBracket, ClosingCurlyBracket, SemiColumn, Coma, Equals, None}

pub struct Lexer{ index:usize, current:char, data: String }

impl Lexer{
    pub fn new(data:&str)->Self{
        let string = String::from(data);
        let init = string.chars().nth(0).unwrap();
        Lexer{ index:0, current:init, data: string, }
    }
    
    pub fn has_next(&mut self)->bool{
        while self.index < self.data.len(){
            self.current = self.data.chars().nth(self.index).unwrap();
            let passable = (self.current == ' ') || (self.current == '\n') || (self.current == '\t');
            if ! passable { return true; }
            self.index += 1;
        }
        return false;
    }
    
    fn pop(&mut self)->char{
        let init = self.data.chars().nth(self.index).unwrap();
        self.index += 1;
        return init;
    }
    
    pub fn get_next_token(&mut self)->Result<Token, String>{
		if self.current.is_alphabetic(){
			return self.get_name_token();
		}
		
		if self.current.is_numeric(){
			return self.get_number_token();
		}
		match self.current{
			'<' => { self.pop(); return Ok(Token::OpenAngleBracket); }
			'>' => { self.pop(); return Ok(Token::ClosingAngleBracket); }
			'{' => { self.pop(); return Ok(Token::OpenCurlyBracket); }
			'}' => { self.pop(); return Ok(Token::ClosingCurlyBracket); }
			'[' => { self.pop(); return Ok(Token::OpenSquareBracket); }
			']' => { self.pop(); return Ok(Token::ClosingSquareBracket); }
			':' => { self.pop(); return Ok(Token::SemiColumn); }
			',' => { self.pop(); return Ok(Token::Coma); }
			'/' => { self.pop(); return Ok(Token::ForwardSlash); }
			'=' => { self.pop(); return Ok(Token::Equals); }
			'"' => { return self.get_string_token(); }
			'\''=> { return self.get_string_token(); }
			_   => { return Result::Err(format!("unexpected token {} encountered", self.pop())); }
		}
	}

	fn get_name_token(&mut self)->Result<Token, String>{
		let mut builder = String::new();
		
		loop{
			builder.push(self.pop());
			if self.index < self.data.len(){
				self.current = self.data.chars().nth(self.index).unwrap();
				if !self.current.is_alphanumeric(){ break;}
			}
		}

		if builder.eq("true") || builder.eq("false"){
			return Ok(Token::Boolean(builder));
		}
		return Ok(Token::Name(builder));
	}

	fn get_number_token(&mut self)->Result<Token, String>{
		let mut builder = String::new();
		
		loop{
			builder.push(self.pop());
			if self.index < self.data.len(){
				self.current = self.data.chars().nth(self.index).unwrap();
				if !self.current.is_numeric(){ break;}
			}
		}
		return Ok(Token::Number(builder));
	}
	
	fn get_string_token(&mut self)->Result<Token, String>{
		let open = self.pop();
		let mut builder = String::new();
		while self.index < self.data.len(){
		    let close = self.data.chars().nth(self.index).unwrap();
			if close == open{
                self.pop();
				return Ok(Token::String( builder));
			}else{
				builder.push(self.pop());
			}
		}
		return Err(format!("Expecting a closing {}", if open == '\''{ "'"} else {"\""}));
	}
}