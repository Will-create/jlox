pub struct Scanner {}


impl Scanner {
    pub fn new(contents: &str) -> Self {
        println!("{}", contents);
        Self {}
    }

    pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, String> {
        //todo!();

        //return vec!(Token{ name : String::from("Test")})

        return Ok(vec!())
    }
}

#[derive(Debug)]
pub enum TokenType {
    // Single-char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Start,

    // One or Two chars
    Bang,
    BangEqual,
    EQUAL,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Litterals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof  
}

#[derive(Debug)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String)
}
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: LitteralValue,
    line: u64
}

impl Token {
    pub fn new (token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line: u64) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line
        }

    }
}

