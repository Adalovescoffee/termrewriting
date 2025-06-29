use std::collections::HashMap;
use std::vec;
use crate::lexer::Lexer;
use crate::parser::{Parser, Node, ParserError, Operator};

pub struct Rewrite {
    term:Node,
    expression:Node,


}
impl Rewrite{     

pub fn rewrite(){

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

// this function matchand binds on a given like node, it doesn't move the node 
// what i need rn is a function that takes this matchandbinds if it return a failure on a given node in the 
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

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module (Node, Operator, Rewrite, matchandbinds)

    // Helper to create a BinaryOp Node concisely
    fn bin_op(lhs: Node, op: Operator, rhs: Node) -> Node {
        Node::BinaryOp(Box::new(lhs), op, Box::new(rhs))
    }

    #[test]
    fn test_matchandbinds_number_literals() {
        let pattern_node = Node::Number(5); // Direct pattern node

        // Test 1: Exact value match
        let target1 = Node::Number(5);
        let mut relations1 = HashMap::new();
        assert!(matchandbinds(&pattern_node, &target1, &mut relations1), "Should match 5 to 5");
        assert!(relations1.is_empty(), "No variables, so relations should be empty");

        // Test 2: Mismatched literal value
        let target2 = Node::Number(10);
        let mut relations2 = HashMap::new();
        assert!(!matchandbinds(&pattern_node, &target2, &mut relations2), "Should NOT match 5 to 10 (value mismatch)");
        assert!(relations2.is_empty(), "No match, no bindings");

        // Test 3: Mismatched type (Number vs Variable) - handled by same_type
        let target3 = Node::Variable('x');
        let mut relations3 = HashMap::new();
        assert!(!matchandbinds(&pattern_node, &target3, &mut relations3), "Should NOT match Number to Variable (type mismatch)");
        assert!(relations3.is_empty(), "No match, no bindings");
    }

    #[test]
    fn test_matchandbinds_variable_wildcard() {
        // Pattern: 'x' (acts as a wildcard due to same_type definition)
        let pattern_node = Node::Variable('x'); // Direct pattern node

        // Test 1: Variable matches a Number
        let target1 = Node::Number(10);
        let mut relations1 = HashMap::new();
        assert!(matchandbinds(&pattern_node, &target1, &mut relations1), "Pattern 'x' should match Number 10 (wildcard)");
        assert_eq!(relations1.get(&'x'), Some(&Node::Number(10)), "Binding 'x' to Number(10)");

        // Test 2: Variable matches another Variable
        let target2 = Node::Variable('y');
        let mut relations2 = HashMap::new();
        assert!(matchandbinds(&pattern_node, &target2, &mut relations2), "Pattern 'x' should match Variable 'y' (wildcard)");
        assert_eq!(relations2.get(&'x'), Some(&Node::Variable('y')), "Binding 'x' to Variable('y')");

        // Test 3: Variable matches a BinaryOp
        let target3 = bin_op(Node::Number(1), Operator::Add, Node::Number(2)); // (1 + 2)
        let mut relations3 = HashMap::new();
        assert!(matchandbinds(&pattern_node, &target3, &mut relations3), "Pattern 'x' should match BinaryOp (1+2) (wildcard)");
        assert_eq!(relations3.get(&'x'), Some(&target3), "Binding 'x' to (1+2)");
    }

    #[test]
    fn test_matchandbinds_binary_op_structure_and_operator() {
        // Pattern: (a + b)
        let pattern_node = bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b'));

        // Test 1: Matching operator and type, then successful recursive match of children
        // Pattern: (a + b)
        // Target:  (10 + 20)
        let target1 = bin_op(Node::Number(10), Operator::Add, Node::Number(20));
        let mut relations1 = HashMap::new();
        assert!(matchandbinds(&pattern_node, &target1, &mut relations1), "Should match (a+b) to (10+20)");
        assert_eq!(relations1.get(&'a'), Some(&Node::Number(10)), "Binding 'a' to 10");
        assert_eq!(relations1.get(&'b'), Some(&Node::Number(20)), "Binding 'b' to 20");

        // Test 2: Mismatched operator (a * b)
        // Pattern: (a + b)
        // Target:  (10 * 20)
        let target2 = bin_op(Node::Number(10), Operator::Multiply, Node::Number(20));
        let mut relations2 = HashMap::new();
        assert!(!matchandbinds(&pattern_node, &target2, &mut relations2), "Should NOT match due to operator mismatch ('Add' vs 'Multiply')");
        assert!(relations2.is_empty(), "No match, no bindings");
    }

    #[test]
    fn test_matchandbinds_recursive_pattern_complex() {
        // Pattern: (x + 1) * y
        let pattern_node = bin_op(bin_op(Node::Variable('x'), Operator::Add, Node::Number(1)), Operator::Multiply, Node::Variable('y'));

        // Target: (5 + 1) * z
        let target1 = bin_op(bin_op(Node::Number(5), Operator::Add, Node::Number(1)), Operator::Multiply, Node::Variable('z'));
        let mut relations1 = HashMap::new();
        assert!(matchandbinds(&pattern_node, &target1, &mut relations1), "Should match (x+1)*y to (5+1)*z");
        assert_eq!(relations1.get(&'x'), Some(&Node::Number(5)), "Binding 'x' to 5");
        assert_eq!(relations1.get(&'y'), Some(&Node::Variable('z')), "Binding 'y' to 'z'");

        // Target: (5 + 2) * z (mismatched literal in LHS child)
        let target2 = bin_op(bin_op(Node::Number(5), Operator::Add, Node::Number(2)), Operator::Multiply, Node::Variable('z'));
        let mut relations2 = HashMap::new();
        assert!(!matchandbinds(&pattern_node, &target2, &mut relations2), "Should NOT match (x+1)*y to (5+2)*z due to 1 != 2");
        assert_eq!(relations2.get(&'x'), Some(&Node::Number(5)), "x should have tried to bind to 5");
        assert!(relations2.get(&'y').is_none(), "y should not bind if overall fails");

        // Target: (5 * 1) * z (mismatched operator in LHS child)
        let target3 = bin_op(bin_op(Node::Number(5), Operator::Multiply, Node::Number(1)), Operator::Multiply, Node::Variable('z'));
        let mut relations3 = HashMap::new();
        assert!(!matchandbinds(&pattern_node, &target3, &mut relations3), "Should NOT match due to nested operator mismatch (Add vs Multiply)");
        assert!(relations3.is_empty(), "No match, so bindings should be empty");
    }

    #[test]
    fn test_matchandbinds_repeated_variables_consistency() {
        // Rule: x + x = ... (x appears twice in the pattern)
        let pattern_node = bin_op(Node::Variable('x'), Operator::Add, Node::Variable('x'));

        // Test 1: Consistent match (a*b) + (a*b)
        let subtree = bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b'));
        let target1 = bin_op(subtree.clone(), Operator::Add, subtree.clone());
        let mut relations1 = HashMap::new();
        assert!(matchandbinds(&pattern_node, &target1, &mut relations1), "Should match (a*b) + (a*b) consistently");
        assert_eq!(relations1.get(&'x'), Some(&subtree), "Binding 'x' to (a*b)");

        // Test 2: Inconsistent match (a*b) + (a+b) - 'x' binds to different things
        let target2_lhs_child = bin_op(Node::Variable('a'), Operator::Multiply, Node::Variable('b'));
        let target2_rhs_child = bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b'));
        let target2 = bin_op(target2_lhs_child.clone(), Operator::Add, target2_rhs_child.clone());
        let mut relations2 = HashMap::new();
        assert!(!matchandbinds(&pattern_node, &target2, &mut relations2), "Should NOT match due to inconsistent 'x' binding");
        // Relations will have 'x' bound to `target2_lhs_child` initially, then it fails.
        assert_eq!(relations2.get(&'x'), Some(&target2_lhs_child), "x should have been bound to the first child, then failed consistency check.");
    }
}