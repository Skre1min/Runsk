use std::collections::HashMap;


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
            _ => (),
        }
    }
}
 