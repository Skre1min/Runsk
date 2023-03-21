use std::{collections::HashMap, intrinsics::breakpoint};


struct Scanner{
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}
impl Scanner {
    
    fn new(source: String)-> Scanner{
        Scanner{
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }
    
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }
    
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(token);
    }
    fn scan_tokens(&mut self) -> Vec<Token>{
        while !self.is_at_end(){
            self.scan_tokens(start);
            
        }
        self.tokens.push(Token::new(Tokentype::Eof,String::new(),None, self.line));
        self.tokens.clone()
    }
    fn scan_token(&mut self, c: char) {
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                self.add_token(if self.match_next('=') { BANG_EQUAL } else { BANG });
            }
            '=' => {
                self.add_token(if self.match_next('=') { EQUAL_EQUAL } else { EQUAL });
            }
            '<' => {
                self.add_token(if self.match_next('=') { LESS_EQUAL } else { LESS });
            }
            '>' => {
                self.add_token(if self.match_next('=') { GREATER_EQUAL } else { GREATER });
            }
            _ => (),

           
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace.
            }
            '\n' => {
                self.line += 1;
            }
            '"' =>{
                self.string();
            }


            _ =>{
                if c.is_ascii_digit() {
                    self.number();
                }else if c.is_ascii_alphabetic() || c == '_' {
                    identifier();
                }else {
                    runsk_error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
    
        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();
    
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
    
        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number(value));
    }
    
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
    
        if self.is_at_end() {
            self.lox.error(self.line, "Unterminated string.");
            return;
        }
    
        // The closing ".
        self.advance();
    
        // Trim the surrounding quotes.
        let value = self.source[self.start+1..self.current-1].to_string();
        self.add_token(TokenType::String, value);
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1..].chars().next().unwrap()
        }
    }    
    fn is_ascii_digit(c: char)->bool{
        c>='0'&& c<='9'
    }


}
 