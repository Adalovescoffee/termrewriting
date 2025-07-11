use core::fmt;
use std::collections::HashMap;
//use std::vec;
//use crate::lexer::Lexer;
use crate::parser::{ Node, Operator};
//#[derive(PartialEq, Eq)]
pub struct Term {
    pub term:Node,
    pub size:i16,
    


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
fn _complexitysize(&self)-> i32 {// this is when number of operations is the same 
let size = 0    ; 
    fn rec(node:&Node,number:i32)->i32{
            match node {
                Node::Number(_) => {number + 1}
                Node::Variable(_) =>{number +1}
                Node::BinaryOp(lhs,_ ,_rhs ) =>{
                    rec(lhs,number +1) 


                }


            }

    

    }

    return rec(&self.term,size) 
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
  
