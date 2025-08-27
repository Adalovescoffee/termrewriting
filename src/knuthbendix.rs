use core::fmt; 
use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::term::{nodesubst, unification, Term}; 


// here there'd be a list of possible rules you can choose, apply superposition 
// add new rewrite rules, check for confluency 
// fix commutativity 
// 

pub struct Axiom {
    pub lhs:Term,
    pub rhs:Term
}
impl Axiom {
    fn criticalterms(&self,other:&Axiom)->Option<(Term,Term)>{
        if let Some(substitution) = unification(&self.lhs, &other.lhs){
            
            let (axiom1lhs,size1) = nodesubst(&self.lhs.term, &substitution);
            let axiom1:Term = Term{term:axiom1lhs,size:size1};
            let (axiom2lhs ,size2)= nodesubst(&other.lhs.term, &substitution);
            let axiom2:Term = Term{term:axiom2lhs,size:size2};
            // for now i'll do this but later i'll fix it to be clean 
            let selflaw = ((&self.lhs.term,self.lhs.size),(&self.rhs.term,self.rhs.size));
            let otherlaw = ((&other.lhs.term,other.lhs.size),(&other.rhs.term,other.rhs.size));
            
            let axiom1rhs = axiom1.rewriteby(selflaw);
            let axiom2rhs = axiom2.rewriteby(otherlaw);
            
            
            return Some((axiom1rhs,axiom2rhs)); 
        }
        


        else{
            return None


        }
    }


}