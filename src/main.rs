use std::env::args;
use std::fs;

use cnone::lexer::get_tokens;
use cnone::ast::Parser;
use cnone::lexer::Lexer;
use cnone::lexer::TokenType;
use cnone::preprocessor::Preprocessor;
fn main() {
    let args = args().collect::<Vec<String>>();
    let file = match args.get(1) {
        Some(v) => v,
        None => "./test/test.cnone",
    };
    let c_code = fs::read_to_string(file).unwrap();
    let mut code = Preprocessor::new(&c_code);
    let mut lexer = Lexer::new(code.preprocess());
    let mut tokens = get_tokens(&mut lexer).unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("Program: {:#?}", ast);
    // panic!("unimplemented");
}
