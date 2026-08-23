use std::env::args;
use std::fs;

use cnone::ast::Parser;
use cnone::lexer::Lexer;
use cnone::lexer::TokenType;
use cnone::preprocessor::Preprocessor;
fn main() {
    let args = args().collect::<Vec<String>>();
    let file = match args.get(1) {
        Some(v) => v,
        None => "./.test.c",
    };
    let c_code = fs::read_to_string(file).unwrap();
    let mut code = Preprocessor::new(&c_code);
    let mut lexer = Lexer::new(code.preprocess());
    let mut tokens = Vec::new();
    // println!("{:#?}",code);
    loop {
        let token = lexer.get_next_token();
        tokens.push(token.clone());

        // println!("{:?}",token);

        if token.get_token_type() == TokenType::EOF {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("AST: {:#?}", ast);
}
