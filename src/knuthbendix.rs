use core::fmt; 
use std::collections::{HashMap, HashSet,VecDeque};
use std::hash::Hash;
use crate::lexer::Lexer;
use crate::term::{nodesubst, unification,unifyandfill, Term,from_str}; 
use crate::parser::{Node};

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
    
    pub fn order(&self) ->Axiom {
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
    
   pub fn knuth_bendix(mut self) -> Structure {
    let mut ruleset: HashSet<Axiom> = HashSet::new();
    let mut axioms_to_process: VecDeque<Axiom> = self.axioms.into_iter().collect();

    while let Some(axiom) = axioms_to_process.pop_front() {
        // Here, your normalize function is used to normalize both sides.
        // It should take a term and the ruleset to perform the reduction.
        let normalized = normalize(axiom.clone(), &ruleset);
        

        // Check for redundancy after normalization.
        if normalized.lhs== normalized.rhs {
            continue;
        }

        // Orient the axiom into a rule.
        let new_rule = normalized.order();
        // Generate critical pairs between the new rule and all existing rules.
        for existing_rule in ruleset.iter() {
            if let Some(critical_pair) = existing_rule.criticalpairs(&new_rule) {
                // Add new pairs to the queue for processing.
                axioms_to_process.push_back(critical_pair);
            }
        }

        // Add the new rule to the ruleset.
        ruleset.insert(new_rule);
    }

    // The final set of rules.
    Structure { axioms: ruleset }
}

    
    
}
pub fn normalize(mut axiom: Axiom, axiom_set: &HashSet<Axiom>) -> Axiom {
    let mut lhs = axiom.lhs;
    let mut rhs = axiom.rhs;
    loop {
        let mut changed_lhs = false; 
        let mut changed_rhs = false;
        for axiom in axiom_set {
            
            let rewritten_lhs = lhs.rewriteby(((&axiom.lhs.term,axiom.lhs.size),(&axiom.rhs.term,axiom.rhs.size)));
            let rewritten_rhs = rhs.rewriteby(((&axiom.lhs.term,axiom.lhs.size),(&axiom.rhs.term,axiom.rhs.size)));
            if rewritten_rhs != rhs {
                //println!("rhs {} => rewritten rhs {}",rhs.term,rewritten_rhs.term);
                rhs = rewritten_rhs;
                changed_rhs = true;
                
            }
            if rewritten_lhs != lhs {
                //println!("lhs {} => rewritten lhs {}",lhs.term,rewritten_lhs.term);
                lhs = rewritten_lhs;
                changed_lhs = true;
               
            }
            
            if changed_lhs || changed_rhs == true{


                break;
            }
        }
        if !changed_lhs && !changed_rhs{
            return Axiom{lhs:lhs,rhs}; //  this works 
        }
    }
    
}
#[cfg (test)]
mod tests{
use std::vec;

use crate::knuthbendix;

use super::*;
#[test]
fn leftsidenormalization(){

let axiom = Axiom{lhs:from_str("(--f + -f ) + f").unwrap(),rhs:from_str("0 + f").unwrap()};
let mut axiomset:HashSet<Axiom> = HashSet::from([
            Axiom{lhs:from_str("a+0").unwrap(),rhs:from_str("a").unwrap()},
            Axiom{lhs:from_str("0+b").unwrap(),rhs:from_str("b").unwrap()},
            Axiom{lhs:from_str("-c+c").unwrap(),rhs:from_str("0").unwrap()},
            Axiom{lhs:from_str("x+(y+z)").unwrap(),rhs:from_str("(x+y)+z").unwrap()},
            ]);

println!("{:?}",normalize(axiom, &axiomset));
}
#[test]
fn testforrewrite(){
let law = from_str("(a+0) =a").unwrap();
let expression = from_str("(a+0)").unwrap();
if let Some(((lhs,lhs_size),(rhs,rhs_size))) = law.getequality(){
    
println!("term has turned into {:?}",expression.rewriteby(((&lhs,lhs_size),(&rhs,rhs_size))));}
else {

    println!("{:?}",law);   


}
}
#[test]
fn testorderedlaws(){
    let structure = Structure{
        axioms: HashSet::from([
            Axiom{lhs:from_str("a+0").unwrap(),rhs:from_str("a").unwrap()},
            Axiom{lhs:from_str("0+b").unwrap(),rhs:from_str("b").unwrap()},
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
fn grouptheoryfirsttrymaybeihope(){
    let structure = Structure{
        axioms: HashSet::from([
            Axiom{lhs:from_str("a+0").unwrap(),rhs:from_str("a").unwrap()},
            Axiom{lhs:from_str("0+ b").unwrap(),rhs:from_str("b").unwrap()},
            Axiom{lhs:from_str("-c+c").unwrap(),rhs:from_str("0").unwrap()},
            Axiom{lhs:from_str("(x+y)+z").unwrap(),rhs:from_str("x+(y+z)").unwrap()},
            ]),
        };
    let axioms = structure.knuth_bendix().axioms;
    for axiom in axioms {
        println!("{}={}",axiom.lhs.term,axiom.rhs.term);


    }




}
}
