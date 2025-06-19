use std::any::Any;

use crate::lexer::{Lexer,TokenType};
#[derive(Debug,PartialEq)]
pub enum Node{
    Number(i64),
    Variable(char),
    BinaryOp(Box<Node>,Operator,Box<Node>)

}
#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken { expected: TokenType, found: TokenType, position: usize },
    ExpectedNumber { found: TokenType, position: usize },
    ExpectedVariable { found: TokenType, position: usize },
    SyntaxError(String),
    LexerError(String), // Placeholder if lexer errors need to be propagated
}
#[derive(Debug,PartialEq)]
pub enum Operator{
    Add,
    Substract,
    Multiply,
    Divide,

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
    fn peek_token_is(&self, token_type: TokenType) -> bool {
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
                position: 0, // TODO: Get position from lexer
            })
        }








    
    }
    
    fn parse_factor(&mut self)->Result<Node, ParserError>{
    let node = match self.current_token{
        TokenType::Integer(value)=>{
            self.advance();
            Ok(Node::Number(value  ))

        },
        TokenType::Variable(value)=>{
            self.advance();
            Ok(Node::Variable(value))
        },
        TokenType::LParen =>{
            self.expect_and_advance(TokenType::LParen)?;
            let node = self.parse_tuah()?;
            self.expect_and_advance(TokenType::RParen)?;
            Ok(node)
        },
        _ => Err(ParserError::UnexpectedToken {
            expected: TokenType::Integer(42), // 
            found: self.current_token.clone(),
            position: 0, // to be fixed
        }),
    };
    node
    }
    // for * / and stuff inside () 
    fn parse_tuah(&mut self)-> Result<Node, ParserError>{
        let mut lhs = self.parse_factor()?;
        while self.peek_token_is(TokenType::Multiply) ||
      self.peek_token_is(TokenType::Divide)   ||
      matches!(self.peek_token, TokenType::Integer(_)) ||
      matches!(self.peek_token, TokenType::Variable(_)) ||
      self.peek_token_is(TokenType::LParen){
        if self.peek_token_is(TokenType::Multiply){
            self.advance();
            self.advance();
            let rhs = self.parse_factor()?;
            lhs = Node::BinaryOp(Box::new(lhs), Operator::Multiply,Box::new(rhs));

        }
       else if self.peek_token_is(TokenType::Divide){
            self.advance();
            self.advance();
            let rhs = self.parse_factor()?;
            lhs = Node::BinaryOp(Box::new(lhs), Operator::Divide, Box::new(rhs));

        }
        else if matches!(self.peek_token, TokenType::Integer(_))|| self.peek_token_is(TokenType::LParen)||matches!(self.peek_token, TokenType::Variable(_)){
            self.advance();
            let rhs = self.parse_factor()?;
            lhs = Node::BinaryOp(Box::new(lhs), Operator::Multiply, Box::new(rhs));

        }
        
      }
      Ok(lhs)

        
    }
    pub fn parse_term(&mut self)-> Result<Node, ParserError> {
        
        let mut lhs = self.parse_tuah()?; 

        
        loop {
            let operator: Operator;

            match self.peek_token {
                TokenType::Plus => {
                    operator = Operator::Add;
                },
                TokenType::Minus => {
                    operator = Operator::Substract;
                },
                // If it's not '+' or '-', tis over
                _ => break,
            }
            match operator {
                Operator::Add => self.expect_and_advance(TokenType::Plus)?,
                Operator::Substract => self.expect_and_advance(TokenType::Minus)?,
                _ => unreachable!("Should only be Add or Subtract here"), 
            }

           
            let rhs = self.parse_tuah()?; 

            lhs = Node::BinaryOp(Box::new(lhs), operator, Box::new(rhs));
        }

       
        Ok(lhs)
    }
}