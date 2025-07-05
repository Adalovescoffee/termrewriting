mod lexer;
mod parser;
mod term;
use lexer::Lexer;
use parser::{Parser, Node}; // Import what you need from parser


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
    let input1 = "(2*3)+4".to_string();
    let input2 = "a*b=b*a".to_string();
    
    let lexer1 = Lexer::new(input1);
    let lexer2 = Lexer::new(input2);
    let mut parser1 = Parser::new(lexer1);
    let mut parser2 = Parser::new(lexer2);
    let node1 = match parser1.parse_equality(){
        Ok(node) =>{node 
            


        }
        Err(e)=>{
        eprintln!("Error parsing term \"{}\": {:?}", "(2*3)+4", e);
        Node::Number(0)

    }};
    let node2 = match parser2.parse_equality(){
        Ok(node) =>{node 
            


        }
        Err(e)=>{
        eprintln!("Error parsing term \"{}\": {:?}", "a*b = b*a", e);
        Node::Number(0)
        }

    };
        let term = Term{term: node1.clone()};
        println!("By the law \"{}\", \"{}\" => \"\"{}\"", node2.to_string(), node1.to_string(), term.rewriteby(node2).to_string());
    

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

        // Parse the term (expression to be rewritten)
        let mut term_parser = Parser::new(Lexer::new(term_str.to_string()));
        let term_node = match term_parser.parse_equality() { // Use parse_equality
            Ok(node) => node,
            Err(e) => {
                eprintln!("Error parsing term \"{}\": {:?}", term_str, e);
                continue;
            }
        };
        
        // Parse the rule (LHS = RHS)
        let mut rule_parser = Parser::new(Lexer::new(rule_str.to_string()));
        let rule_node = match rule_parser.parse_equality() { // Use parse_equality
            Ok(node) => node,
            Err(e) => {
                eprintln!("Error parsing rule \"{}\": {:?}", rule_str, e);
                continue;
            }
        };
        let a = rule_node.to_string();
        // Create the Rewrite instance
        let rewrite_instance = Term { term: term_node.clone()};

        // Perform the rewrite
        let rewritten_term = rewrite_instance.rewriteby(rule_node);

        // Print the result
        println!("  By the law \"{}\", \"{}\" => \"{}\"",
                 a,
                 term_node.to_string(),
                 rewritten_term.to_string());
        println!(""); // Add a newline for spacing
    }
}
    /*for (i, expr_str) in expressions.into_iter().enumerate() {
        println!("\n--- Parsing Expression {} ---", i + 1);
        println!("Input: \"{}\"", expr_str);

        let input = expr_str.to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        // Attempt to parse the expression
        // We call parse_expression, as it's the top-level for arithmetic expressions
        match parser.parse_equality() {
            Ok(ast) => {
                
                println!("Successfully Parsed AST: {:?}", ast);
                // Here, you would typically evaluate the AST or perform term rewriting
                // For example: println!("Evaluated result: {}", evaluate_ast(&ast));
            },
            Err(e) => {
                eprintln!("Parsing Error: {:?}", e);
            }
        }
    }
}*/