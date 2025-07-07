//use std::any::Any;
use std::fmt;
use crate::lexer::{Lexer,TokenType};
#[derive(PartialEq,Debug,Clone)]

pub enum Node{
    Number(i64),
    Variable(char),
    BinaryOp(Box<Node>,Operator,Box<Node>)

}
impl Node{
  pub fn same_type(&self, other:&Node) -> bool { // i think actually that this is useless but i might be wrong
    match (self, other) {
        (Node::Number(_),Node::Number(_)) => true, 
        (Node::BinaryOp(_,op_self,_), Node::BinaryOp(_,op_other,_))=> op_self == op_other,
        (Node::Variable(_),_)=>true,
        _ => false,




    }

    
  }
  pub fn _get_char(&self) -> Option<char>{
    match self{
        Node::Variable(s)=>Some(*s) ,
        
        _ =>None,

    }


  }
 pub fn get_number(&self) -> Option<i64>{
    match self{
        Node::Number(s)=>Some(*s),
        _ =>None,


    }


 }
}
#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken { expected: TokenType, found: TokenType, position: usize },
    
    _LexerError(String), // Placeholder if lexer errors need to be propagated
}
#[derive(Debug,PartialEq,Clone,Copy)]
pub enum Operator{
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,   

}
pub struct Parser {
    lexer: Lexer,
    current_token: TokenType,
    peek_token: TokenType,
}
impl Parser {
    pub fn new(mut lexer: Lexer)-> Parser{
        let first_token = lexer.get_token();
        let second_token = lexer.get_token();
        Parser{
            lexer:lexer,
            current_token:first_token,
            peek_token:second_token,

        }
    }
    fn advance(&mut self){
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.get_token();


    }
 
    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token == token_type
    }
    fn _peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token == token_type
    }
    fn expect_and_advance(&mut self, expected_type: TokenType) -> Result<(), ParserError> {
        if self.current_token_is(expected_type) {
            self.advance();
            Ok(())
        } else {
            
            Err(ParserError::UnexpectedToken {
                expected: expected_type,
                found: self.current_token.clone(),
                position: self.lexer.position, // TODO: Get position from lexer
            })
        }








    
    }
    
    fn parse_factor(&mut self, mut size:i16)->Result<(Node,i16), ParserError>{
    let node = match self.current_token{
        TokenType::Integer(value)=>{
            self.advance();
            //size = size+1;
             println!("You are currently in parse_factor integer,\"{:?}\",size is :\"{}\"",self.current_token,size);
            Ok((Node::Number(value  ),size))

        },
        
        TokenType::Variable(value)=>{
            self.advance();
            //size = size +1;
             println!("You are currently in parse_factor variable,\"{:?}\",size is :\"{}\"",self.current_token,size);
            Ok((Node::Variable(value),size))
        },
        
        TokenType::LParen =>{
            self.expect_and_advance(TokenType::LParen)?;
            //size = size +1;
             println!("You are currently in parse_factor lparen,\"{:?}\",size is :\"{}\"",self.current_token,size);
            let (node,size) = self.parse_term(size)?;
            self.expect_and_advance(TokenType::RParen)?;
             println!("You are currently in parse_factor rparen,\"{:?}\",size is :\"{}\"",self.current_token,size);
            Ok((node,size))
        },
        _ => Err(ParserError::UnexpectedToken {
            expected: TokenType::Integer(42), // 
            found: self.current_token.clone(),
            position: self.lexer.position, // fixed ehehe i think 
        }),
    };
    node
    }
    // for * / and stuff inside () 
    // parser.rs
pub fn parse_equality(&mut self,mut size:i16) -> Result<(Node,i16), ParserError> {
    let (mut lhs,mut size )= self.parse_term(size)?;
    if self.current_token_is(TokenType::Assign){
        
        self.expect_and_advance(TokenType::Assign)?;
        let (rhs,size) = self.parse_equality(size)?; 
        Ok((Node::BinaryOp(Box::new(lhs),Operator::Assign, Box::new(rhs)),size))

    }
    else{

        Ok((lhs,size))
    }

}
fn parse_tuah(&mut self,mut size:i16 ) -> Result<(Node,i16), ParserError> {
    let (mut lhs,mut size) = self.parse_factor(size)?;// in the case "c*(a*b) this is c "
    
    loop {
        match self.current_token {
            
            TokenType::Multiply => {
                self.advance(); // Consume '*'
                size = size+1; // for the example size here is now equal to 1 
                let (rhs,size) = self.parse_factor(size)?; // (rhs,size) = 
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Multiply, Box::new(rhs));
                println!("You are currently in parse_tuah multiply,\"{:?}\",size is :\"{}\"",self.current_token,size);
            },
            TokenType::Divide => {
                self.advance(); // Consume '/'
                size = size +1;
                let (rhs, size) = self.parse_factor(size)?;
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Divide, Box::new(rhs));
                println!("You are currently in parse_tuah divide,\"{:?}\",size is :\"{}\"",self.current_token,size);
            },
            TokenType::Integer(_) | TokenType::Variable(_) | TokenType::LParen => {
                // Implicit multiplication
                size = size +1; 
                let (rhs, size ) = self.parse_factor(size)?;
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Multiply, Box::new(rhs));
                 println!("You are currently in parse_tuah implicit mult,\"{:?}\",size is :\"{}\"",self.current_token,size);
            },
            _ => break,
        }
        
    }
    
    Ok((lhs,size))
}
    pub fn parse_term(&mut self,mut size:i16) -> Result<(Node,i16), ParserError> {
        let (mut lhs,mut size) = self.parse_tuah(size)?; 
    
        while matches!(self.current_token, TokenType::Plus | TokenType::Minus) {
            let operator = match self.current_token {
                TokenType::Plus => Operator::Add,
                TokenType::Minus => Operator::Subtract,
                _ => unreachable!(),
            };
    
            self.advance(); // consume + or -
             println!("You are currently in parse_term bf advance,\"{:?}\",size is :\"{}\"",self.current_token,size);
            size = size+1; 
            let (rhs,size) = self.parse_tuah(size)?;
             println!("You are currently in parse_term before tuah,\"{:?}\",size is :\"{}\"",self.current_token,size);
            lhs = Node::BinaryOp(Box::new(lhs), operator, Box::new(rhs));
             
        
            println!("You are currently in parse_term lhs def,\"{:?}\",size is :\"{}\"",self.current_token,size);
        }
         println!("You are currently in parse_term endloop,\"{:?}\",size is :\"{}\"",self.current_token,size);
        Ok((lhs,size))
    }
    
    
       
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Assign => write!(f, "="), // If you have Eq
        }
    }
}

// Helper to calculate the displayed width of a string.
// For simplicity, assumes ASCII characters (each char is 1 unit wide).
// For full Unicode, this would need to use a unicode-aware width crate.
fn _string_display_width(s: &str) -> usize {
    s.chars().count()
}

// --- Implementation of Display for Node (The Core ASCII Tree Logic) ---
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Number(val) => write!(f, "{}", val),
            Node::Variable(c) => write!(f, "{}", c),
            Node::BinaryOp(lhs, op, rhs) => {
                let op_str = match op {
                    Operator::Add => "+",
                    Operator::Subtract => "-",
                    Operator::Multiply => "*",
                    Operator::Divide => "/",
                    Operator::Assign => "=", // Changed to just "=" for cleaner output
                };
                // Add parentheses for clarity in nested expressions
                // This is a basic approach; for full correctness, you'd need
                // to consider operator precedence and associativity.
                write!(f, "({} {} {})", lhs, op_str, rhs)
            }
        }
    }
}