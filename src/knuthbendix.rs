use core::fmt; 
use std::collections::{HashMap, HashSet};
use crate::lexer::Lexer;
use crate::term::{nodesubst, unification,unifyandfill, Term}; 


// here there'd be a list of possible rules you can choose, apply superposition 
// add new rewrite rules, check for confluency 
// fix commutativity 
// 
#[derive(Clone)]
pub struct Axiom {
    pub lhs:Term,
    pub rhs:Term
}
impl Axiom {
    fn _criticalterms(&self,other:&Axiom)->Option<(Term,Term)>{
        if let Some(substitution) = unifyandfill(&self.lhs, &other.lhs){
            
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
    
    fn normalize(&self) ->Axiom {
        if self.lhs>self.rhs {
            let axiom = self.clone();
            return axiom
        }
        else if self.lhs < self.rhs {
            let axiom =Axiom{lhs:self.rhs.clone(),rhs:self.lhs.clone()}; 
            return axiom
        }
        else {
            return self.clone()
        }
    }
    
    fn criticalpairs(&self,other:&Axiom)->Option<Axiom>{

        if let Some((lhs,rhs)) = self._criticalterms(other){
            if lhs == rhs {
                return None


            }
            else {
                let criticalpair = Axiom{lhs : lhs,rhs :rhs};
                // i need to order it by <r apparently 
                return Some(criticalpair)
            }

        } 
        else {
            
                return None;
        }

    }

}
pub struct Structure {
    pub axioms: HashSet<Axiom>,

}
impl Structure {
    fn builder(){




    }
    /* 
    pub fn knuthbendix(self)->Option<Structure> {
        let mut ruleset:Vec<Axiom> = Vec::new();
        let mut axiomset:Vec<Axiom>= self.axioms.into_iter().collect();
        while let Some(axiom)= axiomset.pop(){
            for rest in axiomset{
                
                if let Some(newaxiom) = axiom.criticalpairs(&rest){
                   ruleset.push(newaxiom);

                }
                

            }




        }
        return None

    }

    */
    
}