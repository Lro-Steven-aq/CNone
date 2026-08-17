pub mod program_structure;
use program_structure::*;
use crate::lexer::{TokenType, operators::Operator, symbols::Symbol, types::Type};

use super::lexer::Token;

// 解析器结构体对象。
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens, position: 0, }
    }

    pub fn parse(&mut self) -> Program {
        let mut decls = Vec::new();
        
        while !self.is_eof() {
            decls.push(self.parse_decl());
        }
        Program { decls }
    }

    fn parse_decl(&mut self) -> Decl {
        let _type = self.parse_type();
        let name = self.expect_identifier();

        if self.peek() == TokenType::Symbol(Symbol::LParen) {
            /*      
            int func(){
                    ^此处是（因此认为是函数
            }
             */
            self.advance();// 跳过"("
            let params = self.parse_params();
            self.expect(Symbol::RParen);  //  ")"

            let body = if self.peek() == TokenType::Symbol(Symbol::Semicolon) {
                self.advance();
                None
            } else {
                Some(self.parse_block())
            };

            Decl::Function(FunctionDecl { return_type: _type, name: name, params: params, body: body })
        } else {

            // 变量。
            let init = if self.peek() == TokenType::Operater(Operator::Assign){
                self.advance();
                Some(self.parse_expr())
            } else {
                None
            };
            self.expect(Symbol::Semicolon);
            Decl::Varible(VaribleDecl { typ: _type, name: name, init: init })
        }

    }

    /// 解析类型。
    /// int char void char* int*
    fn parse_type(&mut self) -> Type {
        let mut base_type = match self.peek() {
            TokenType::Type(T) => {
                self.advance();
                T
            },
            _ => panic!("Expected type !"),
        };
        while self.peek() == TokenType::Operater(Operator::Star) {
          self.advance();
          base_type = Type::Pointer(Box::new(base_type));
        }
        base_type
    }

    /////////////////////////////////////
    fn parse_expr(&mut self) -> Expr {
        self.parse_assign()
    }

    fn parse_assign(&mut self) -> Expr {
        let left = self.parse_or();
        if self.peek() == TokenType::Operater(Operator::Assign) {
            self.
        }
    }
}