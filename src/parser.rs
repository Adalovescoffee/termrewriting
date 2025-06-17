#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Substract, 
    Multiply,
    Divide, 

}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    Number(u64),
    Operation(Box<Term>, Operator, Box<Term>),


}
pub enum TokenType{
    Plus, 
    Minus, 
    Multiply, 
    Divide, 
    LParen, 
    RParen,
    Integer(i64),
    Variable(char),
    Eof,
    Illegal,
    Equal,
}
pub struct Lexer{
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}
impl Lexer {
    fn new(expression: String) ->Lexer{
       let mut lexer =  Lexer{
            input : expression,
            position : 0,
            read_position : 0, 
            ch: '\0',
        
        };
        lexer.read_char();
        lexer 
    }
    // reads character idk why i'm writing these commnets atp the code speaks for itself
    fn read_char(&mut self){
        if self.read_position>= self.input.len(){
            self.ch = '\0';
            self.position = self.read_position;

        }
        else{

            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position; 
        self.read_position+=1;
    }
    fn read_number(&mut self)->String{
        let start_position = self.position;
        
        while self.ch.is_numeric()== true {
            self.read_char();
            

        } 
        self.input[start_position..self.position].to_string()
        

    }
    //skips " "
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
    //gets token 
    fn get_token(&mut self)->TokenType{
        self.skip_whitespace();
        let token = self.ch;
        let token_type = match self.ch {
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Multiply,
            '/' => TokenType::Divide,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen, 
            '=' => TokenType::Equal,
            '\0' =>TokenType::Eof,
            c @ '0'..='9' => {
                let num_str = self.read_number();
                TokenType::Integer(num_str.parse().expect("oops smt went wrong mbmb"))

            }
            c if c.is_alphabetic() => TokenType::Variable(c),
            _ => TokenType::Illegal,
        };
        match token_type {
            TokenType::Plus | TokenType::Minus | TokenType::Multiply | TokenType::Divide |
            TokenType::LParen | TokenType::RParen | TokenType::Equal |
            TokenType::Variable(_) | TokenType::Illegal => self.read_char(),
            
            TokenType::Integer(_) | TokenType::Eof => {}
        }
        token_type
    }
}



//now i implement parser 