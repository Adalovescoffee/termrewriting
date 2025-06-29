use std::any::Any;
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
  pub fn get_char(&self) -> Option<char>{
    match(self){
        Node::Variable(s)=>Some(*s) ,
        
        _ =>None,

    }


  }
 pub fn get_number(&self) -> Option<i64>{
    match(self){
        Node::Number(s)=>Some(*s),
        _ =>None,


    }


 }
}
#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken { expected: TokenType, found: TokenType, position: usize },
    ExpectedNumber { found: TokenType, position: usize },
    ExpectedVariable { found: TokenType, position: usize },
    SyntaxError(String),
    LexerError(String), // Placeholder if lexer errors need to be propagated
}
#[derive(Debug,PartialEq,Clone)]
pub enum Operator{
    Add,
    Substract,
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
            let node = self.parse_term()?;
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
    // parser.rs
pub fn parse_equality(&mut self) -> Result<Node, ParserError> {
    let mut lhs = self.parse_term()?;
    if self.current_token_is(TokenType::Assign){
        
        self.expect_and_advance(TokenType::Assign)?;
        let rhs = self.parse_equality()?; 
        Ok(Node::BinaryOp(Box::new(lhs),Operator::Assign, Box::new(rhs)))

    }
    else{

        Ok(lhs)
    }

}
fn parse_tuah(&mut self) -> Result<Node, ParserError> {
    let mut lhs = self.parse_factor()?;
    
    loop {
        match self.current_token {
            
            TokenType::Multiply => {
                self.advance(); // Consume '*'
            let rhs = self.parse_factor()?;
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Multiply, Box::new(rhs));
            },
            TokenType::Divide => {
                self.advance(); // Consume '/'
                let rhs = self.parse_factor()?;
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Divide, Box::new(rhs));
            },
            TokenType::Integer(_) | TokenType::Variable(_) | TokenType::LParen => {
                // Implicit multiplication
                let rhs = self.parse_factor()?;
                lhs = Node::BinaryOp(Box::new(lhs), Operator::Multiply, Box::new(rhs));
            },
            _ => break,
        }
    }
    
    Ok(lhs)
}
    pub fn parse_term(&mut self) -> Result<Node, ParserError> {
        let mut lhs = self.parse_tuah()?; 
    
        while matches!(self.current_token, TokenType::Plus | TokenType::Minus) {
            let operator = match self.current_token {
                TokenType::Plus => Operator::Add,
                TokenType::Minus => Operator::Substract,
                _ => unreachable!(),
            };
    
            self.advance(); // consume + or -
            let rhs = self.parse_tuah()?;
            lhs = Node::BinaryOp(Box::new(lhs), operator, Box::new(rhs));
        }
    
        Ok(lhs)
    }
    
    
       
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Substract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Assign => write!(f, "="), // If you have Eq
        }
    }
}

// Helper to calculate the displayed width of a string.
// For simplicity, assumes ASCII characters (each char is 1 unit wide).
// For full Unicode, this would need to use a unicode-aware width crate.
fn string_display_width(s: &str) -> usize {
    s.chars().count()
}

// --- Implementation of Display for Node (The Core ASCII Tree Logic) ---
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Recursive helper to build the ASCII tree lines.
        // Returns a vector of strings, where each string is a line of the ASCII tree.
        fn build_ascii_lines(node: &Node) -> Vec<String> {
            match node {
                Node::Number(val) => vec![val.to_string()],
                Node::Variable(c) => vec![c.to_string()],
                Node::BinaryOp(left, op, right) => {
                    let op_str = op.to_string(); // Get the operator string (e.g., "+", "*")
                    let op_width = string_display_width(&op_str);

                    let left_lines = build_ascii_lines(left);
                    let right_lines = build_ascii_lines(right);

                    let left_root_str = &left_lines[0];
                    let right_root_str = &right_lines[0];

                    let left_root_width = string_display_width(left_root_str);
                    let right_root_width = string_display_width(right_root_str);

                    // Determine the padding and total width for the current level
                    // This is a simplified alignment.
                    let gap_between_children = 3; // Minimum spaces between children roots
                 //   let branch_char_width = 1; // Width of '/' or '\'

                    // Calculate the width needed for the two children plus the gap
                    let children_total_width = left_root_width + right_root_width + gap_between_children;

                    // The width for the operator line is the max of its own width or the children's combined width
                    let current_level_width = op_width.max(children_total_width);

                    // Calculate padding for the operator to center it
                    let op_padding_left = (current_level_width - op_width) / 2;
                    let op_padding_right = current_level_width - op_width - op_padding_left;

                    let mut result_lines = Vec::new();

                    // Line 1: The Operator
                    result_lines.push(format!("{}{}{}", " ".repeat(op_padding_left), op_str, " ".repeat(op_padding_right)));

                    // Line 2: The Branches
                    // This is the most complex part for alignment.
                    // The idea is to place '/' roughly above left_root_str and '\' above right_root_str
                    let left_branch_start_pos = op_padding_left + op_width / 2;
                 //   let right_branch_start_pos = op_padding_left + op_width / 2 + 1; // +1 to offset '\'

                    let mut branch_line = String::new();
                    // Pad until the left branch character
                    branch_line.push_str(&" ".repeat(left_branch_start_pos.saturating_sub(1)));
                    branch_line.push('/');
                    // Pad between branches
                    branch_line.push_str(&" ".repeat(current_level_width.saturating_sub(branch_line.len() + 1))); // Adjust spacing
                    branch_line.push('\\');
                    result_lines.push(branch_line);


                    // Lines 3 onwards: Combined children subtrees
                    let max_child_lines = left_lines.len().max(right_lines.len());
                    for i in 0..max_child_lines {
                        let left_line = left_lines.get(i).unwrap_or(&"".to_string()).clone();
                        let right_line = right_lines.get(i).unwrap_or(&"".to_string()).clone();

                        let left_line_width = string_display_width(&left_line);
                      //  let right_line_width = string_display_width(&right_line);

                        // Calculate padding to align children under their branches
                        // This is a heuristic and might need fine-tuning for complex cases.
                        let padding_to_center_left = (left_root_width.saturating_sub(left_line_width) / 2);
                       // let padding_to_center_right = (right_root_width.saturating_sub(right_line_width) / 2);

                        let actual_left_offset = op_padding_left; // Rough alignment
                        let actual_right_offset = op_padding_left + left_root_width + gap_between_children;

                        let combined_line = format!(
                            "{}{}{}{}",
                            " ".repeat(actual_left_offset + padding_to_center_left),
                            left_line,
                            " ".repeat(actual_right_offset.saturating_sub(actual_left_offset + left_line_width)),
                            right_line
                        );
                        result_lines.push(combined_line.trim_end().to_string());
                    }

                    result_lines
                }
            }
        }

        // Call the recursive builder and print all the generated lines
        let lines = build_ascii_lines(self);
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
