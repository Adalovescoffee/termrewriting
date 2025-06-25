use crate::Parser::{Parser,Node,ParserError};
use lexer::Lexer;
let rules = vec![
"a*1 = a",
"a*b = b*a"
"a*a*a = a*a",

];
