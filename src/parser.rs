//use std::any::Any;
use std::fmt;
use crate::lexer::{Lexer,TokenType};
#[derive(PartialEq,Debug,Clone,Hash)]
/// a node is either a number a variable or a binary op of itself 
pub enum Node{
    Number(i64),
    Variable(char),
    BinaryOp(Box<Node>,Operator,Box<Node>),
    UnaryOp(Operator,Box<Node>)
}


impl Node{

  /// check if two nodes are of the same types 
  pub fn same_type(&self, other:&Node) -> bool { // i think actually that this is useless but i might be wrong
    match (self, other) {
        (Node::Number(_),Node::Number(_)) => true, 
        (Node::BinaryOp(_,op_self,_), Node::BinaryOp(_,op_other,_))=> op_self == op_other,
        (Node::UnaryOp(op_self,_ ),Node::UnaryOp(op_other,_ ))=> op_self == op_other,
        (Node::Variable(_),_)=>true,
        _ => false,




    }

    
  }
  pub fn size(self)->i16{
            match self {
                Node::Number(_) => 0,
                Node::Variable(_) =>0, 
                Node::BinaryOp(lhs,_ ,rhs ) =>{
                    1 + lhs.size() + rhs.size()


                }
                Node::UnaryOp(_,rhs)=> {
                    1 + rhs.size()}// idk if it should add 1 or not we'll see  

            }
        }
  /// previous get_char from back when i didn't know rust had cool patternmatching
  pub fn _get_char(&self) -> Option<char>{
    match self{
        Node::Variable(s)=>Some(*s) ,
        
        _ =>None,

    }


  }
 /// gets the value from a node::number()
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
#[derive(Debug,PartialEq,Clone,Copy,Hash)]
/// operator can either be '+','-','*','/','=', for the sake of group theory, ^-1 needs to be implemented eventually 
pub enum Operator{
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,   

}
/// parser struct : lexer , the current token its in, and the token it's peeking
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
                position: self.lexer.position, // TODO: Get position from lexer
            })
        }








    
    }
    
    fn parse_factor(&mut self)->Result<(Node,i16), ParserError>{
    let node = match self.current_token{
        TokenType::Integer(value)=>{
            self.advance();
            //opsnumber = opsnumber+1;
            // println!("You are currently in parse_factor integer,\"{:?}\",opsnumber is :\"{}\"",self.current_token,0);
            Ok((Node::Number(value  ),0))

        },
        
        TokenType::Variable(value)=>{
            self.advance();
            //opsnumber = opsnumber +1;
             //println!("You are currently in parse_factor variable,\"{:?}\",opsnumber is :\"{}\"",self.current_token,0);
            Ok((Node::Variable(value),0))
        },
        //unary operatory minus not normal one!! 
        TokenType::Minus => {
            self.advance(); //advance the token 
            //println!("{:?}",self.current_token);
            if let TokenType::Integer(value) = self.current_token{
                self.expect_and_advance(TokenType::Integer(value))?;
                //println!("{}",-value);
               
                Ok((Node::UnaryOp(Operator::Subtract,Box::new(Node::Number(value))),1))// wtf do i consider this as an op wtf :[ 

            }
            else if let TokenType::Minus = self.current_token{
                let mut counter = 0;
                while self.peek_token == TokenType::Minus{
                    self.expect_and_advance(TokenType::Minus)?;
                    counter = counter + 1; 

                
                }
                let countercopy = counter;
                let (node,opsnumber) = self.parse_factor()?;
                let mut last = Node::UnaryOp(Operator::Subtract,Box::new(node)); 
                while counter!= 0 {
                    last = Node::UnaryOp(Operator::Subtract,Box::new(last));
                    counter = counter - 1; 

                }
                Ok((last,opsnumber + countercopy + 1))

            }
            
            //add option for variable, then for left parenthesis? -a+b is variable -(a+b) is
            else if let TokenType::Variable(value ) =self.current_token{
                self.expect_and_advance(TokenType::Variable(value))?;
                Ok((Node::UnaryOp(Operator::Subtract,Box::new(Node::Variable((value)))),1))
                


            }
            else if let TokenType::LParen = self.current_token{
                self.expect_and_advance(TokenType::LParen)?;
                let(node, opsnumber) = self.parse_term()?;
                self.expect_and_advance(TokenType::RParen)?;
                Ok((Node::UnaryOp(Operator::Subtract,Box::new(node)),opsnumber + 1))
                

            }
            else {
                 Err(ParserError::UnexpectedToken {
            expected: TokenType::Integer(40), // 
            found: self.current_token.clone(),
            position: self.lexer.position, // fixed ehehe i think 
        })

                

            }
        }, 
        TokenType::LParen =>{
            self.expect_and_advance(TokenType::LParen)?;
            //opsnumber = opsnumber +1;
             //println!("You are currently in parse_factor lparen,\"{:?}\",opsnumber is :\"{}\"",self.current_token,0);
            let (node,opsnumber) = self.parse_term()?;
            self.expect_and_advance(TokenType::RParen)?;
             //println!("You are currently in parse_factor rparen,\"{:?}\",opsnumber is :\"{}\"",self.current_token,0);
            Ok((node,opsnumber))
        },
        _ => Err(ParserError::UnexpectedToken {
            expected: TokenType::Integer(42), // 
            found: self.current_token.clone(),
            position: self.lexer.position, // fixed ehehe i think 
        }),
    };
    //println!("{:?}",node);
    node
    }
    // for * / and stuff inside () 
  

pub fn parse_equality(&mut self) -> Result<((Node,i16),(Node,i16)),ParserError> {
    let (mut lhs,mut lhsops )= self.parse_term()?;
    if self.current_token_is(TokenType::Assign){
        
        self.expect_and_advance(TokenType::Assign)?;
        let (rhs,rhsops) = self.parse_term()?; 
        //opsnumber = opsnumber + rhsops;
        Ok(((lhs,lhsops),(rhs,rhsops)))

    }
    else{

        Ok(((lhs.clone(),lhsops),(lhs,lhsops))) // cloning here :c 
    }
    
}   

fn parse_tuah(&mut self ) -> Result<(Node,i16), ParserError> {
    let (mut lhs,mut opsnumber) = self.parse_factor()?;// in the case "c*(a*b) this is c "
    //println!("{}",lhs);
    loop {
        match self.current_token {
            
            TokenType::Multiply => {
                self.advance(); // Consume '*'
                opsnumber = opsnumber   +1; // for the example opsnumber here is now equal to 1 
                let (rhs,rhsops) = self.parse_factor()?; // (rhs,opsnumber) = 
                opsnumber = opsnumber + rhsops; 
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Multiply, Box::new(rhs));
                //println!("You are currently in parse_tuah multiply,\"{:?}\",opsnumber is :\"{}\"",self.current_token,opsnumber);
            },
            TokenType::Divide => {
                self.advance(); // Consume '/'
                opsnumber = opsnumber +1;
                let (rhs, rhsops) = self.parse_factor()?;
                opsnumber = opsnumber + rhsops; 
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Divide, Box::new(rhs));
                //println!("You are currently in parse_tuah divide,\"{:?}\",opsnumber is :\"{}\"",self.current_token,opsnumber);
            },
            TokenType::Integer(_) | TokenType::Variable(_) | TokenType::LParen => {
                // Implicit multiplication
                opsnumber = opsnumber +1; 
                let (rhs, rhsops ) = self.parse_factor()?;
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Multiply, Box::new(rhs));
                opsnumber = opsnumber + rhsops; 
                 //println!("You are currently in parse_tuah implicit mult,\"{:?}\",opsnumber is :\"{}\"",self.current_token,opsnumber);
            },
            _ => break,
        }
        
    }
    
    Ok((lhs,opsnumber))
}
    pub fn parse_term(&mut self) -> Result<(Node,i16), ParserError> {
        let (mut lhs,mut opsnumber) = self.parse_tuah()?; 
      // println!("what's going on term,\"{:?}\", opsnumber : \"{}\"",self.current_token,opsnumber);
        while matches!(self.current_token, TokenType::Plus | TokenType::Minus) {
            let operator = match self.current_token {
                TokenType::Plus => Operator::Add,
                TokenType::Minus => Operator::Subtract,
                _ => unreachable!(),
            };
    
            self.advance(); // consume + or -
           //  println!("You are currently in parse_term bf advance,\"{:?}\",opsnumber is :\"{}\"",self.current_token,opsnumber);
            opsnumber = opsnumber+1; 
            let (rhs,rhsops) = self.parse_tuah()?;
            opsnumber = opsnumber + rhsops;
            
           //  println!("You are currently in parse_term before tuah,\"{:?}\",opsnumber is :\"{}\"",self.current_token,opsnumber);
            // i'm thinking here i should add a conditional on when lhs is empty? 
            if operator == Operator::Subtract{
                lhs = Node::BinaryOp(Box::new(lhs),Operator::Add,Box::new(Node::UnaryOp(Operator::Subtract,Box::new(rhs))));

            }
            else {
            lhs = Node::BinaryOp(Box::new(lhs), operator, Box::new(rhs));
            }
        
          //  println!("You are currently in parse_term lhs def,\"{:?}\",opsnumber is :\"{}\"",self.current_token,opsnumber);
        }
        // println!("You are currently in parse_term endloop,\"{:?}\",opsnumber is :\"{}\"",self.current_token,opsnumber);
        Ok((lhs,opsnumber))
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
            Node::UnaryOp(op,rhs) => {
                write!(f,"(- {})",rhs)

            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testsimpleunaryopnumber(){
        let number = "-2".to_string();
        let lexer = Lexer::new(number);
        let mut parser = Parser::new(lexer);
        let node = match parser.parse_equality(){
        Ok(node) =>{node 
            


        }
        Err(e)=>{
        eprintln!("Error parsing term \"{}\": {:?}", "-2", e);
        ((Node::Variable('f'),0),(Node::Variable('f'),0))

    }};
     println!("{}",node.0.0);
     assert_eq!(node.0.0, Node::Number(-2)) 
     
    }

    #[test]
    fn testsimpleunaryvariable(){
        let variable = "-a".to_string();
        let lexer = Lexer::new(variable);
        let mut parser = Parser::new(lexer);
        let node = match parser.parse_equality(){
            Ok(node) => {
                node
            }
            Err(e) => {
            eprintln!("Error parsing term \"{}\";{:?}","-2",e);
            ((Node::Variable('f'),0),(Node::Variable('f'),0))


            }};
            println!("{}",node.0.0);
            assert_eq!(node.0.0,Node::UnaryOp(Operator::Subtract,Box::new(Node::Variable('a'))))
    }
    #[test]
    fn testsimplenegativeparenthesis(){
        let exp  = "-(a+b)".to_string();
        let lexer = Lexer::new(exp);
        let mut parser = Parser::new(lexer);
        let node = match parser.parse_equality(){
        Ok(node) => {
            node

        }

        Err(e) => {
            eprintln!("Error parsing term \"{}\";{:?}","-2",e);
            ((Node::Variable('f'),0),(Node::Variable('f'),0))


            }};
        println!("{}",node.0.0);

    }
    #[test]
    fn testnewsimpleminusimplementation(){
    let exp  = "-a - b".to_string();
        let lexer = Lexer::new(exp);
        let mut parser = Parser::new(lexer);
        let node = match parser.parse_equality(){
        Ok(node) => {
            node

        }

        Err(e) => {
            eprintln!("Error parsing term \"{}\";{:?}","-2",e);
            ((Node::Variable('f'),0),(Node::Variable('f'),0))


            }};
        println!("{}",node.0.0);
        



    }
    #[test]
    fn testhardminussimplementation(){
    let exp  = "---a".to_string();
        let lexer = Lexer::new(exp);
        let mut parser = Parser::new(lexer);
        let node = match parser.parse_equality(){
        Ok(node) => {
            node

        }

        Err(e) => {
            eprintln!("Error parsing term \"{}\";{:?}","--a",e);
            ((Node::Variable('f'),0),(Node::Variable('f'),0))


            }};
        println!("{:?}",(node.0.0,node.0.1));




    }
    
}