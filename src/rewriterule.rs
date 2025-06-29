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



#[cfg(test)]
mod tests {
    use crate::parser::Operator;

    use super::*; // Import everything from the parent module (Node, Operator, Rewrite)

    // Helper to create a BinaryOp Node concisely
    fn bin_op(lhs: Node, op: Operator, rhs: Node) -> Node {
        Node::BinaryOp(Box::new(lhs), op, Box::new(rhs))
    }

    #[test]
    fn test_canmatch_number_literals() {
        // Pattern: 5
        let rewrite_rule = Rewrite { term: Node::Number(5), expression: Node::Number(5) }; // expression field is unused for canmatch

        // Test 1: Exact match
        let target1 = Node::Number(5);
        assert!(rewrite_rule.canmatch(&target1), "Should match 5 to 5");

        // Test 2: Mismatched literal value
        let target2 = Node::Number(10);
        assert!(!rewrite_rule.canmatch(&target2), "Should NOT match 5 to 10");

        // Test 3: Mismatched type
        let target3 = Node::Variable('x');
        assert!(!rewrite_rule.canmatch(&target3), "Should NOT match Number to Variable (due to same_type)");
    }

    #[test]
    fn test_canmatch_variable_pattern_strict_type() {
        // Pattern: Variable('x')
        let rewrite_rule = Rewrite { term: Node::Variable('x'), expression: Node::Variable('x') };

        // Test 1: Target is another Variable (matches same_type)
        let target1 = Node::Variable('y');
        assert!(rewrite_rule.canmatch(&target1), "Pattern 'x' should match Variable 'y' (same_type)");

        // Test 2: Target is a Number (DOES NOT match same_type)
        let target2 = Node::Number(10);
        assert!(!rewrite_rule.canmatch(&target2), "Pattern 'x' should NOT match Number 10 (diff type)");

        // Test 3: Target is a BinaryOp (DOES NOT match same_type)
        let target3 = bin_op(Node::Number(1), Operator::Add, Node::Number(2));
        assert!(!rewrite_rule.canmatch(&target3), "Pattern 'x' should NOT match BinaryOp (diff type)");
    }

    #[test]
    fn test_canmatch_binary_op_recursive_issues() {
        // Pattern: a + b
        let rewrite_rule = Rewrite {
            term: bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b')),
            expression: Node::Variable('a'), // RHS is irrelevant for canmatch
        };

        // Test 1: Target: (10 + 20)
        // Expected: Should match as per your `if let` (op matches) and recursive calls (which are flawed).
        // Actual Behavior: For a + b pattern, it checks if `other` is a BinaryOp and operator is Add.
        // THEN, it calls `self.canmatch(10)` and `self.canmatch(20)`.
        // This is trying to match `(a+b)` against `10` AND `(a+b)` against `20`.
        // Both these sub-matches will fail the initial `same_type` check, resulting in `false`.
        let target1 = bin_op(Node::Number(10), Operator::Add, Node::Number(20));
        assert!(!rewrite_rule.canmatch(&target1), "Should NOT match due to incorrect recursion logic.");


        // Test 2: Mismatched operator (a * b)
        // Pattern: a + b
        // Target:  10 * 20
        let target2 = bin_op(Node::Number(10), Operator::Multiply, Node::Number(20));
        // The initial `same_type` check passes (both are BinaryOp).
        // The `if let Node::BinaryOp(tlhs,op_self,trhs)= other` will succeed as types match.
        // However, `op_self` (from pattern `Add`) will compare `!=` `op_other` (from target `Multiply`)
        // at the point of implicit comparison, causing the `if let` to fail and return false.
        assert!(!rewrite_rule.canmatch(&target2), "Should NOT match due to operator mismatch");

        // Test 3: Complex nested scenario - pattern matches target's children
        // Pattern: (x + 1) * y
        // Target:  ( (a*b) + (c*d) ) * ( (e-f) + (g/h) )
        //
        // Your `canmatch` recursive calls (`self.canmatch(tlhs) && self.canmatch(trhs)`)
        // would try to match `(x+1)*y` against `(a*b)+(c*d)` AND `(x+1)*y` against `(e-f)+(g/h)`.
        // These will quickly fail at the root `same_type` check, as `(x+1)*y` is BinaryOp(Multiply)
        // but its children are BinaryOp(Add), so the types won't match.
        let rewrite_rule_complex = Rewrite {
            term: bin_op(bin_op(Node::Variable('x'), Operator::Add, Node::Number(1)), Operator::Multiply, Node::Variable('y')),
            expression: Node::Number(0),
        };
        let target_complex = bin_op(
            bin_op(Node::Variable('a'), Operator::Add, Node::Variable('b')),
            Operator::Multiply,
            bin_op(Node::Variable('c'), Operator::Substract, Node::Variable('d'))
        );
        assert!(!rewrite_rule_complex.canmatch(&target_complex), "Should NOT match due to incorrect recursive logic.");
    }
}