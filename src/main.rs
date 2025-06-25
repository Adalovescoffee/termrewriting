mod lexer;
mod parser;
mod rewriterule;
use lexer::Lexer;
use parser::{Parser, Node, ParserError}; // Import what you need from parser

fn main() {
    // Test expressions
    let expressions = vec![ 
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

    for (i, expr_str) in expressions.into_iter().enumerate() {
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
}