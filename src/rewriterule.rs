use std::collections::HashMap;
use std::vec;
use crate::lexer::Lexer;
use crate::parser::{Parser,Node,ParserError};

pub struct Rewrite {
    term:Node,
    expression:Node,


}
impl Rewrite{     

pub fn rewrite(){

}
pub fn canmatch(&self,other:&Node/* ,relations:HashMap<char,Node>*/)-> bool{ // here self is the lhs btw 
    //let mut elhs = &self.term.clone();
    if self.term.same_type(other) == false {

        return false
    }
    match self.term {
    Node::Number(a) => a == other.get_number().unwrap(),
    Node::Variable(a) => self.term.same_type(other),
    Node::BinaryOp(plhs,op_self,prhs) =>{
    if let Node::BinaryOp(tlhs,op_self,trhs)= other{
        canmatch(&self,tlhs) && canmatch(&self,trhs)

     

    }


    
    else {
        false
        //kys :D 
    }
    



    }
    _ => false,
}


}

}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn justryingoutstuff(){
        let a = Node::Number(32);
        let b = Node::Variable('a');
        let mut m:HashMap<char,Node> = HashMap::new();
        m.insert(b.get_char().unwrap(),a);
        println!("{:?}",m)
    }


}