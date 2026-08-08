use std::env::args;
use std::fs;


use cnone::lexer::TokenType;
use cnone::preprocessor::Preprocessor;
use cnone::lexer::Lexer;
fn main() {
    let args = args().collect::<Vec<String>>();
    let file = match args.get(1){
        Some(v) => v,
        None => "./.test.c", 
    };
    let c_code = fs::read_to_string(file).unwrap();
    let mut code = Preprocessor::new(&c_code);
    let mut lexer = Lexer::new(code.preprocess());
    println!("{:#?}",code);
    loop {
        let token = lexer.get_next_token();
        println!("{:?}",token);

        if token.get_token_type() == TokenType::EOF {
            break;
        }
        // match token {
        //     Token::Constant(l) => println!("{:?}",l),
        //     _ => (),
        // }
    }
}
