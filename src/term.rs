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
pub fn rewriteby(&self, law:((&Node,i16),(&Node,i16)))-> (Node,i16){
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

    rec(&self.term,&lhs_node,&rhs_node)


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
  

/* 
#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display; // Import Display trait for println!

    // Helper for binaryop
    fn bin_op(lhs: Node, op: Operator, rhs: Node) -> Node {
        Node::BinaryOp(Box::new(lhs), op, Box::new(rhs))
    }

    // Helper to print debug info for failing tests
    fn print_debug_info<T: Display>(test_name: &str, term: &Node, rule: &Node, rewritten: &Node, expected: &T) {
        println!("\n Test: {} ", test_name);
        println!("  Term (Input): {}", term);
        println!("  Rule: {}", rule);
        println!("  Rewritten: {}", rewritten);
        println!("  Expected: {}", expected);
       // println!("  Raw Rewritten (Debug): {:?}", rewritten);
       // println!("  Raw Expected (Debug): {:?}", expected);
    }

    #[test]
    fn test_equalitysides_standalone() {
        // Test 1: Valid assignment rule
        let rule_expr = bin_op(Node::Variable('x'), Operator::Assign, Node::Number(10));
        let result = equalitysides(rule_expr.clone()); // Clone for the function call
        assert!(result.is_some(), "equalitysides should return Some for a valid assignment");
        let (lhs, rhs) = result.unwrap();
        assert_eq!(lhs, Node::Variable('x'), "LHS should be 'x'");
        assert_eq!(rhs, Node::Number(10), "RHS should be 10");

        // Test 2: Not an assignment (e.g., addition)
        let rule_expr_add = bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b'));
        let result_add = equalitysides(rule_expr_add.clone());
        assert!(result_add.is_none(), "equalitysides should return None for non-assignment");

        // Test 3: Not a binary op (e.g., just a number)
        let rule_expr_num = Node::Number(5);
        let result_num = equalitysides(rule_expr_num.clone());
        assert!(result_num.is_none(), "equalitysides should return None for non-binary op");
    }

    #[test]
    fn test_term_rewriteby_basic_match() {
        // Rule: x * 1 = x
        let rule_lhs_pattern = bin_op(Node::Variable('x'), Operator::Multiply, Node::Number(1));
        let rule_rhs_subst = Node::Variable('x');
        let rule_node = bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone());

        let term_instance = Term {
            term: bin_op(Node::Number(5), Operator::Multiply, Node::Number(1)), // Target expression
        };

        let expected_expr = Node::Number(5);
        let rewritten_expr = term_instance.rewriteby(rule_node.clone());
        print_debug_info("basic_match", &term_instance.term, &rule_node, &rewritten_expr, &expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Rule 'x*1=x' should transform '5*1' to '5'");

        // Target: (a + b) * 1
        let target_expr_complex = bin_op(
            bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b')),
            Operator::Multiply,
            Node::Number(1)
        );
        let term_instance_complex = Term {
            term: target_expr_complex.clone(), // Target expression
        };
        let expected_expr_complex = bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b'));
        let rewritten_expr_complex = term_instance_complex.rewriteby(rule_node.clone());
        print_debug_info("basic_match_complex", &target_expr_complex, &rule_node, &rewritten_expr_complex, &expected_expr_complex);
        assert_eq!(rewritten_expr_complex, expected_expr_complex, "Rule 'x*1=x' should transform '(a+b)*1' to '(a+b)'");
    }

    #[test]
    fn test_term_rewriteby_no_match() {
        // Rule: x * 1 = x
        let rule_lhs_pattern = bin_op(Node::Variable('x'), Operator::Multiply, Node::Number(1));
        let rule_rhs_subst = Node::Variable('x');
        let rule_node = bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone());

        let term_instance = Term {
            term: bin_op(Node::Number(5), Operator::Add, Node::Number(2)), // Target: 5 + 2 (no match for x*1)
        };
        let expected_expr = bin_op(Node::Number(5), Operator::Add, Node::Number(2));
        let rewritten_expr = term_instance.rewriteby(rule_node.clone());
        print_debug_info("no_match", &term_instance.term, &rule_node, &rewritten_expr, &expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Rule should not apply, expression should be unchanged");

        // Target: 5 * 2 (mismatched literal)
        let target_expr_mismatch = bin_op(Node::Number(5), Operator::Multiply, Node::Number(2));
        let term_instance_mismatch = Term {
            term: target_expr_mismatch.clone(),
        };
        let expected_expr_mismatch = target_expr_mismatch.clone();
        let rewritten_expr_mismatch = term_instance_mismatch.rewriteby(rule_node.clone());
        print_debug_info("no_match_mismatch_literal", &target_expr_mismatch, &rule_node, &rewritten_expr_mismatch, &expected_expr_mismatch);
        assert_eq!(rewritten_expr_mismatch, expected_expr_mismatch, "Rule should not apply, expression should be unchanged");
    }

    #[test]
    fn test_term_rewriteby_deep_match() {
        // Rule: x * 1 = x
        let rule_lhs_pattern = bin_op(Node::Variable('x'), Operator::Multiply, Node::Number(1));
        let rule_rhs_subst = Node::Variable('x');
        let rule_node = bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone());

        let term_instance = Term {
            term: bin_op( // Target: (3 + (5 * 1)) - 2
                bin_op(Node::Number(3), Operator::Add, bin_op(Node::Number(5), Operator::Multiply, Node::Number(1))),
                Operator::Subtract,
                Node::Number(2)
            ),
        };
        let expected_expr = bin_op(
            bin_op(Node::Number(3), Operator::Add, Node::Number(5)),
            Operator::Subtract,
            Node::Number(2)
        );
        let rewritten_expr = term_instance.rewriteby(rule_node.clone());
        print_debug_info("deep_match", &term_instance.term, &rule_node, &rewritten_expr, &expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Rule should apply deeply: (3 + (5*1)) - 2 -> (3 + 5) - 2");
    }

    #[test]
    fn test_term_rewriteby_repeated_variable_rule() {
        // Rule: x + x = 2 * x
        let rule_lhs_pattern = bin_op(Node::Variable('x'), Operator::Add, Node::Variable('x'));
        let rule_rhs_subst = bin_op(Node::Number(2), Operator::Multiply, Node::Variable('x'));
        let rule_node = bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone());

        let term_instance = Term {
            term: bin_op( // Target: (a * b) + (a * b)
                bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b')),
                Operator::Add,
                bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b'))
            ),
        };

        let subtree = bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b'));
        let expected_expr = bin_op(Node::Number(2), Operator::Multiply, subtree.clone());
        let rewritten_expr = term_instance.rewriteby(rule_node.clone());
        print_debug_info("repeated_variable_rule", &term_instance.term, &rule_node, &rewritten_expr, &expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Rule 'x+x=2*x' should transform '(a*b)+(a*b)' to '2*(a*b)'");

        // Target: (a * b) + (a + b) (inconsistent 'x' binding)
        let target_lhs_child = bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b'));
        let target_rhs_child = bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b'));
        let term_instance_inconsistent = Term {
            term: bin_op(target_lhs_child, Operator::Add, target_rhs_child),
        };
        let expected_expr_inconsistent = term_instance_inconsistent.term.clone();
        let rewritten_expr_inconsistent = term_instance_inconsistent.rewriteby(rule_node.clone());
        print_debug_info("repeated_variable_rule_inconsistent", &term_instance_inconsistent.term, &rule_node, &rewritten_expr_inconsistent, &expected_expr_inconsistent);
        assert_eq!(rewritten_expr_inconsistent, expected_expr_inconsistent, "Rule should NOT apply due to inconsistent 'x' binding");
    }

    #[test]
    fn test_term_rewriteby_rule_malformed() {
        // Rule: Just a number (not an assignment)
        let malformed_rule = Node::Number(5);
        let term_instance = Term { term: bin_op(Node::Number(1), Operator::Add, Node::Number(2)) };
        let expected_expr = term_instance.term.clone();
        let rewritten_expr = term_instance.rewriteby(malformed_rule.clone());
        print_debug_info("rule_malformed", &term_instance.term, &malformed_rule, &rewritten_expr, &expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Malformed rule should not apply, return original expression");
    }
}



*/