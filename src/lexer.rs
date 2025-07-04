#[derive(Debug, PartialEq, Eq, Clone,Copy)]
pub enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Eq,     
    Assign, 
    Integer(i64),
    Variable(char), 
    Eof,
    Illegal,
}

pub struct Lexer {
    input: String,
    position: usize,      
    read_position: usize, 
    ch: char,            
}

impl Lexer {
    /// Creates a new Lexer instance and initializes it by reading the first character.
    pub fn new(expression: String) -> Lexer {
        let mut lexer = Lexer {
            input: expression,
            position: 0,
            read_position: 0,
            ch: '\0', // placeholder
        };
        lexer.read_char(); 
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; 
        } else {
            
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.as_bytes().get(self.read_position).map(|&b| b as char)
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
    pub fn read_number(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    pub fn get_token(&mut self) -> TokenType {
        self.skip_whitespace(); 

        let token_type = match self.ch {
            // Single-character operators and delimiters
            '+' => {
                self.read_char();
                TokenType::Plus
            },
            '-' => {
                self.read_char();
                TokenType::Minus
            },
            '*' => {
                self.read_char(); 
                TokenType::Multiply
            },
            '/' => {
                self.read_char(); 
                TokenType::Divide
            },
            '(' => {
                self.read_char();
                TokenType::LParen
            },
            ')' => {
                self.read_char(); 
                TokenType::RParen
            },

           
            '=' => {
                if self.peek_char() == Some('=') {
                    self.read_char(); 
                    self.read_char(); 
                    TokenType::Eq 
                } else {
                    self.read_char(); 
                    TokenType::Assign 
                }
            },

           
            c @ '0'..='9' => {
               
                let num_str = self.read_number();
                TokenType::Integer(num_str.parse::<i64>().expect("Failed to parse number"))
            },

          
            c if c.is_alphabetic() => {
                self.read_char(); 
                TokenType::Variable(c)
            },

            
            '\0' => TokenType::Eof, 

           
            _ => {
                self.read_char(); 
                TokenType::Illegal
            },
        };


        token_type 
    }
}
