mod lexer;
mod parser;
mod term;
use lexer::Lexer;
use parser::{Parser, Node,ParserError}; // Import what you need from parser


use crate::term::Term;


fn main() {
    // Test expressions
    /*let expressions = vec![ 
        "1 + 2 * 3",
        "5 - x / y",
        "(2 + 3) * 4",
        "a+b + (c+d)", 
        "10 * (x + 5)",
        "x + y * z - 1",
        "2(x + 3)", 
        "x*1 = x",
        "a*b = b*a",
        "a*a*a  = a*a",
        "a = b + 1", 
        "x == y",    
        "123",       
        "z",         
        "5 +",       
        "3 * (4 + 5",// Error case: missing parenthesis
        "# invalid", // Error case: illegal character
    ];
    */
    let input1 = "a +b*1".to_string();
    let input2 = "a*1=a".to_string();

    let lexer1 = Lexer::new(input1);
    let lexer2 = Lexer::new(input2);
    let mut parser1 = Parser::new(lexer1);
    let mut parser2 = Parser::new(lexer2);
    let size:i16 = 0; 
    let node1 = match parser1.parse_equality(){
        Ok(node) =>{node 
            


        }
        Err(e)=>{
        eprintln!("Error parsing term \"{}\": {:?}", "(2*3)+4", e);
        ((Node::Variable('f'),0),(Node::Variable('f'),0))

    }};
    
     
    println!("this term \"{}\" has \"{}\" operation",node1.0.0,node1.0.1.to_string());
    let node2 = match parser2.parse_equality(){
        Ok(((n1,size1),(n2,size2))) =>{((n1,size1),(n2,size2) )
            


        }
        Err(e)=>{
        eprintln!("Error parsing term \"{}\": {:?}", "a*b = b*a", e);
       ((Node::Variable('f'),size),(Node::Variable('f'),size))
        }

    };
     
        let term1 = Term{term: node1.0.0,size:node1.0.1 };
          
        let rewrittenterm = term1.rewriteby(((&node2.0.0,node2.0.1),(&node2.1.0,node2.1.1)));
        let term2 = Term{term:node2.0.0,size:node2.0.1};  
        println!("By the law \"{}\", \"{}\" => \"{}\"", term2, term1, rewrittenterm);
    

    let rewrite_examples = vec![
        // Example 1: Identity for Addition (x + 0 = x)
        ("a + 0", "x + 0 = x"),
        // Example 2: Commutativity of Multiplication (a * b = b * a)
        ("3 * x", "a * b = b * a"),
        // Example 3: Distributivity (a * (b + c) = (a * b) + (a * c))
        ("x * (y + z)", "a * (b + c) = (a * b) + (a * c)"),
        // Example 4: Nested Application (2 * (x + 0))
        ("2 * (x + 0)", "y + 0 = y"), // Should transform 2 * (x + 0) to 2 * x
        // Example 5: Rule Not Applicable (no match)
        ("a + b", "x * 0 = 0"),
        // Example 6: More Complex Distributivity ( (a + b) * c = (a * c) + (b * c) )
        ("(x + y) * z", "(a + b) * c = (a * c) + (b * c)"),
        // Example 7: Double Application (e.g., identity twice)
        ("(x + 0) + 0", "a + 0 = a"), // Should apply twice
        // Example 8: Another Commutativity (a + b = b + a)
        ("3 + x", "a + b = b + a"),
        // Example 9: Zero Multiplication (x * 0 = 0)
        ("y * 0", "x * 0 = 0"),
        // Example 10: Complex expression with multiple potential matches (only one applies per pass)
        ("(a + 0) * (b + 0)", "x + 0 = x"), // Should simplify both sides
    ];
    for (i, (term_str, rule_str)) in rewrite_examples.into_iter().enumerate() {
        println!("--- Rewriting Example {} ---", i + 1);

        
        let (term_node, term_ops) = match parse_input_to_rule_tuple(term_str) {
            Ok(((node, ops), _)) => (node, ops), 
            Err(e) => {
                eprintln!("Error parsing term \"{}\": {:?}", term_str, e);
                continue;
            }
        };
        
        let term_instance = Term { term: term_node.clone(), size: term_ops };

       
        let (rule_pattern_tuple, rule_subst_tuple) = match parse_input_to_rule_tuple(rule_str) {
            Ok(rule_tuple) => rule_tuple,
            Err(e) => {
                eprintln!("Error parsing rule \"{}\": {:?}", rule_str, e);
                continue;
            }
        };

        let law_for_rewriteby = (
            (&rule_pattern_tuple.0, rule_pattern_tuple.1), // (&Node, i16) for pattern
            (&rule_subst_tuple.0, rule_subst_tuple.1),     // (&Node, i16) for substitution
        );
        let rewritten_term_instance = term_instance.rewriteby(law_for_rewriteby);

        println!("  By the law \"{}\", \"{}\" => \"{}\"",
                 rule_str,
                 term_instance, 
                 rewritten_term_instance); 
        println!(""); 
    }
}
   
    
fn parse_input_to_rule_tuple(input_str: &str) -> Result<((Node, i16), (Node, i16)), ParserError> {
    let lexer = Lexer::new(input_str.to_string());
    let mut parser = Parser::new(lexer);
   
    parser.parse_equality()
}