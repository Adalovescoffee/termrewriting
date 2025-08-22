use core::fmt;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::Hash;

//use std::vec;
use crate::lexer::Lexer;
use crate::parser::{ Node, Operator,Parser,ParserError};

//#[derive(PartialEq, Eq)]

/// a term is a node (variable,number, binary operation of nodes) and has a size (number of operations)
pub struct Term {
    pub term:Node,
    pub size:i16,
    
    // i should probably add a vec here or smt where i store all the rewrites of the term in order, then for equality i'd just need first to check for the intersection of the rr and see if there is anything matching to 
    //check for equality     


}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.complexitysize() == other.complexitysize()
    }
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
    fn rec(targetnode:&Node,rule_pattern:&Node,subst_pattern:&Node)->(Node,i16){
        if let Some(relations) = matchandassigns(rule_pattern,targetnode ){
            println!("inside rewrite if {:?}", relations);
            return nodesubst(subst_pattern,&relations);


        }
        match targetnode{
            Node::BinaryOp(lhs,op,rhs) => {
                let (new_lhs,lhsize) = rec(lhs,rule_pattern,subst_pattern);
                let (new_rhs,rhsize) = rec(rhs,rule_pattern,subst_pattern);
                println!("new_lhs,new_rhs, {:?}, {:?}",new_lhs,new_rhs);
                
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
pub fn _equalitysides(term:&Node)->Option<(Node,(Node))>{
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
pub fn unificationd(pattern: &Node, target: &Node) -> Option<HashMap<char, Node>> {
    let mut relations: HashMap<char, Node> = HashMap::new();
    
    fn unify_helper(pattern: &Node, target: &Node, relations: &mut HashMap<char, Node>) -> bool {
        match (pattern, target) {
            (Node::Variable(p_var), _) => {
                if let Some(bound) = relations.get(p_var).cloned() {
                    // If variable is already bound, unify with its value
                    unify_helper(&bound, target, relations)
                } else {
                    // Occurs check to prevent infinite recursion
                    if occurs(p_var, target, relations) {
                        return false;
                    }
                    relations.insert(*p_var, target.clone());
                    true
                }
            }
            (_, Node::Variable(t_var)) => {
                unify_helper(target, pattern, relations) // Swap and retry
            }
            (Node::Number(p_val), Node::Number(t_val)) => p_val == t_val,
            (Node::BinaryOp(p_lhs, p_op, p_rhs), Node::BinaryOp(t_lhs, t_op, t_rhs)) => {
                if p_op != t_op {
                    return false;
                }
                unify_helper(p_lhs, t_lhs, relations) && unify_helper(p_rhs, t_rhs, relations)
            }
            (Node::UnaryOp(p_op, p_rhs), Node::UnaryOp(t_op, t_rhs)) => {
                if p_op != t_op {
                    return false;
                }
                unify_helper(p_rhs, t_rhs, relations)
            }
            _ => false,
        }
    }

    fn occurs(var: &char, node: &Node, relations: &HashMap<char, Node>) -> bool {
        match node {
            Node::Variable(n_var) => {
                if n_var == var {
                    true
                } else if let Some(bound) = relations.get(n_var) {
                    occurs(var, bound, relations)
                } else {
                    false
                }
            }
            Node::BinaryOp(lhs, _, rhs) => occurs(var, lhs, relations) || occurs(var, rhs, relations),
            Node::UnaryOp(_, rhs) => occurs(var, rhs, relations),
            _ => false,
        }
    }

    if unify_helper(pattern, target, &mut relations) {
        // Resolve all variable bindings
        let mut changed = true;
        while changed {
            let mut updates = Vec::new();
    for (key, value) in relations.iter() {
        if let Node::Variable(v) = value {
            if let Some(resolved) = relations.get(v) {
                updates.push((key.clone(), resolved.clone()));
                changed = true;
            }
        }
    }
    // Apply updates
    for (key, resolved) in updates {
        if let Some(value) = relations.get_mut(&key) {
            *value = resolved;
        }
    }
        }
        Some(relations)
    } else {
        None
    }
}
// this function matchand binds on a given like node, it doesn't move the node 
// what i need rn is a function that takes this matchandbinds if it return a failure on a given node in the b 
/// On a given node of the ast, it attempts to match if one node can be substituted (subsumpted?)
pub fn matchandassigns(pattern:&Node, target:&Node)->Option<HashMap<char,Node>>{  // this is called basic unification apparently
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
            Ok(((lhs_node, lhs_ops), (rhs_node, _rhs_ops))) => {
                
                if lhs_node == rhs_node { 
                    Ok(Term { term: lhs_node, size: lhs_ops })
                } else {
                    // If lhs_node != rhs_node, it means parse_equality found an assignment
                    // for now only viable for single term not assignements 
                    // 
                     Ok(Term { term: lhs_node, size: lhs_ops })
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
        //assert_eq!(boolt,true)




    }
    #[test]
    fn unificationvariablenumber(){

        let t1 = from_str("x").unwrap();
        let t2 = from_str("0").unwrap();
        println!("x,0 {:?}",unification(&t2.term,&t1.term));
        println!("x,y:{:?}",unification(&t1.term,&t2.term));

        // this works 
    }
    #[test]
    fn unificationvariables(){

        let t1 = from_str("x").unwrap();
        let t2 = from_str("y").unwrap();
        println!("x,y:{:?}",unification(&t2.term,&t1.term));


    }
    #[test]
    fn unificationbinaryop(){
        let t1 = from_str("-x + x").unwrap();
        let t2 = from_str("a + 0 ").unwrap();
        println!("unification of -x + x and a + 0 leads to :{:?}",unification(&t2.term,&t1.term));}
    #[test]
    fn unificationmoreshenanigans(){
        let t1 = from_str("(x + y) * (z - 5)").unwrap();
        let t2 = from_str("(3 + 4) * (w - 5)").unwrap();
       println!("unification of (x + y) * (z - 5) and (3 + 4) * (w - 5) leads to :{:?}",unification(&t2.term,&t1.term));


    
    }
    #[test]
    fn unificationhard (){
    let t1 = from_str("(x + 2) * z").unwrap();
    let t2 = from_str("(y + y   ) *3").unwrap();
       println!("unification of (x + 2) *z and (y + y) * 3  leads to :{:?}",unification(&t2.term,&t1.term));


            


    }
    #[test]
    fn chatgeppittysaidthiswouldntworkbutitdidehehe(){
        let t1 = from_str("x ").unwrap();
        let t2 = from_str("y + y ").unwrap();
       println!("unification of x and y + y  leads to :{:?}",unification(&t2.term,&t1.term));   
    }
    #[test]
    fn chatgeppittysaidthiswouldntworkbutitdidehehehe(){
        let t1 = from_str("x + x").unwrap();
        let t2 = from_str("(y + 1) + (3 + 1)").unwrap();
       println!("unification of x+x and (y + 1) + (3 + 1)  leads to :{:?}",unification(&t2.term,&t1.term));   
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
}


pub fn variable(node:&Node)-> Vec<char> {
    let mut variables:Vec<char> = Vec::new();
    fn rec(node:&Node,mut vars:Vec<char>)->Vec<char>{
        match node {
            Node::Number(_) => {return vars;}

            Node::Variable(char) => {
                if vars.contains(char)==false {
                    vars.push(*char);
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

pub fn unification(pattern:&Node,target:&Node)-> Option<HashMap<char,Node>>{
    let mut relations:HashMap<char,Node> = HashMap::new();
    let mut chars:Vec<char> = Vec::new();
    pub fn fifi(pattern:&Node,target:&Node,relations:&mut HashMap<char,Node>,chars:&mut Vec<char>)->bool{
        // okay now i need to have smt like if a variable is related to another variable then add the char to a list, and if that variable gets related to smt else 
       
        if pattern.same_type(target)== false && target.same_type(pattern)==false {
            return false; 

        }        
       
        else if pattern.same_type(target)==true {
            if let Node::Variable(pattern_char) = pattern{
                let prev = relations.get(pattern_char).cloned(); // ngl idk what i'm doing here 
                if let Some(prev) = prev{
                    if prev == *target {
                        return true 

                    }
                    /*else if let Node::Variable(prev_char) = prev{
                        fifi(&prev,target,relations,chars);


                    }*/
                    
                    else if let Node::Variable(prev_char)   = prev {
                        if let Node::Number(target_number) = target {
                            relations.remove(pattern_char);
                            relations.insert(*pattern_char,Node::Number(*target_number));
                            println!("current hash is {:?}",relations);
                            return true; 


                        }
                    
                        
                        fifi(&  prev,target,relations,chars);


                        
                    }
                    else if let Node::Variable(target_char) = target {

                        fifi(target,&prev,relations,chars);

                        // this is where the issue is , basically 

                    }
                    else {// aka when what i'm binding to 
                        
                        return fifi(&prev, target, relations, chars);
                        // fifi(&prev,target,relations,chars);

                    }
                       
                        


                        


                    
                         // i feel like i need to only return if it's false and do smt else when it's true maybe i'm wrong :/
                }  

                else {
                        // here i'm asking if both are variable and therefore i'm adding both a => b and b=>a :3 
                    if let Node::Variable(target_char) = target{
                        // if they're not already there maybe? but like there is no way i think? (DANGER NEEDS THINKING )
                        chars.push(*pattern_char); 
                        chars.push(*target_char);
                       
                        relations.insert(*pattern_char,Node::Variable(*target_char));
                        relations.insert(*target_char,Node::Variable(*pattern_char));
                        println!("current hash is {:?}",relations);
                        return  true;

                        }
                        else {
                            let mut m = variable(target); 
                            // if all ms are attached to smt already i guess ? 
                            
                            let mut bool:bool = true;
                            /*for k in m {
                                if !relations.contains_key(&k){
                                    bool = false;
                                }
                                


                            }*/
                            if bool {
                             //   let mut node = nodesubst(target, relations); 
                                //  println!("node substitution from {:?} is {:?} and the variables that have stuff in them are: {:?}",target,node.0,chars);
                                if occurs(*pattern_char,&target){
                                    return false; 
                                }
                                else {
                                relations.insert(*pattern_char,target.clone());
                                println!("current hash is {:?}",relations);
                                if m!= []{
                                 chars.push(*pattern_char);
                                }
                               
                                return true; 
                            }
                            }

                            else {
                            if occurs(*pattern_char,&target){
                                return false;
                            }
                            else {
                            relations.insert(*pattern_char,target.clone());
                            println!("current hash is {:?}",relations);
                            return true;}}

                        }

                    }    

                
                }



            
            match pattern {

                Node::Number(value)=> {
                    return(*value == target.get_number().unwrap());


                }
                Node::UnaryOp(_,prhs) => {
                    if let Node::UnaryOp(_,trhs) = target {
                        let rmatch = fifi(prhs,trhs,relations,chars);
                        return rmatch;

                    }
                    else {
                        return false;// i was wondering if i reverse pattern, target here but nah 
                    }
                



                }

                Node::BinaryOp(plhs,_,prhs) => {
                    if let Node::BinaryOp(tlhs,_,trhs) = target { 
                        let lmatch = fifi(plhs,tlhs,relations,chars);
                        if lmatch == false {return false;}
                        let rmatch = fifi(prhs,trhs,relations,chars);
                        if rmatch == false {return false;}
                        return true; 




                    }
                    else {
                        return false; 
                    }




                }
                _ => {return false; }// i guess this is the case for variable?? which i mean fair 

            }
        }



        
        else if target.same_type(pattern) == true && pattern.same_type(target) == false{
            return fifi(target,pattern,relations,chars);


        }
        else {
            return false; 
        }
    
    }
    if fifi(pattern,target,&mut relations,&mut chars){
        for c in chars {
        if let Some(node) =relations.remove(&c){
            let (node,size) = nodesubst(&node, &relations);
            if occurs(c,&node) {
                break // tbh idk what i should od here 
            }
            else {
            relations.insert(c,node);}
            println!("current hash is {:?}",relations);
        }            



        }
        //println!("current variables that are assigned to smt are {:?}",chars);
        Some(relations)
    }
    else {
        None 
    }

}


///takes a pattern as well as a relations related to the pattern, fills the missing variables from the pattern with identity! 
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



///takes 
pub fn unify(pattern:&Node, target:&Node)->Option<HashMap<char,Node>>{
    let mut copy = pattern; 
   
    if let Some(relations )= matchandassigns(pattern,target){

        return Some(relations)

    }
    // :D
    
    else if copy.same_type(&Node::Variable('a'))==false || copy.same_type(&Node::Number(0))==false{
        let mut result:Option<HashMap<char,Node>>;
        
            match copy {
            /*Node::Number(value) => {

            }
            Node::Variable() => {



            }*/
                Node::BinaryOp(lhs,op,rhs) => {
                    if let Some(relations) = unify(lhs,target){
                        if let Some(relations) = free(rhs,relations){
                            return Some(relations)
                        }
                        else{
                            return None
                        }
                        
                    }
                    else if let Some(relations) = unify(rhs,target){
                        if let Some(relations) = free(lhs,relations){
                            return Some(relations)
                        }
                        else{
                            return None
                        }


                    }
                    else {
                       return None


                    }

                }   
                Node::UnaryOp(op,rhs ) => {
                    if let Some(relations) = unify(rhs,target){
                        return Some(relations)

                    }
                    else {
                        return None
                    }


                }
                _ => {
                    return None


                }
            }



        

    }
    else {
        return None
    }


}