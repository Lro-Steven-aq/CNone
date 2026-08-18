pub mod program_structure;
use program_structure::*;
use crate::lexer::{TokenType, keywords::Keyword, literalvalue::Literal, operators::Operator, symbols::Symbol, types::Type};

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
            let params = self.parse_params();  // params 是指形参，args是指实参。
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
            self.advance();
            let right = self.parse_assign();
            Expr::Assign(Box::new(left), Box::new(right))
        } else {
            left
        }
    }

    /// 解析形参。
    fn parse_params(&mut self) -> Vec<Param> {
        let mut params = Vec::new();
        if self.peek() == TokenType::Symbol(Symbol::RParen) {
            return params; // 无参。
        }
        loop {
            let _type = self.parse_type();
            let identifier = self.expect_identifier();
            params.push(Param { typ: _type, name: identifier });

            match self.peek() {
                TokenType::Symbol(Symbol::Comma) => {
                    self.advance();
                },
                TokenType::Symbol(Symbol::RParen) => break,
                _ => panic!("Require ',' or ')'"),
            }
        }
        params
    }

    fn parse_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();

        if self.peek() == TokenType::Symbol(Symbol::RParen) {
            return args;
        }

        loop {
            args.push(self.parse_expr());
            match self.peek() {
                TokenType::Symbol(Symbol::Comma) => {
                    self.advance();
                },
                TokenType::Symbol(Symbol::RParen) => break,
                _ => panic!("Require ',' or ')'"),
            }
        }
        args
    }
    /// 解析一个代码块。
    /**
         * ```c
         * {
         *   int a;
         *   float b;
         *  char ch = 'M';
         *    }
         * ```
    */
    fn parse_block(&mut self) -> Block {
        self.expect(Symbol::LBrace);
        let mut stmts = Vec::new();
        while self.peek() != TokenType::Symbol(Symbol::RBrace) {
            stmts.push(self.parse_stmt());  // 当下一个不是"}"时，解析、推入语句。
            // 拼凑起来！！
        }
        self.expect(Symbol::RBrace);
        Block { stmts }
    }

    /// 语句解析。
    /// Important!
    /// Core!
    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {   // 列举一个代码块里面所可能遇见的关键字。
            TokenType::Keyword(Keyword::If) => {
                // if 处理。
                self.advance();
                self.expect(Symbol::LParen);

                let condition = self.parse_expr();
                self.expect(Symbol::RParen);

                let then_stmts = self.parse_stmt();
                let else_stmts = if self.peek() == TokenType::Keyword(Keyword::Else) {
                    self.advance();
                    Some(Box::new(self.parse_stmt()))
                } else {
                    None
                };

                Stmt::If(condition, Box::new(then_stmts), else_stmts)
            },
            TokenType::Keyword(Keyword::Return) => {
                self.advance();                                         //  分号。
                let expr = if self.peek() != TokenType::Symbol(Symbol::Semicolon) {
                    Some(self.parse_expr())
                } else {
                    None
                };
                self.expect(Symbol::Semicolon);
                Stmt::Return(expr)
            },
            TokenType::Symbol(Symbol::LBrace) => {
                Stmt::Block(self.parse_block())
            },
            TokenType::Type(_) => {
                let decl = self.parse_decl();
                match decl {
                    Decl::Varible(var) => Stmt::VaribleDecl(var),
                    _ => panic!("Expected variables declarations"),
                }
            },
            _ => {
                // regard as normal expr
                let expr = self.parse_expr();
                self.expect(Symbol::Semicolon);
                Stmt::Expr(expr)
            },

        }
    }

    ///////////////////////////////辅助方法，不公开。///////////////////////////////////////
    
    ///
    fn peek(&self) -> TokenType {
        self.tokens
            .get(self.position)
            .map(|T| T.get_token_type().clone())
            .unwrap_or(TokenType::EOF)
    }

    /// 向前进一步
    fn advance(&mut self) {
        self.position += 1;
    }

    fn is_eof(&self) -> bool {
        self.peek() == TokenType::EOF
    }

    fn expect(&mut self, expected: Symbol) {
        if self.peek() == TokenType::Symbol(expected.clone()) {
            self.advance();
        } else {
            panic!("Expected {:#?}, got {:#?}.", expected, self.peek());
        }
    }

    fn expect_identifier(&mut self) -> String {
        match self.peek() {
            TokenType::Identifer(identifier) => {
                self.advance();
                identifier
            },
            _ => panic!("Expected identifier"),
        }
    }

}



impl Parser {
    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();

        while self.peek() == TokenType::Operater(Operator::LogicOr) {
            self.advance();
            let right = self.parse_and();
            left = Expr::BinaryExp(BinaryOp::Or, left, right);
        }
        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_eq();

        while self.peek() == TokenType::Operater(Operator::LogicAnd) {
            self.advance();
            let right = self.parse_eq();
            left = Expr::BinaryExp(BinaryOp::Add, Box::new(left), Box::new(right));
        }
        left
    }
    // TODO: eq,add,mul,unary,primary






        /// 表达式最基本的单位。
    fn parse_primary(&mut self) -> Expr {
        match self.peek() {
            TokenType::Constant(Literal::Interage(N)) => {
                self.advance();
                Expr::Integer(N)
            },
            TokenType::Identifer(identifier) => {
                self.advance();
                if self.peek() == TokenType::Symbol(Symbol::LParen) {
                    // 此处即为函数的调用。
                    self.advance();
                    let args = self.parse_args();
                    self.expect(Symbol::RParen);
                    Expr::CallFunction(identifier, args)                   
                } else {
                    Expr::Identifier(identifier)
                }
            },
            TokenType::Symbol(Symbol::LParen) => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(Symbol::RParen);
                expr
            },
            _ => panic!("Unexpected token: {:#?}", self.peek()) // 其他鬼魂直接报错。什么魑魅魍魉管他呢。

        }
    }
}