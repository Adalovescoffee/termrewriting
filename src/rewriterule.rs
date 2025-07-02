use std::collections::HashMap;
use std::vec;
use crate::lexer::Lexer;
use crate::parser::{Parser, Node, ParserError, Operator};

pub struct Rewrite {
    pub term:Node,
    pub expression:Node,


}
impl Rewrite{     
pub fn rewrite(&self)-> Node{
    let (elhs,erhs) = match self.equalitysides(){
        Some((lhs_node, rhs_node)) =>(lhs_node,rhs_node),
        None => {

            eprintln!("issue around equalityside expression not being right");
            return self.term.clone();

        }
    };
    fn rec(targetnode:&Node,rule_pattern:&Node,subst_pattern:&Node)->Node{
        if let Some(relations) = matchandassigns(rule_pattern,targetnode ){
            return nodesubst(subst_pattern,&relations);


        }
        match targetnode{
            Node::BinaryOp(lhs,op,rhs) => {
                let new_lhs = rec(lhs,rule_pattern,subst_pattern);
                let new_rhs = rec(rhs,rule_pattern,subst_pattern);
                Node::BinaryOp(Box::new(new_lhs),*op,Box::new(new_rhs))


            }

            Node::Number(val) => Node::Number(*val),
            Node::Variable(c) => Node::Variable(*c),

        }

    }

    rec(&self.term,&elhs,&erhs)


}

// this fct obviously turns a node into a string 









pub fn equalitysides(&self)->Option<(Node,Node)>{
    if let Node::BinaryOp(lhs,Operator::Assign ,rhs) = self.expression.clone(){
        return Some((*lhs,*rhs))
    }
    else {

        return None 
    }

}



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







}
//substitues a node 

pub fn nodesubst(snode:&Node,relations:&HashMap<char,Node>)->Node{
    match snode{
        Node::Number(n) =>{

            Node::Number(*n)
        }
        Node::Variable(c) =>{
            if let Some(mnode) = relations.get(c){
                mnode.clone()


            }
            else{


                Node::Variable(*c)
            }


        }
        Node::BinaryOp(lhs,op ,rhs ) => {
            let new_lhs = nodesubst(lhs,relations); 
            let new_rhs = nodesubst(rhs, relations);
            Node::BinaryOp(Box::new(new_lhs),*op,Box::new(new_rhs))


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

    // Helper to create a BinaryOp Node concisely
    fn bin_op(lhs: Node, op: Operator, rhs: Node) -> Node {
        Node::BinaryOp(Box::new(lhs), op, Box::new(rhs))
    }

    #[test]
    fn test_equalitysides() {
        // Test 1: Valid assignment rule
        let rule_expr = bin_op(Node::Variable('x'), Operator::Assign, Node::Number(10));
        let rewrite_rule = Rewrite { term: Node::Number(0), expression: rule_expr }; // Dummy term, focus on expression
        let result = rewrite_rule.equalitysides();
        assert!(result.is_some(), "equalitysides should return Some for a valid assignment");
        let (lhs, rhs) = result.unwrap();
        assert_eq!(lhs, Node::Variable('x'), "LHS should be 'x'");
        assert_eq!(rhs, Node::Number(10), "RHS should be 10");

        // Test 2: Not an assignment (e.g., addition)
        let rule_expr_add = bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b'));
        let rewrite_rule_add = Rewrite { term: Node::Number(0), expression: rule_expr_add };
        let result_add = rewrite_rule_add.equalitysides();
        assert!(result_add.is_none(), "equalitysides should return None for non-assignment");

        // Test 3: Not a binary op (e.g., just a number)
        let rule_expr_num = Node::Number(5);
        let rewrite_rule_num = Rewrite { term: Node::Number(0), expression: rule_expr_num };
        let result_num = rewrite_rule_num.equalitysides();
        assert!(result_num.is_none(), "equalitysides should return None for non-binary op");
    }

    #[test]
    fn test_rewrite_method_basic_match() {
        // Rule: x * 1 = x
        let rule_lhs_pattern = bin_op(Node::Variable('x'), Operator::Multiply, Node::Number(1));
        let rule_rhs_subst = Node::Variable('x');
        let rewrite_rule = Rewrite {
            term: bin_op(Node::Number(5), Operator::Multiply, Node::Number(1)), // Target expression
            expression: bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone()), // The rule
        };

        // Expected: 5
        let expected_expr = Node::Number(5);
        let rewritten_expr = rewrite_rule.rewrite(); // Call the public rewrite method
        println!("Test: basic_match");
        println!("  Target: {}", rewrite_rule.term);
        println!("  Rule: {}", rewrite_rule.expression);
        println!("  Rewritten: {}", rewritten_expr);
        println!("  Expected: {}", expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Rule 'x*1=x' should transform '5*1' to '5'");

        // Target: (a + b) * 1
        let target_expr_complex = bin_op(
            bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b')),
            Operator::Multiply,
            Node::Number(1)
        );
        let rewrite_rule_complex = Rewrite {
            term: target_expr_complex.clone(), // Target expression
            expression: bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone()), // The rule
        };
        let expected_expr_complex = bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b'));
        let rewritten_expr_complex = rewrite_rule_complex.rewrite();
        println!("Test: basic_match_complex");
        println!("  Target: {}", target_expr_complex);
        println!("  Rule: {}", rewrite_rule_complex.expression);
        println!("  Rewritten: {}", rewritten_expr_complex);
        println!("  Expected: {}", expected_expr_complex);
        assert_eq!(rewritten_expr_complex, expected_expr_complex, "Rule 'x*1=x' should transform '(a+b)*1' to '(a+b)'");
    }

    #[test]
    fn test_rewrite_method_no_match() {
        // Rule: x * 1 = x
        let rule_lhs_pattern = bin_op(Node::Variable('x'), Operator::Multiply, Node::Number(1));
        let rule_rhs_subst = Node::Variable('x');
        let rewrite_rule = Rewrite {
            term: bin_op(Node::Number(5), Operator::Add, Node::Number(2)), // Target: 5 + 2 (no match for x*1)
            expression: bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone()), // The rule
        };
        let expected_expr = bin_op(Node::Number(5), Operator::Add, Node::Number(2)); // Should remain unchanged
        let rewritten_expr = rewrite_rule.rewrite();
        println!("Test: no_match");
        println!("  Target: {}", rewrite_rule.term);
        println!("  Rule: {}", rewrite_rule.expression);
        println!("  Rewritten: {}", rewritten_expr);
        println!("  Expected: {}", expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Rule should not apply, expression should be unchanged");

        // Target: 5 * 2 (mismatched literal)
        let target_expr_mismatch = bin_op(Node::Number(5), Operator::Multiply, Node::Number(2));
        let rewrite_rule_mismatch = Rewrite {
            term: target_expr_mismatch.clone(), // Target expression
            expression: bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone()), // The rule
        };
        let expected_expr_mismatch = target_expr_mismatch.clone(); // Should remain unchanged
        let rewritten_expr_mismatch = rewrite_rule_mismatch.rewrite();
        println!("Test: no_match_mismatch_literal");
        println!("  Target: {}", rewrite_rule_mismatch.term);
        println!("  Rule: {}", rewrite_rule_mismatch.expression);
        println!("  Rewritten: {}", rewritten_expr_mismatch);
        println!("  Expected: {}", expected_expr_mismatch);
        assert_eq!(rewritten_expr_mismatch, expected_expr_mismatch, "Rule should not apply, expression should be unchanged");
    }

    #[test]
    fn test_rewrite_method_deep_match() {
        // Rule: x * 1 = x
        let rule_lhs_pattern = bin_op(Node::Variable('x'), Operator::Multiply, Node::Number(1));
        let rule_rhs_subst = Node::Variable('x');
        let rewrite_rule = Rewrite {
            term: bin_op( // Target: (3 + (5 * 1)) - 2
                bin_op(Node::Number(3), Operator::Add, bin_op(Node::Number(5), Operator::Multiply, Node::Number(1))),
                Operator::Subtract,
                Node::Number(2)
            ),
            expression: bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone()), // The rule
        };
        // Expected: (3 + 5) - 2
        let expected_expr = bin_op(
            bin_op(Node::Number(3), Operator::Add, Node::Number(5)),
            Operator::Subtract,
            Node::Number(2)
        );
        let rewritten_expr = rewrite_rule.rewrite();
        println!("Test: deep_match");
        println!("  Target: {}", rewrite_rule.term);
        println!("  Rule: {}", rewrite_rule.expression);
        println!("  Rewritten: {}", rewritten_expr);
        println!("  Expected: {}", expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Rule should apply deeply: (3 + (5*1)) - 2 -> (3 + 5) - 2");
    }

    #[test]
    fn test_rewrite_method_repeated_variable_rule() {
        // Rule: x + x = 2 * x
        let rule_lhs_pattern = bin_op(Node::Variable('x'), Operator::Add, Node::Variable('x'));
        let rule_rhs_subst = bin_op(Node::Number(2), Operator::Multiply, Node::Variable('x'));
        let rewrite_rule = Rewrite {
            term: bin_op( // Target: (a * b) + (a * b)
                bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b')),
                Operator::Add,
                bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b'))
            ),
            expression: bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone()), // The rule
        };

        let subtree = bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b'));
        let expected_expr = bin_op(Node::Number(2), Operator::Multiply, subtree.clone()); // 2 * (a * b)
        let rewritten_expr = rewrite_rule.rewrite();
        println!("Test: repeated_variable_rule");
        println!("  Target: {}", rewrite_rule.term);
        println!("  Rule: {}", rewrite_rule.expression);
        println!("  Rewritten: {}", rewritten_expr);
        println!("  Expected: {}", expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Rule 'x+x=2*x' should transform '(a*b)+(a*b)' to '2*(a*b)'");

        // Target: (a * b) + (a + b) (inconsistent 'x' binding)
        let target_lhs_child = bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b'));
        let target_rhs_child = bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b'));
        let rewrite_rule_inconsistent = Rewrite {
            term: bin_op(target_lhs_child, Operator::Add, target_rhs_child),
            expression: bin_op(rule_lhs_pattern.clone(), Operator::Assign, rule_rhs_subst.clone()), // The rule
        };
        let expected_expr_inconsistent = rewrite_rule_inconsistent.term.clone(); // Should remain unchanged
        let rewritten_expr_inconsistent = rewrite_rule_inconsistent.rewrite();
        println!("Test: repeated_variable_rule_inconsistent");
        println!("  Target: {}", rewrite_rule_inconsistent.term);
        println!("  Rule: {}", rewrite_rule_inconsistent.expression);
        println!("  Rewritten: {}", rewritten_expr_inconsistent);
        println!("  Expected: {}", expected_expr_inconsistent);
        assert_eq!(rewritten_expr_inconsistent, expected_expr_inconsistent, "Rule should NOT apply due to inconsistent 'x' binding");
    }

    #[test]
    fn test_rewrite_method_rule_malformed() {
        // Rule: Just a number (not an assignment)
        let rewrite_rule = Rewrite { term: bin_op(Node::Number(1), Operator::Add, Node::Number(2)), expression: Node::Number(5) }; // expression is not an assignment
        let expected_expr = rewrite_rule.term.clone(); // Should return original term
        let rewritten_expr = rewrite_rule.rewrite();
        println!("Test: rule_malformed");
        println!("  Target: {}", rewrite_rule.term);
        println!("  Rule: {}", rewrite_rule.expression);
        println!("  Rewritten: {}", rewritten_expr);
        println!("  Expected: {}", expected_expr);
        assert_eq!(rewritten_expr, expected_expr, "Malformed rule should not apply, return original expression");
    }
}


