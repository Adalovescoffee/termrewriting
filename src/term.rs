use core::fmt;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::Hash;
use std::collections::HashSet;
//use std::vec;
use crate::lexer::Lexer;
use crate::parser::{ Node, Operator,Parser,ParserError};

//#[derive(PartialEq, Eq)]

/// a term is a node (variable,number, binary operation of nodes) and has a size (number of operations)
#[derive(Clone,Debug,Hash)]
pub struct Term {
    pub term:Node,
    pub size:i16,
    
    // i should probably add a vec here or smt where i store all the rewrites of the term in order, then for equality i'd just need first to check for the intersection of the rr and see if there is anything matching to 
    //check for equality     


}
// In term.rs, add this after the PartialEq implementation
impl Eq for Term {}
impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        // First check if sizes are equal as a quick optimization
        if self.size != other.size {
            return false;
        }
        
        // Then compare the actual term structure
        nodes_equal(&self.term, &other.term)
    }
}

// Helper function to compare node structures
fn nodes_equal(node1: &Node, node2: &Node) -> bool {
    let mut variable_equality: HashMap<char, char> = HashMap::new();
    
    fn rec(node1: &Node, node2: &Node, variable_equality: &mut HashMap<char, char>) -> bool {
        match (node1, node2) {
            (Node::Number(n1), Node::Number(n2)) => n1 == n2,
            (Node::Variable(c1), Node::Variable(c2)) => {
                
                if let (Some(mapped_c1), Some(mapped_c2)) = (
                    variable_equality.get(c1),
                    variable_equality.get(c2),
                ) {
                   
                    *mapped_c1 == *c2 && *mapped_c2 == *c1
                } else if let Some(mapped_c1) = variable_equality.get(c1) {
                    
                    *mapped_c1 == *c2
                } else if let Some(mapped_c2) = variable_equality.get(c2) {
                    
                    *mapped_c2 == *c1
                } else {
                    
                    variable_equality.insert(*c1, *c2);
                    variable_equality.insert(*c2, *c1);
                    true
                }
            }
            (Node::UnaryOp(op1, rhs1), Node::UnaryOp(op2, rhs2)) => {
                op1 == op2 && rec(rhs1, rhs2, variable_equality)
            }
            (Node::BinaryOp(lhs1, op1, rhs1), Node::BinaryOp(lhs2, op2, rhs2)) => {
                op1 == op2 
                    && rec(lhs1, lhs2, variable_equality) 
                    && rec(rhs1, rhs2, variable_equality)
            }
            _ => false,
        }
    }
    
    rec(node1, node2, &mut variable_equality)
}

/// order of complexity aka comparing the number of operations in a tree + checking the lhnode of the trees if equality   
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


// since rust can't have multiple orders implemented for the same struct we're making a wrapper structs 
/// subsumption order wrapper 
pub struct BySubsumption<'a>(pub &'a Term);

impl <'a> PartialEq for BySubsumption<'a> {
    fn eq(&self,other:&Self)-> bool {
        self.0.subsumes(&other.0.term) && other.0.subsumes(&other.0.term)


    }


}
impl <'a> PartialOrd for BySubsumption<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0.subsumes(&other.0.term),other.0.subsumes(&self.0.term))
        {
            (true,true) => Some(Ordering::Equal),
            (true,false) => Some(Ordering::Less),
            (false,true) => Some(Ordering::Greater),
            (false,false) =>{ 
                println!("the two terms are incomparable");
                
            None}


        }

    }



} 
/*impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    
        
    }


        
    



}*/


impl Term{     
pub fn getequality(&self) -> Option<((Node, i16), (Node, i16))> {
    if let Node::BinaryOp(lhs, Operator::Assign, rhs) = &self.term {
        let lhs = (*lhs).clone();
        let rhs = (*rhs).clone();
        return Some(((*lhs.clone(), lhs.size()), (*rhs.clone(), rhs.size())));
    } else {
        return None;
    }
}

/// returns the number of operations in a term 
fn complexitysize(&self)-> i16{// this is when number of operations is the same 
   // wtf u're telling me i don't account for the lhs? wtf is going on ohh nvm this is the one when equality ok
    fn rec(node:&Node)->i16{
            match node {
                Node::Number(_) => 0,
                Node::Variable(_) =>0, 
                Node::BinaryOp(lhs,_ ,_rhs ) =>{
                    1 + rec(lhs)


                }
                Node::UnaryOp(_,rhs)=> {
                    1 + rec(rhs) }// idk if it should add 1 or not we'll see  

            }

    

    }

    return rec(&self.term) 
}
// okay here is when things become interesting 
// ok so we decided self has term + size law has to have some sort of size value inbedded only issue is that size
// maybe it should be law :((lhs,size),(rhs,size)) 
//time to change



/// rewrites term by an equality, returns it as a term 
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
    //let self_renamed = changecommonvariables(&self, &Term{term:lhs_node.clone(),size:lhs_size});
    fn rec(targetnode:&Node,rule_pattern:&Node,subst_pattern:&Node)->(Node,i16){
        if let Some(relations) = matchandassigns(rule_pattern,targetnode ){
            //println!("inside rewrite if {:?}", relations);
            return nodesubst(subst_pattern,&relations);


        }
        match targetnode{
            Node::BinaryOp(lhs,op,rhs) => {
                let (new_lhs,lhsize) = rec(lhs,rule_pattern,subst_pattern);
                let (new_rhs,rhsize) = rec(rhs,rule_pattern,subst_pattern);
                //println!("new_lhs,new_rhs, {:?}, {:?}",new_lhs,new_rhs);
                
                (Node::BinaryOp(Box::new(new_lhs),*op,Box::new(new_rhs)),lhsize + rhsize +1) 

            
            
            }
             Node::UnaryOp(op,rhs) => {// for now the only op is minus 
                let (new_rhs,rhsize)= rec(rhs,rule_pattern,subst_pattern);
                //let (new_lhs,lhsize)= rec(&Node::Number(0),rule_pattern,subst_pattern);// so here we consider -a to be 0 - a 
                // possible issue -- a :c yep that was dumb
                (Node::UnaryOp(*op,Box::new(new_rhs)), rhsize + 1)
            }
            Node::Number(val) => (Node::Number(*val),0),
            Node::Variable(c) => (Node::Variable(*c),0)

        }

    }

    let (term,size )=rec(&self.term,&lhs_node,&rhs_node);
    return Term{term:term,size:size}


}

/// checks if one term is a subsumption of another  (not tested )
pub fn subsumes(&self, target:&Node)->bool{  // this is called basic unification apparently
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

    if matchandbinds(&self.term, target, &mut relations){
        true

    }
    else{

        false
    }



}
}

impl fmt::Display for Term {

    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{

        write!(f,"{},of size {}",self.term,self.size)



    }




}

// this fct obviously turns a node into a string 















// subsumption ordering x*y < (a+b)*(c+d) 





///check if it's an equality, returns lhs and rhs if it's not 
pub fn _equalitysides_node(term:&Node)->Option<(Node,Node)>{
    if let Node::BinaryOp(lhs,Operator::Assign ,rhs) = term.clone(){
        return Some((*lhs,*rhs))
    }
    else {

        return None 
    }

}

///substitutes a node 
pub fn nodesubst(snode:&Node,relations:&HashMap<char,Node>)->(Node,i16){ 
    
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
            if op == &Operator::Subtract  {
                (Node::UnaryOp(Operator::Subtract,Box::new(new_rhs)),rhsize + 1)

            }
            else {
            (Node::BinaryOp(Box::new(new_lhs),*op,Box::new(new_rhs)),lhsize + rhsize +1)}
            

        }
        Node::UnaryOp(op,rhs) => {
            
            let (new_rhs,rhsize) = nodesubst(rhs,relations);
            (Node::UnaryOp(*op,Box::new(new_rhs)), rhsize + 1)


        }

    }


}
/// counts the size of a tree 
pub fn countsize(node:&Node)->i16{
  
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
        Node::UnaryOp(_,rhs ) => {
            return countsize(rhs) + 1; 

        }

    }


}

pub fn changecommonvariables(pattern:&Term,target:&Term)->(Term,HashMap<char,Node>){
    let pattern_variables = variable(&pattern.term);
    let target_variables = variable(&target.term);
    let size = pattern.size;
    let commonchars: HashSet<char> = pattern_variables.intersection(&target_variables).cloned().collect();
    let alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let alphabetset:HashSet<char> = alphabet.into_iter().collect();
    let mut recording_hash:HashMap<char,Node> = HashMap::new();
    //let patternterm = Term{term:pattern.clone(),size:0};
    //let union:HashSet<char>= pattern_variables.union(&target_variables).cloned().collect();
    let mut difference: Vec<char> = alphabetset.difference(&commonchars).cloned().collect();
    pub fn rec(pattern: &Node, commonchars: HashSet<char>, difference: &mut Vec<char>, hash:&mut HashMap<char,Node>) -> Node {
        
        match pattern {
            Node::BinaryOp(lhs, Operator, rhs) => {
                return Node::BinaryOp(Box::new(rec(&lhs, commonchars.clone(), difference,hash)), *Operator, Box::new(rec(&rhs, commonchars, difference,hash)))

            }
            Node::Variable(char) => {
                if commonchars.contains(&char){
                    if let Some(c) = difference.pop(){
                        
                        //checking that this wasn't met before 
                        if let Some(Node::Variable(p)) = hash.get(&char){
                            return Node::Variable(*p);


                        }
                        else {
                        hash.insert(*char, Node::Variable(c));
                        return Node::Variable(c);
                        }

                    }
                    else {
                        println!("not enough letters in the alphabet :c ");
                        hash.insert(*char, Node::Variable(*char));
                        return Node::Variable(*char);

                    }

               
            }
            else{
                return Node::Variable(*char);

            }
            }
            Node::UnaryOp(op,rhs ) => {
                return Node::UnaryOp(*op,Box::new(rec(&rhs,commonchars, difference,hash)))


            }
            Node::Number(number) => {
                return Node::Number(*number)
            }
        }
    


    }
return (Term{term:rec(&pattern.term,commonchars,&mut difference,&mut recording_hash),size:size},recording_hash)



}
// this function matchand binds on a given like node, it doesn't move the node 
// what i need rn is a function that takes this matchandbinds if it return a failure on a given node in the b 
/// On a given node of the ast, it attempts to match if one node can be substituted (subsumpted?)
pub fn matchandassigns(pattern:&Node, target:&Node)->Option<HashMap<char,Node>>{  // this is called basic unification apparently
    //let patternrenamed = changecommonvariables(pattern, target);
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
            /*if !pattern.same_type(target){
                return false;
            }*/
            match pattern {
                Node::Number(value) => {
                    *value == target.get_number().unwrap()


             }  
                Node::UnaryOp(_,prhs )=> {
                    // i have a feeling this if statement is useless actually since sametype already does that 
                    if let Node::UnaryOp(_,trhs ) = target {
                        let rmatch = matchandbinds(prhs,trhs,relations);
                        if rmatch == false {return false}
                        return true 

                    }
                    else {
                        false

                    }



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
  
/// helper that defines term from str (so far no error when term is an equality 
pub fn from_str(s: &str) -> Result<Term, String> {
        let lexer = Lexer::new(s.to_string());
        let mut parser = Parser::new(lexer);

        match parser.parse_equality() {
            Ok(((lhs_node, lhs_ops), (rhs_node, rhs_ops))) => {
                
                if lhs_node == rhs_node { 
                    Ok(Term { term: lhs_node, size: lhs_ops })
                } else {
                    // If lhs_node != rhs_node, it means parse_equality found an assignment
                    // for now only viable for single term not assignements 
                    // i guess time to fix assignements :) 
                   let term = Node::BinaryOp(Box::new(lhs_node),Operator::Assign,Box::new(rhs_node));
                Ok(Term{term:term,size:lhs_ops + rhs_ops + 1})
                }
            },
            Err(e) => Err(format!("Parsing error for '{}': {:?}", s, e)),
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Helper function for binary operations in tests
    fn bin_op(lhs: Node, op: Operator, rhs: Node) -> Node {
        Node::BinaryOp(Box::new(lhs), op, Box::new(rhs))
    }
    
    #[test]
    fn renaming(){
        let t1 = from_str("(a + 0)+ a").unwrap();
        let t2 = from_str("a  + c").unwrap();
        let renamed = changecommonvariables(&t1, &t2);
        let nodesubstitution = nodesubst(&t2.term, &renamed.1);
        println!("{}",renamed.0);
        println!("substitution : {}",Term{term:nodesubstitution.0,size:nodesubstitution.1});
      



    }
    #[test]
    fn equaltest(){
        let t1 = from_str("(b + 0)+ a").unwrap();
        let t2 = from_str("(c + 0) + c").unwrap();
        let isequal =(t1 == t2);
        assert_eq!(true,isequal)


    }
    #[test]
    fn testsubsumptionorder(){
        let t1 = from_str("a + b").unwrap();
        let t2 = from_str("(x*y) + c").unwrap();
        
        let term1 = BySubsumption(&t1);
        let term2 = BySubsumption(&t2);
        
        let boolt = term1 < term2;
        assert_eq!(boolt,true)
      
        

    }
    #[test]
    fn testsubsumptionorder2(){
        let t1 = from_str("a + b").unwrap();
        let t2 = from_str("b + c").unwrap();
        
        let term1 = BySubsumption(&t1);
        let term2 = BySubsumption(&t2);
        
        let boolt = term1 < term2;
        assert_eq!(boolt,false)
      
        

    }
    #[test]
    fn testsubsumptionorder3(){
        let t1 = from_str("a + b").unwrap();
        let t2 = from_str("c").unwrap();
        
        let term1 = BySubsumption(&t1);
        let term2 = BySubsumption(&t2);
        
        let boolt = term1 < term2;
        assert_eq!(boolt,false)
      
        

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
     #[test]

    fn test_term_partial_ord_complex_tie_break_left_leaning() {
        // Term A: (a + b) * c
        // Ops (size): 2 (for '+' and '*')
        // Left-leaning Nodes (complexitysize):
        //   '*' (1) + left child of '*' (which is '(a+b)')
        //   '+' (1) + left child of '+' (which is 'a')
        //   'a' (1)
        // Total complexitysize for Term A: 1 (for '*') + 1 (for '+') + 1 (for 'a') = 3 nodes
        let term_a_node = Node::BinaryOp(
            Box::new(Node::BinaryOp(
                Box::new(Node::Variable('a')),
                Operator::Add,
                Box::new(Node::Variable('b')),
            )),
            Operator::Multiply,
            Box::new(Node::Variable('c')),
        );
        let term_a = Term { term: term_a_node.clone(), size: countsize(&term_a_node) };
        println!("Term A: {}", term_a);
        println!("  Actual size: {}", term_a.size);
        println!("  Actual complexitysize: {}", term_a.complexitysize());
        assert_eq!(term_a.size, 2, "Term A size should be 2");


        let term_b_node = Node::BinaryOp(
            Box::new(Node::Variable('x')),
            Operator::Multiply,
            Box::new(Node::BinaryOp(
                Box::new(Node::Variable('y')),
                Operator::Add,
                Box::new(Node::Variable('z')),
            )),
        );
        let term_b = Term { term: term_b_node.clone(), size: countsize(&term_b_node) };
        println!("Term B: {}", term_b);
        println!("  Actual size: {}", term_b.size);
        println!("  Actual complexitysize: {}", term_b.complexitysize());
        assert_eq!(term_b.size, 2, "Term B size should be 2");

        // Test the comparison
        println!("\nComparing Term A and Term B:");
        println!("  Term A: {} (size={}, complexity={})", term_a.term, term_a.size, term_a.complexitysize());
        println!("  Term B: {} (size={}, complexity={})", term_b.term, term_b.size, term_b.complexitysize());

        assert_eq!(term_a.partial_cmp(&term_b), Some(Ordering::Greater), "Term A should be Greater than Term B");
        assert_eq!(term_b.partial_cmp(&term_a), Some(Ordering::Less), "Term B should be Less than Term A (symmetric check)");
        assert!(term_a > term_b, "Term A should be greater than Term B using '>' operator");
        println!("Term A: {}, > term B: {}",term_a.term,term_b.term);
        assert!(term_b < term_a, "Term B should be less than Term A using '<' operator");
    }

    

    #[test]
    fn subsumptionunificationtest(){
         let t1 = from_str("x + 0   ").unwrap();
        let t2 = from_str("-x + x").unwrap();
        
        let term1 = BySubsumption(&t1);
        let term2 = BySubsumption(&t2);


        println!("Hashmap for (-a +a -> (x+y)+z ) is {:?}",matchandassigns(&t2.term, &t1.term));
        let boolt = term1 >term2;
        assert_eq!(boolt,false)




    }
    #[test]
    fn unificationvariablenumber(){

        let t1 = from_str("x").unwrap();
        let t2 = from_str("0").unwrap();
        println!("x,0 {:?}",unification(&t2,&t1));
        println!("x,0:{:?}",unification(&t1,&t2));

        // this works 
    }
    #[test]
    fn unificationvariables(){

        let t1 = from_str("x").unwrap();
        let t2 = from_str("y").unwrap();
        println!("x,y:{:?}",unification(&t2,&t1));


    }
    #[test]
    fn unificationbinaryop(){
        let t1 = from_str("-x + x").unwrap();
        let t2 = from_str("a + 0 ").unwrap();
        println!("unification of -x + x and a + 0 leads to :{:?}",unification(&t2,&t1));}
    #[test]
    fn unificationmoreshenanigans(){
        let t1 = from_str("(x + y) * (z - 5)").unwrap();
        let t2 = from_str("(3 + 4) * (w - 5)").unwrap();
       println!("unification of (x + y) * (z - 5) and (3 + 4) * (w - 5) leads to :{:?}",unification(&t2,&t1));


    
    }
    #[test]
    fn unificationhard (){
    let t1 = from_str("(x + 2) * z").unwrap();
    let t2 = from_str("(y + y   ) *3").unwrap();
       println!("unification of (x + 2) *z and (y + y) * 3  leads to :{:?}",simpleunification(&t2,&t1));


            


    }
    #[test]
    fn chatgeppittysaidthiswouldntworkbutitdidehehe(){
        let t1 = from_str("x ").unwrap();
        let t2 = from_str("y + y ").unwrap();
       println!("unification of x and y + y  leads to :{:?}",unification(&t2,&t1));   
    }
    #[test]
    fn chatgeppittysaidthiswouldntworkbutitdidehehehe(){
        let t1 = from_str("x + x").unwrap();
        let t2 = from_str("(y + 1) + (3 + 1)").unwrap();
       println!("unification of x+x and (y + 1) + (3 + 1)  leads to :{:?}",unification(&t1,&t2));   
    }
    #[test]
    fn simplevariablescheck(){

        let t1 = from_str("(x+y)").unwrap();
        println!("the variables here are : {:?}",variable(&t1.term))


    }

    #[test]
    fn variablescheckcomplicated(){

        let t1 = from_str("-(-(a+0)+y)+ b").unwrap();
        println!("the variables here are : {:?}",variable(&t1.term))


    }
    #[test]
    fn occursvariable(){
        let t1 = from_str("(x +a) + y").unwrap().term;
        println!("x+y have the variable x in it :  {:?}",occurs('x', &t1));


    }
    #[test]
    fn ultimateunificationtest(){
        let t1 = from_str("-a + a").unwrap();
        let t2 = from_str("(x+y)+z ").unwrap();
       println!("unification of -a + a and (x+y)+z  leads to :{:?}",unification(&t1,&t2)); 


    }
    #[test]
    fn unificationtest2(){
        let t1 = from_str("(x+5)*z").unwrap();
        let t2 = from_str("(3+y)*2 ").unwrap();
       println!("unification of (x+5)*z and (3+y)*2  leads to :{:?}",unification(&t1,&t2)); 


    }
    #[test]
    fn unificationtest3(){
        let t1 = from_str("(x + y) * (x + y)").unwrap();
        let t2 = from_str("(a + 2) * (a + b) ").unwrap();
       println!("unification of (x + y) * (x + y) and (a + 2) * (a + b) leads to :{:?}",unification(&t1,&t2)); 


    }
    #[test]
    fn unificationn(){
        let t1 = from_str("(x  + 5) + d ").unwrap();
        let t2 = from_str("(z + z) + z ").unwrap();
       println!("unification of x  + 5 and (z + z) + z leads to :{:?}",unification(&t1,&t2)); 



    }
    #[test]
    fn groupaxiom12(){
        let t1 = from_str("0 + x ").unwrap();
        let t2 = from_str("a + 0 ").unwrap();
       println!("unification of 0 + x and a + 0 leads to :{:?}",unifyandfill(&t1,&t2)); 


    }
    #[test]
    fn groupaxiom23(){
        let t1 = from_str("a + 0 ").unwrap();
        let t2 = from_str("-x + x ").unwrap();
       println!("unification of a + 0 and -x + x leads to :{:?}",unifyandfill(&t1,&t2)); 


    }
    #[test]
    fn groupaxiom14(){
        let t1 = from_str("0 + a ").unwrap();
        let t2 = from_str("(x + y) + z").unwrap();
       println!("unification of 0 + a and (x + y) + z leads to :{:?}",unifyandfill(&t1,&t2)); 



    }
    #[test]
    fn groupaxiom24(){
        let t1 = from_str("a + 0 ").unwrap();
        let t2 = from_str("(x + y) + z ").unwrap();
       println!("unification of a + 0 and (x + y) +z leads to :{:?}",unifyandfill(&t2,&t1)); 
       println!("{:?}",t2>t1);

    }
    #[test]
    fn groupaxiom24ez(){
        let t1 = from_str("a + 0 ").unwrap();
        let t2 = from_str("(x + y)+z ").unwrap();
       println!("unification of a + 0 and (x + y) leads to :{:?}",unifyandfill(&t1,&t2)); 
       

    }
    #[test]
    fn groupaxiom34(){
        let t1 = from_str("-a + a ").unwrap();
        let t2 = from_str("(x + y) + z ").unwrap();
       println!("unification of -a + a and (x + y) +z leads to :{:?}",unifyandfill(&t1,&t2)); 


    }
    #[test]
    fn findtest(){

        let hashmap: HashMap<char, Node> = vec![
            ('x', Node::Variable('a')),
            ('y', Node::Number(0)),
            ('a', Node::Variable('x')),
        ].into_iter().collect();

        println!("find function gives :{:?} ", find('a',&hashmap));
    }
}


pub fn variable(node:&Node)-> HashSet<char> {
    let mut variables:HashSet<char> = HashSet::new();
    fn rec(node:&Node,mut vars:HashSet<char>)->HashSet<char>{
        match node {
            Node::Number(_) => {return vars;}

            Node::Variable(char) => {
                if vars.contains(char)==false {
                    vars.insert(*char);
                    return vars 
                }
                else {
                    return vars


                }

            }
            Node::BinaryOp(lhs,_ ,rhs ) => {
                vars = rec(lhs,vars);
                vars = rec(rhs,vars);
                return vars 

            }
            Node::UnaryOp(_,rhs  ) => {
                vars = rec(rhs,vars);
                return vars 


            }


        }
    
    }
    return rec(node,variables)



}
fn find(c: char, relations: &HashMap<char, Node>) -> Option<Node> {
    let mut visited = HashSet::new();
    let mut current = c;
    
    while let Some(node) = relations.get(&current) {
        if visited.contains(&current) {
            return Some(node.clone())
        }
        visited.insert(current);
        
        match node {
            Node::Variable(next_char) => {
                current = *next_char;
            }
            
            _ => return Some(node.clone()),
        }
    }
    
    Some(Node::Variable(current))
}


// idk man leave me alone i'm tired 
// okay we have an issue here i imagine which is the fact if a variable is free it won't appear and that needs to change soon 
pub fn occurs( variable_char:char,node:&Node)-> bool{
    let chars = variable(node);
    if chars.contains(&variable_char) {

        return true 
    }
    else {

        return false 
    }



}

pub fn unification(pattern: &Term, target: &Term) -> Option<HashMap<char, Node>> {
    let mut relations: HashMap<char, Node> = HashMap::new();
    let mut chars: HashSet<char> = HashSet::new();
    
    let targetterm= &target.term;
    let patternterm = &pattern.term;
    fn fifi( pattern: &Node, target: &Node, relations: &mut HashMap<char, Node>, chars: &mut HashSet<char> ) -> bool {
        let root_pattern = match pattern {
            Node::Variable(c) => find(*c, relations),
            _ => Some(pattern.clone()),
        };

        let root_target = match target {
            Node::Variable(c) => find(*c, relations),
            _ => Some(target.clone()),
        };

        if root_pattern.is_none() || root_target.is_none() {
            return false;
        }

        let root_pattern = root_pattern.unwrap();
        let root_target = root_target.unwrap();

        if root_pattern == root_target {
            return true;
        }

        match (&root_pattern, &root_target) {
            (Node::Variable(p_char), Node::Variable(t_char)) => {
              
                relations.insert(*p_char, Node::Variable(*t_char));
                //println!("hashmap is {:?}",relations);
                chars.insert(*p_char);
                //chars.insert(*t_char);
                true
            }
            (Node::Variable(p_char), _) => {
                if occurs(*p_char, &root_target) {
                    false
                } else {
                    relations.insert(*p_char, root_target.clone());
                    //println!("hashmap is {:?}",relations);
                    chars.insert(*p_char);
                    true
                }
            }
            (_, Node::Variable(t_char)) => fifi(&root_target, &root_pattern, relations, chars),
            (Node::Number(p_val), Node::Number(t_val)) => p_val == t_val,
            (Node::UnaryOp(p_op, p_rhs), Node::UnaryOp(t_op, t_rhs)) => {
                p_op == t_op && fifi(p_rhs, t_rhs, relations, chars)
            }
            (Node::BinaryOp(p_lhs, p_op, p_rhs), Node::BinaryOp(t_lhs, t_op, t_rhs)) => {
                
                p_op == t_op && fifi(p_lhs, t_lhs, relations, chars) &&
                fifi(p_rhs, t_rhs, relations, chars)
                    // need to know which one is correct and return on the variables there to themselves, add some checks that it coincides 
                    // with the other side too example : 
                    // for now i'll leave it there, i'm hungy too :c 
                
                

               
            }
            _ => false,
        }
    }
    pub fn finalchecksubstitution(chars:&mut HashSet<char>,relations:&mut HashMap<char,Node>,pattern: &Node,target: &Node) -> HashMap<char,Node>{
        let mut substitution = HashMap::new();
        let mut charscopy = chars.clone();
        let patternvariable:HashSet<char> = variable(pattern).into_iter().collect();
        let targetvariable:HashSet<char> = variable(target).into_iter().collect();

        //println!("chars is {:?}",charscopy);
        for c in charscopy {
            if let Some(mut root) = find(c, &relations) {
                println!("{:?}",(root.clone(),c));
                if let Node::Variable(d) = &root {
                    if *d == c {
                        continue;
                    }
                    substitution.insert(*d,Node::Variable(c));
                    chars.insert(*d);
                }
                let rootvariable:HashSet<char> = variable(&root).into_iter().collect();
                
                let mut hashroot:HashMap<char,Node> = HashMap::new();
                for v in rootvariable.clone(){
                    if let Some(Node::Number(numba))=find(v,relations){
                        hashroot.insert(v, Node::Number(numba));


                    }
                    else {
                        if let Some(vnode) = find(v,relations){
                            let vendnodevariable:HashSet<char> = variable(&vnode).into_iter().collect();
                            if patternvariable.contains(&c)&& vendnodevariable.is_subset(&targetvariable){
                                hashroot.insert(v,vnode);

                            
                            }
                        else if targetvariable.contains(&c)&&vendnodevariable.is_subset(&patternvariable){
                            hashroot.insert(v,vnode);
                            }
                            else {
                               // println!("medemedewehaveissues,vnode:{:?}",vnode);


                            }
                        }

                    }
                    substitution.insert(c,root.clone());
                    chars.insert(c);
                }
                   
                
                substitution.insert(c, nodesubst(&root, &hashroot).0);
                chars.insert(c);
                
            }
           
        }
      //  println!("hashmapp inside finalcheck is {:?}",substitution);
       // println!("chars inside of finalcheck is {:?}",chars);
        return(substitution)
    

}
    pub fn unifyandfill(pattern_term:&Node,target_term:&Node,mut relations:HashMap<char,Node>,mut chars:HashSet<char>)->Option<HashMap<char,Node>>{
        if let Node::BinaryOp(tlhs,_,trhs) = &target_term{
            let mut copyrelations = relations.clone();
            match (fifi(&pattern_term,&tlhs,&mut relations,&mut chars),fifi(&pattern_term,&trhs,&mut copyrelations,&mut chars.clone())){
                (true,true) => {
                    //println!("???? both sides can match weird also rhs is {:?} and chars is {:?}",trhs,chars);
                    let check = variable(trhs);
                    let mut result = finalchecksubstitution(&mut chars, &mut relations,&pattern_term,tlhs);
                    for c in check {
                        if chars.contains(&c){
                            return None;


                        }
                        // need to add a check if this c is already in chars or smt so that there are no issues 
                        result.insert(c, Node::Variable(c));
                        chars.insert(c);


                    }
                    //println!("this is final chars i think{:?}",chars);
                    if variable(pattern_term).is_subset(&chars) && variable(target_term).is_subset(&chars){
                        //println!("yey it matches also the result is : {:?}",result);

                    }
                    else {
                        //println!("yuck : {:?}",result);   
                    }
                    Some(result)
                    
                }
                (true,false)=> {
                   // println!("true,false");
                    
                    let result = Some(finalchecksubstitution(&mut chars, &mut copyrelations,&pattern_term,tlhs));
                    //println!("chars here is {:?}", chars );
                    return result;
                }
                (false,true) => {

                    //println!("false,true");
                    let result = Some(finalchecksubstitution(&mut chars, &mut copyrelations,&pattern_term,trhs));
                    //println!("chars here is {:?}", chars );                    
                    return result;

                }
                (false,false)=> {
                    
                 return None
                }


                _ => {None}
            }

        }
        else{
            return None;
        }

    



    
    }
    //println!("pattern> target {:?}",target<pattern);
    //println!("pattern< target {:?}",target>pattern);
    if fifi(&patternterm, &targetterm, &mut relations, &mut chars)&& variable(patternterm).is_subset(&chars) && variable(targetterm).is_subset(&chars) {
        //println!("owo");
        
        Some(finalchecksubstitution(&mut chars, &mut relations,patternterm,targetterm))
    } 
    
    else {
        let mut relationscopy =  relations.clone();
        let mut charscopy = chars.clone(); 
        if let Some(unification) = unifyandfill(&pattern.term, &target.term, relationscopy, charscopy){
            //println!("aaaa {:?}",charscopy.);
            return Some(unification); 
        }
        else if let Some(unification) = unifyandfill(&target.term, &pattern.term, relations, chars){
           // println!("aaaa {:?}",chars);
            return Some(unification);


        }
        else {
            return None; 
        }
    }
}
pub fn simpleunification(pattern: &Term, target: &Term) -> Option<HashMap<char, Node>> {
    let mut relations: HashMap<char, Node> = HashMap::new();
    let mut chars: HashSet<char> = HashSet::new();
    let patternterm = &pattern.term;
    let targetterm= &target.term;
    fn fifi( pattern: &Node, target: &Node, relations: &mut HashMap<char, Node>, chars: &mut HashSet<char> ) -> bool {
        let root_pattern = match pattern {
            Node::Variable(c) => find(*c, relations),
            _ => Some(pattern.clone()),
        };

        let root_target = match target {
            Node::Variable(c) => find(*c, relations),
            _ => Some(target.clone()),
        };

        if root_pattern.is_none() || root_target.is_none() {
            return false;
        }

        let root_pattern = root_pattern.unwrap();
        let root_target = root_target.unwrap();

        if root_pattern == root_target {
            return true;
        }

        match (&root_pattern, &root_target) {
            (Node::Variable(p_char), Node::Variable(t_char)) => {
              
                relations.insert(*p_char, Node::Variable(*t_char));
                relations.insert(*t_char,Node::Variable(*p_char));
                //println!("hashmap is {:?}",relations);
                chars.insert(*p_char);
                chars.insert(*t_char);
                //chars.insert(*t_char);
                true
            }
            (Node::Variable(p_char), _) => {
                if occurs(*p_char, &root_target) {
                    false
                } else {
                    relations.insert(*p_char, root_target.clone());
                    //println!("hashmap is {:?}",relations);
                    chars.insert(*p_char);
                    true
                }
            }
            (_, Node::Variable(t_char)) => fifi(&root_target, &root_pattern, relations, chars),
            (Node::Number(p_val), Node::Number(t_val)) => p_val == t_val,
            (Node::UnaryOp(p_op, p_rhs), Node::UnaryOp(t_op, t_rhs)) => {
                p_op == t_op && fifi(p_rhs, t_rhs, relations, chars)
            }
            (Node::BinaryOp(p_lhs, p_op, p_rhs), Node::BinaryOp(t_lhs, t_op, t_rhs)) => {
                
                p_op == t_op && fifi(p_lhs, t_lhs, relations, chars) &&
                fifi(p_rhs, t_rhs, relations, chars)
                    // need to know which one is correct and return on the variables there to themselves, add some checks that it coincides 
                    // with the other side too example : 
                    // for now i'll leave it there, i'm hungy too :c 
                
                

               
            }
            _ => false,
        }
    }
    pub fn finalchecksubstitution(chars:&mut HashSet<char>,relations:&mut HashMap<char,Node>,pattern: &Node,target: &Node) -> HashMap<char,Node>{
        let mut substitution = HashMap::new();
        let mut charscopy = chars.clone();
        let patternvariable:HashSet<char> = variable(pattern).into_iter().collect();
        let targetvariable:HashSet<char> = variable(target).into_iter().collect();

        //println!("chars is {:?}",charscopy);
        for c in charscopy {
            if let Some(mut root) = find(c, &relations) {
                //println!("aaaaaaaaa {:?}",(root.clone(),c));
                if let Node::Variable(d) = &root {
                    if *d == c {
                        continue;
                    }
                    substitution.insert(*d,Node::Variable(c));
                    chars.insert(*d);
                }
                let rootvariable:HashSet<char> = variable(&root).into_iter().collect();
                
                let mut hashroot:HashMap<char,Node> = HashMap::new();
                for v in rootvariable.clone(){
                    if let Some(Node::Number(numba))=find(v,relations){
                        hashroot.insert(v, Node::Number(numba));


                    }
                    else {
                        if let Some(vnode) = find(v,relations){
                            let vendnodevariable:HashSet<char> = variable(&vnode).into_iter().collect();
                            if patternvariable.contains(&c)&& vendnodevariable.is_subset(&targetvariable){
                                hashroot.insert(v,vnode);

                            
                            }
                        else if targetvariable.contains(&c)&&vendnodevariable.is_subset(&patternvariable){
                            hashroot.insert(v,vnode);
                            }
                            else {
                                //println!("medemedewehaveissues,vnode:{:?}",vnode);


                            }
                        }

                    }
                    substitution.insert(c,root.clone());
                    chars.insert(c);
                }
                   
                
                substitution.insert(c, nodesubst(&root, &hashroot).0);
                chars.insert(c);
                
            }
           
        }
        //println!("hashmapp inside finalcheck is {:?}",substitution);
        //println!("chars inside of finalcheck is {:?}",chars);
        return(substitution)
    

}
   
    //println!("pattern> target {:?}",target<pattern);
    //println!("pattern< target {:?}",target>pattern);
    
    if fifi(&patternterm, &targetterm, &mut relations, &mut chars)&& variable(patternterm).is_subset(&chars) && variable(targetterm).is_subset(&chars) {
        //println!("owo");
        
        Some(finalchecksubstitution(&mut chars, &mut relations,patternterm,targetterm))
    } 
    
    else {
      //println!("chars is :{:?}",chars);
      return(None)
    }
}
pub fn unifyandfill(pattern: &Term, target: &Term) -> Option<HashMap<char, Node>> {
    
    if let Some(subst) = simpleunification(pattern, target) {
        //println!("ok this is where the problem is ");
        return Some(subst);
    }
    
    
    let pattern_term = pattern.clone().term; 
    let target_term = target.clone().term;
    match target_term {
        Node::BinaryOp(lhs, _, rhs) => {
            if let Some(subst) = unifyandfill(pattern, &Term{term:*lhs,size:0}) {
                return Some(subst);
            }
            if let Some(subst) = unifyandfill(pattern, &Term{term:*rhs,size:0}) {
                return Some(subst);
            }
        }
        Node::UnaryOp(_, rhs) => {
            if let Some(subst) = unifyandfill(pattern, &Term{term:*rhs,size:0}) {
                return Some(subst);
            }
        }
        _ => {}
    }
    match pattern_term {
        Node::BinaryOp(lhs, _, rhs) => {
            if let Some(subst) = unifyandfill( &Term{term:*lhs,size:0},target) {
                return Some(subst);
            }
            if let Some(subst) = unifyandfill( &Term{term:*rhs,size:0},target) {
                return Some(subst);
            }
        }
        Node::UnaryOp(_, rhs) => {
            if let Some(subst) = unifyandfill(&Term{term:*rhs,size:0},target) {
                return Some(subst);
            }
        }
        _ => {}
    }
    None
    //unifyandfill(target,pattern)
}
/* 
 pub fn unifyfill(pattern_term:&Node,target_term:&Node,mut relations:HashMap<char,Node>,mut chars:HashSet<char>)->Option<HashMap<char,Node>>{
        if let Node::BinaryOp(tlhs,_,trhs) = &target_term{
            let mut copyrelations = relations.clone();
            match (fifi(&pattern_term,&tlhs,&mut relations,&mut chars),fifi(&pattern_term,&trhs,&mut copyrelations,&mut chars.clone())){
                (true,true) => {
                    println!("???? both sides can match weird also rhs is {:?} and chars is {:?}",trhs,chars);
                    let check = variable(trhs);
                    let mut result = finalchecksubstitution(&mut chars, &mut relations,&pattern_term,tlhs);
                    for c in check {
                        if chars.contains(&c){
                            return None;


                        }
                        // need to add a check if this c is already in chars or smt so that there are no issues 
                        result.insert(c, Node::Variable(c));
                        chars.insert(c);


                    }
                    println!("this is final chars i think{:?}",chars);
                    if variable(pattern_term).is_subset(&chars) && variable(target_term).is_subset(&chars){
                        println!("yey it matches also the result is : {:?}",result);

                    }
                    else {
                        println!("yuck : {:?}",result);   
                    }
                    Some(result)
                    
                }
                (true,false)=> {
                    println!("true,false");
                    
                    let result = Some(finalchecksubstitution(&mut chars, &mut copyrelations,&pattern_term,tlhs));
                    println!("chars here is {:?}", chars );
                    return result;
                }
                (false,true) => {

                    println!("false,true");
                    let result = Some(finalchecksubstitution(&mut chars, &mut copyrelations,&pattern_term,trhs));
                    println!("chars here is {:?}", chars );                    
                    return result;

                }
                (false,false)=> {
                    
                 return None
                }


                _ => {None}
            }

        }
        else{
            return None;
        }

    



    
    }
*/
/// need to find a better name
/* 
pub fn compunification(pattern:&Node,target:&Node)-> Option<HashMap<char,Node>>{
    let mut freevariables:Vec<char> = Vec::new();
    pub fn rec(pattern:&Node,target:&Node,freevariables:Vec<char>)->bool{
        match (pattern,target) {
            (Node:Variable)





        }




    }


}


*/


///takes a pattern as well as a relations related to the pattern, fills the missing variables from the pattern with identity! 
/// don't think this is useful, will delete but who knows 
pub fn free(pattern:&Node,relations:HashMap<char,Node>)->Option<HashMap<char,Node>>{
    //let mut relationscopy = relations; 
    let mut id = 0;
    let mut relationscopy:HashMap<char,Node> = relations.clone();
    pub fn rec(pattern:&Node,relations:&mut HashMap<char,Node>,mut id:i64)-> bool{
        // this changes depending on the op 
        // 1 => * 
        // 0 => + 
        match pattern {
            Node::Variable(char) => {
                if let Some(prev) = relations.get(char){
                    
                        return prev == pattern; 
                }    
                else {
                    relations.insert(*char,Node::Number(id)); 
                    return true;  
                
                }


            }
            Node::BinaryOp(lhs,op,rhs) => {
                if op== &Operator::Add || op == &Operator::Subtract {
                    id = 0;
                    rec(lhs,relations,id) && rec(rhs,relations,id)
                }
                else if op == &Operator::Multiply || op == &Operator::Divide {

                    id = 1; 
                    rec(lhs,relations,id) && rec(rhs,relations,id)

                }
                else {
                    return false;

                }


            }
            Node::UnaryOp(op,rhs) => {
                // for now the only unaryop is minus so 
                id = 0;
                rec(rhs,relations,id)

            }
            Node::Number(number) => {
                // wtf do i do here continue ? 
                    
                return false; 

            }

            }


        }
    if rec(pattern, &mut relationscopy,id) {
        Some(relationscopy)


    }
    else {

        None
    }
}


