use core::fmt; 
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::lexer::Lexer;
use crate::term::{nodesubst, unification,unifyandfill, Term,from_str}; 


// here there'd be a list of possible rules you can choose, apply superposition 
// add new rewrite rules, check for confluency 
// fix commutativity 
// 
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
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
    
    fn order(&self) ->Axiom {
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
#[derive(Debug)]
pub struct Structure {
    pub axioms: HashSet<Axiom>,

}
impl Structure {
    
    pub fn builder(){




    }
    pub fn ordered(self)->Structure{
        let mut orderedhashset:HashSet<Axiom> = HashSet::new();
        for axiom in self.axioms{
            let ordered_axiom = axiom.order();
            orderedhashset.insert(ordered_axiom);

        }
        return Structure{axioms:orderedhashset}
    }
    
    pub fn knuthbendix(self)->Option<Structure> {
        let mut ruleset:HashSet<Axiom> = HashSet::new();
        let mut axiomset:HashSet<Axiom>= self.axioms.clone();
        let mut axiomsetvec:Vec<Axiom> = self.axioms.into_iter().collect();
        while !axiomset.is_empty(){
            let mut axiom = axiomsetvec.pop().unwrap();
            axiomset.retain(|f| *f!=axiom); // hopefully this only retains everything but axiom
            axiom = axiom.order();
            if axiom.lhs!= axiom.rhs{



            }





        }
        return None
    }

    
    
}
pub fn normalize(){



}
#[cfg (test)]
mod tests{
use std::vec;

use crate::knuthbendix;

use super::*;
#[test]
fn testorderedlaws(){
    let structure = Structure{
        axioms: HashSet::from([
            Axiom{lhs:from_str("a+0").unwrap(),rhs:from_str("a").unwrap()},
            Axiom{lhs:from_str("b").unwrap(),rhs:from_str("0+b").unwrap()},
            Axiom{lhs:from_str("-c+c").unwrap(),rhs:from_str("0").unwrap()},
            Axiom{lhs:from_str("(x+y)+z").unwrap(),rhs:from_str("x+(y+z)").unwrap()},
            ]),
        };
        let ordered_structure = structure.ordered();
        for axiom in ordered_structure.axioms {
        println!("{}={}",axiom.lhs.term,axiom.rhs.term);


    }


}
#[test]
fn grouptheoryfirsttrymaybeihope(){/* 
    let structure = Structure{
        axioms: vec![
        Axiom{lhs:from_str("a+0").unwrap(),rhs:from_str("a+0").unwrap()},
        Axiom{lhs:from_str("0+b").unwrap(),rhs:from_str("b+0").unwrap()},
        Axiom{lhs:from_str("-c+c").unwrap(),rhs:from_str("0").unwrap()},
        Axiom{lhs:from_str("(x+y)+z").unwrap(),rhs:from_str("x+(y+z)").unwrap()}




        ]
    };
    let axioms = structure.knuthbendix().axioms;
    for axiom in axioms {
        println!("{}={}",axiom.lhs.term,axiom.rhs.term);


    }



*/
}
}
