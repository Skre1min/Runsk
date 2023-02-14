#[derive(Debug)]
struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: i32,
}

impl Token {
    fn new(r#type: TokenType, lexeme: String, literal: Option<Object>, line: i32) -> Token {
        Token { r#type, lexeme, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.r#type, self.lexeme, self.literal.unwrap_or("".to_string()))
    }
}