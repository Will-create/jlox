#[derive(Debug, Clone)]
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
    Star,

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
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: usize
}


impl Scanner { 
    pub fn new(source: &str) -> Self {
        
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
       while !self.is_at_end() {
            self.start =  self.current;
            self.scan_token()?;
        
       }

       self.tokens.push(Token {token_type: TokenType::Eof, lexeme: "".to_string(), literal: None, line: self.line  });

        return Ok(self.tokens);
    }  


    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len() as u64
    }

    fn scan_token(self: &mut Self) -> Result<Token, String> {
       let c = self.advance();

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
        _ => return Err(format!("Unrecognized char: {}", c))
       }

       todo!();
    }


    fn advance(self: &mut Self) -> char {
      let c = self.source[self.current];
      self.current += 1;
      c
    }

    fn add_token(self: &mut Self, token_type: TokenType) {

    }
}



impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
     }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String)
}
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line: usize 
}

impl Token {
    pub fn new (token_type: TokenType, lexeme: String, literal: LiteralValue, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal: Some(literal),
            line
        } 

    }

    pub fn to_string(self: &Self) -> String {
         format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}


/*
    var test = 0.1;
    var text2 = test + 0.2;
    
*/
