use core::fmt;
use std::collections::HashMap;
use std::cmp::Ordering;

//use std::vec;
//use crate::lexer::Lexer;
use crate::parser::{ Node, Operator};
//#[derive(PartialEq, Eq)]

pub struct Term {
    pub term:Node,
    pub size:i16,
    


}
impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.complexitysize() == other.complexitysize()
    }
}
impl PartialOrd for Term {
    fn partial_cmp (&self, other:&Self) ->Option<Ordering> {
        let size1 = self.size; 
        let size2 = other.size; 
        if size1> size2 {

           return  Some(Ordering::Greater)
        }
        if size1 < size2 {


         return Some(Ordering::Less)
        }
        else {
            let sizec1:i16 = self.complexitysize();
            let sizec2:i16 = other.complexitysize();
            if sizec1>sizec2 {

                return Some(Ordering::Greater)

            }
            if sizec1<sizec2 {

                return Some(Ordering::Less)

            }
            else{
                return Some(Ordering::Equal)
            
            }
        
        }


    }



}
/*impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    
        
    }


        
    



}*/


impl Term{     
//gives the number of nodes on the left 
// example : 
//                +
//              /  \
//             *    2
//            / \ 
//           a  b
// would have size 3 :D 
fn complexitysize(&self)-> i16{// this is when number of operations is the same 
   
    fn rec(node:&Node)->i16{
            match node {
                Node::Number(_) => 0,
                Node::Variable(_) =>0, 
                Node::BinaryOp(lhs,_ ,_rhs ) =>{
                    1 + rec(lhs) 


                }


            }

    

    }

    return rec(&self.term) 
}
// okay here is when things become interesting 
// ok so we decided self has term + size law has to have some sort of size value inbedded only issue is that size
// maybe it should be law :((lhs,size),(rhs,size)) 
//time to change
pub fn rewriteby(&self, law:((&Node,i16),(&Node,i16)))-> Term{
    /*let (elhs,erhs) = match equalitysides(law){
        Some((lhs_node, rhs_node)) =>(lhs_node,rhs_node),
        None => {

            eprintln!("issue around equalityside expression not being right");
            return self.term.clone();

        }
    };*/
    let (elhs, erhs) = law;
    let (lhs_node,lhs_size) = elhs;
    let (rhs_node,rhs_size) = erhs;
    fn rec(targetnode:&Node,rule_pattern:&Node,subst_pattern:&Node)->(Node,i16){
        if let Some(relations) = matchandassigns(rule_pattern,targetnode ){
            return nodesubst(subst_pattern,&relations);


        }
        match targetnode{
            Node::BinaryOp(lhs,op,rhs) => {
                let (new_lhs,lhsize) = rec(lhs,rule_pattern,subst_pattern);
                let (new_rhs,rhsize) = rec(rhs,rule_pattern,subst_pattern);
                (Node::BinaryOp(Box::new(new_lhs),*op,Box::new(new_rhs)),lhsize + rhsize +1) 


            }

            Node::Number(val) => (Node::Number(*val),0),
            Node::Variable(c) => (Node::Variable(*c),0)

        }

    }

    let (term,size )=rec(&self.term,&lhs_node,&rhs_node);
    return Term{term:term,size:size}


}


}

impl fmt::Display for Term {

    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{

        write!(f,"{},of size {}",self.term,self.size)



    }




}

// this fct obviously turns a node into a string 











/* 
//first try for canmatch
pub fn canmatch(&self,other:&Node/* ,relations:HashMap<char,Node>*/)-> bool{ // here self is the lhs btw 
    //let mut elhs = &self.term.clone();
    if self.term.same_type(other) == false {

        return false
    }
    match &self.term {
    Node::Number(a) => *a == other.get_number().unwrap(),
    Node::Variable(a) => self.term.same_type(other),
    Node::BinaryOp(plhs,op_self,prhs) =>{
    if let Node::BinaryOp(tlhs,op_self,trhs)= other{
        self.canmatch(tlhs) && self.canmatch(trhs)

     

    }


    
    else {
        false
        //ahaha haha funn
    }
    



    }
    _ => false,
}
}


*/






//check if it's an equality, returns lhs and rhs if it's not 
pub fn equalitysides(term:&Node)->Option<(Node,(Node))>{
    if let Node::BinaryOp(lhs,Operator::Assign ,rhs) = term.clone(){
        return Some((*lhs,*rhs))
    }
    else {

        return None 
    }

}
//substitutes a node 
pub fn nodesubst(snode:&Node,relations:&HashMap<char,Node>)->(Node,i16){
    let nodesize = 0; 
    match snode{
        Node::Number(n) =>{
             
            (Node::Number(*n),0)
        }
        Node::Variable(c) =>{
            if let Some(mnode) = relations.get(c){
                (mnode.clone(),countsize(mnode))


            }
            else{


                (Node::Variable(*c),0)
            }


        }
        Node::BinaryOp(lhs,op ,rhs ) => {
            let (new_lhs,lhsize) = nodesubst(lhs,relations); 
            let (new_rhs ,rhsize)= nodesubst(rhs, relations);
            (Node::BinaryOp(Box::new(new_lhs),*op,Box::new(new_rhs)),lhsize + rhsize +1)


        }

    }


}
// 
pub fn countsize(node:&Node)->i16{
    let size:i16 = 0; 
    match node{
        Node::Number(_) =>{
            return 0;
        }
        Node::Variable(_)=>{
            return 0;
        }
        Node::BinaryOp(lhs,_ ,rhs ) => {
            return countsize(lhs) + countsize(rhs) + 1;

        }


    }


}

// this function matchand binds on a given like node, it doesn't move the node 
// what i need rn is a function that takes this matchandbinds if it return a failure on a given node in the b 
pub fn matchandassigns(pattern:&Node, target:&Node)->Option<HashMap<char,Node>>{
    let mut relations = HashMap::new();
    pub fn matchandbinds(pattern:&Node, target:&Node, relations:&mut HashMap<char,Node>)->bool{ 
            if pattern.same_type(target)== false{
                return false

            }
            if let Node::Variable(pattern_char) = pattern{
                if let Some(prev) = relations.get(pattern_char){
                
                return prev == target;// false when same var is assigned to diff things (i feel this might get fucked later on)

            }
                else {

                    relations.insert(*pattern_char,target.clone());
                    return true;
                }
            }
            if !pattern.same_type(target){
                return false;
            }
            match pattern {
                Node::Number(value) => {
                    *value == target.get_number().unwrap()


             }
                Node::BinaryOp(plhs,_,prhs ) => {
                    if let Node::BinaryOp(tlhs,_,trhs) = target{
                        let lmatch = matchandbinds(plhs, tlhs, relations);
                        if lmatch == false {return false}
                        let rmatch = matchandbinds(prhs,trhs,relations); 
                        if rmatch == false {return false}
                        return true
                    
                    }
                    else {
                        false 

                    }


                }

                _ => false
            }



    }

    if matchandbinds(pattern, target, &mut relations){
        Some(relations)

    }
    else{

        None
    }



}
  
#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function for binary operations in tests
    fn bin_op(lhs: Node, op: Operator, rhs: Node) -> Node {
        Node::BinaryOp(Box::new(lhs), op, Box::new(rhs))
    }

    #[test]
    fn test_term_partial_ord_by_size() {
        let term_small_ops = Term { term: Node::Number(1), size: 0 }; // 0 ops
        let term_medium_ops = Term { term: bin_op(Node::Number(1), Operator::Add, Node::Number(2)), size: 1 }; // 1 op
        let term_large_ops = Term { term: bin_op(term_medium_ops.term.clone(), Operator::Multiply, Node::Number(3)), size: 2 }; // 2 ops

        // Test based on 'size' attribute (primary comparison)
        assert!(term_small_ops < term_medium_ops, "0 ops < 1 op");
        assert!(term_medium_ops < term_large_ops, "1 op < 2 ops");
        assert!(term_small_ops < term_large_ops, "0 ops < 2 ops");
        assert!(term_large_ops > term_small_ops, "2 ops > 0 ops");

        // Test equality based on size
        let term_small_ops_clone = Term { term: Node::Variable('x'), size: 0 }; // Another 0-op term
        assert!(term_small_ops == term_small_ops_clone, "Terms with same size should be equal by size initially");
    }
}