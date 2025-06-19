mod parser;
mod lexer;
use lexer::Lexer;
use parser::{Parser, Node, ParserError};
fn main() {
    // Test expressions
    let expressions = vec![
        "1 + 2 * 3",
        "5 - x / y",
        "(2 + 3) * 4",
        "a b + c d", // Implicit multiplication: a*b + c*d
        "10 * (x + 5)",
        "x + y * z - 1",
        "2(x + 3)", // Implicit multiplication: 2*(x+3)
        "a = b + 1", // Example with assignment 
        "x == y",    // Example with equality 
        "123",       // Single number
        "z",         // Single variable
        "5 +",       // Error case: incomplete expression
        "3 * (4 + 5",// Error case: missing parenthesis
        "# invalid", // Error case: illegal character
        "ab"
    ];

    for (i, expr_str) in expressions.into_iter().enumerate() {
        println!("\n--- Parsing Expression {} ---", i + 1);
        println!("Input: \"{}\"", expr_str);

        let input = expr_str.to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        match parser.parse_term() {
            Ok(ast) => {
                println!("Successfully Parsed AST: {:?}", ast);
            },
            Err(e) => {
                eprintln!("Parsing Error: {:?}", e);
            }
        }
    }
}

