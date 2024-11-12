pub struct Scanner {}


impl Scanner {
    pub fn new(contents: &str) -> Self {
        println!("{}", contents);
        Self {}
    }

    pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, String> {
        //todo!();

        //return vec!(Token{ name : String::from("Test")})

        return Ok(vec!(Token::LEFT_BRACE, Token::RIGHT_BRACE))
    }
}

#[derive(Debug)]
pub enum Token {
    // Single-char tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    START,

    // One or Two chars
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Litterals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    nil,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF   
}