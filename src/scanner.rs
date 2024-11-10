pub struct Scanner {}


impl Scanner {
    pub fn new(contents: &str) -> Self {
        println!("{}", contents);
        Self {}
    }

    pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, String> {
        //todo!();

        //return vec!(Token{ name : String::from("Test")})

        return Ok(vec!(Token{ name: String::from("Test")}))
    }
}

#[derive(Debug)]
pub struct Token {
    name: String
}