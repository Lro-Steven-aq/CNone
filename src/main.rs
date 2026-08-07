
use cnone::preprocessor::Preprocessor;
use cnone::lexer::Lexer;
use cnone::lexer::Token;
fn main() {
    let c_code = r#"int main (void) {
        int kode = 9;/*  cc=[]  */
        float num=0.9*kode; //num=8.1
        return 0; // this means no errors.
}
    "#;
    let mut code = Preprocessor::new(c_code);
    let mut lexer = Lexer::new(code.preprocess());
    println!("{:#?}",code);
    loop {
        let token = lexer.get_next_token();
        println!("{:?}",token);

        if token == Token::EOF {
            break;
        }
        // match token {
        //     Token::Constant(l) => println!("{:?}",l),
        //     _ => (),
        // }
    }
}
