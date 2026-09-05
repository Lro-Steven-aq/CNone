pub mod block;
pub mod declarations;
pub mod expr;
mod field;
pub mod operators;
mod param;
pub mod program;
pub mod stmt;

use std::collections::HashMap;

use crate::lexer::TokenType;
use crate::lexer::keywords::Keyword;
use crate::lexer::literalvalue::Literal;
use crate::lexer::operators::Operator;
use crate::lexer::symbols::Symbol;
use block::Block;
use declarations::Decl;
use declarations::FunctionDecl;
// use declarations::StructDecl;
// use declarations::TypeDefDecl;
use declarations::VaribleDecl;
use expr::Expr;
// use field::Field;
use operators::BinaryOp;
use operators::UnaryOp;
use param::Param;
use program::Program;
use stmt::Stmt;
// 再导出，便于使用。
pub use crate::lexer::types::Type;

use super::lexer::Token;
// 解析器结构体对象。
#[derive(Debug, Clone, PartialEq)]

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    typedefs: HashMap<String, Type>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            position: 0,
            typedefs: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut decls = Vec::new();

        while !self.is_eof() {
            decls.push(self.parse_decl());
        }
        Program { decls }
    }

    fn parse_decl(&mut self) -> Decl {
        match self.peek() {
            // TokenType::Keyword(Keyword::Struct) => self.parse_struct(),
            // TokenType::Keyword(Keyword::Typedef) => self.parse_typedef(),
            _ => self.parse_functions_or_variables(),
        }
    }

    /// 为了解决parse_decl函数的复杂性，我们将建立此函数。
    /// 此函数只解析函数或者是变量。
    fn parse_functions_or_variables(&mut self) -> Decl {
        let _type = self.parse_type();
        let name = self.expect_identifier();

        if self.peek() == TokenType::Symbol(Symbol::LParen) {
            /*
            int func(){
                    ^此处是（因此认为是函数
            }
             */
            self.advance(); // 跳过"("
            let params = self.parse_params(); // params 是指形参，args是指实参。
            self.expect(Symbol::RParen); //  ")"

            let body = if self.peek() == TokenType::Symbol(Symbol::Semicolon) {
                self.advance();
                None
            } else {
                Some(self.parse_block())
            };

            Decl::Function(FunctionDecl {
                return_type: _type,
                name: name,
                params: params,
                body: body,
            })
        } else {
            // 变量。
            let init = if self.peek() == TokenType::Operater(Operator::Assign) {
                self.advance();
                Some(self.parse_expr())
            } else {
                None
            };
            self.expect(Symbol::Semicolon);
            Decl::Varible(VaribleDecl {
                typ: _type,
                name: name,
                init: init,
            })
        }
    }

    // /// 解析结构体。
    // fn parse_struct(&mut self) -> Decl {
    //     /*
    //        struct STRUCT_NAME {
    //            类型1 字段a;
    //            类型2 字段b;
    //            ...   ...;
    //        };
    //         ^ 注意分号。
    //        struct STRUCT_NAME obj;
    //     */
    //     self.expect_keyword(Keyword::Struct);
    //     let name = self.expect_identifier();
    //     self.expect(Symbol::LBrace);

    //     // let mut fields = Vec::new();
    //     while self.peek() != TokenType::Symbol(Symbol::RBrace) {
    //         let typ = self.parse_type();
    //         let field_name = self.expect_identifier();
    //         // fields.push(Field {
    //         //     name: field_name,
    //         //     typ: typ,
    //         // });
    //         panic!("Unsupport Struct .");
    //         // self.expect(Symbol::Semicolon);
    //     }
    //     self.advance(); // }
    //     self.expect(Symbol::Semicolon); // ;
    //     // Decl::Struct(StructDecl {
    //     //     name: name,
    //     //     fields: fields,
    //     // })
    //     panic!("Unsupport Struct.")
    // }

    /// 解析typedef
    // fn parse_typedef(&mut self) -> Decl {
    //     self.expect_keyword(Keyword::Typedef);
    //     let typ = self.parse_type();
    //     let alias = self.expect_identifier();
    //     self.expect(Symbol::Semicolon);
    //     self.typedefs.insert(alias.clone(), typ.clone());
    //     // Decl::TypeDef(TypeDefDecl {
    //     //     typ: typ,
    //     //     alias: alias,
    //     // })
    //     panic!("Unsupport Typedef .");
    // }

    /// 解析类型。
    /// int char void char* int* struct STRUCT_NAME FILE*
    fn parse_type(&mut self) -> Type {
        let mut base_type = match self.peek() {
            TokenType::Type(t) => {
                self.advance();
                t
            }
            // TokenType::Keyword(Keyword::Struct) => {
            //     self.advance();
            //     let name = self.expect_identifier();
            //     Type::Struct(name)
            // }
            TokenType::Identifer(identifier) => {
                if let Some(typ) = self.typedefs.get(&identifier).cloned() {
                    self.advance();
                    typ
                } else {
                    panic!("Expected type, got \"{}\"", identifier);
                }
            }
            _ => panic!("Expected type: {:#?}", self.tokens[self.position]),
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
            params.push(Param {
                typ: _type,
                name: identifier,
            });

            match self.peek() {
                TokenType::Symbol(Symbol::Comma) => {
                    self.advance();
                }
                TokenType::Symbol(Symbol::RParen) => break,
                _ => panic!("Require ',' or ')': {:#?}", self.tokens[self.position]),
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
                }
                TokenType::Symbol(Symbol::RParen) => break,
                _ => panic!("Require ',' or ')': {:#?}", self.tokens[self.position]),
            }
        }
        args
    }
    /// 解析一个代码块。
    /**
     * ``c
     * {
     *   int a;
     *   float b;
     *  char ch = 'M';
     *    }
     * ``
     */
    fn parse_block(&mut self) -> Block {
        self.expect(Symbol::LBrace);
        let mut stmts = Vec::new();
        while self.peek() != TokenType::Symbol(Symbol::RBrace) {
            stmts.push(self.parse_stmt()); // 当下一个不是"}"时，解析、推入语句。
            // 拼凑起来！！
        }
        self.expect(Symbol::RBrace);
        Block { stmts }
    }

    /// 语句解析。
    /// Important!
    /// Core!
    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            // 列举一个代码块里面所可能遇见的关键字。
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
            }
            TokenType::Keyword(Keyword::Return) => {
                self.advance(); //  分号。
                let expr = if self.peek() != TokenType::Symbol(Symbol::Semicolon) {
                    Some(self.parse_expr())
                } else {
                    None
                };
                self.expect(Symbol::Semicolon);
                Stmt::Return(expr)
            }
            TokenType::Symbol(Symbol::LBrace) => Stmt::Block(self.parse_block()),
            TokenType::Type(_) => {
                let decl = self.parse_decl();
                match decl {
                    Decl::Varible(var) => Stmt::VaribleDecl(var),
                    _ => panic!(
                        "Expected variables declarations: {:#?}",
                        self.tokens[self.position]
                    ),
                }
            }
            _ => {
                // regard as normal expr
                let expr = self.parse_expr();
                self.expect(Symbol::Semicolon);
                Stmt::Expr(expr)
            }
        }
    }

    ///////////////////////////////辅助方法，不公开。///////////////////////////////////////

    ///
    fn peek(&self) -> TokenType {
        self.tokens
            .get(self.position)
            .map(|t| t.get_token_type().clone())
            .unwrap_or(TokenType::EOF)
    }

    /// 向前进一步
    fn advance(&mut self) {
        self.position += 1;
    }

    fn is_eof(&self) -> bool {
        self.peek() == TokenType::EOF
    }

    /// 对于符号进行断言。
    fn expect(&mut self, expected: Symbol) {
        if self.peek() == TokenType::Symbol(expected.clone()) {
            self.advance();
        } else {
            panic!(
                "Expected {:#?}, got {:#?}.",
                expected, self.tokens[self.position]
            );
        }
    }

    /// 对于标识符进行断言并返回标识符。
    /// ## Return String
    fn expect_identifier(&mut self) -> String {
        match self.peek() {
            TokenType::Identifer(identifier) => {
                self.advance();
                identifier
            }
            _ => panic!("Expected identifier {:#?}", self.tokens[self.position]),
        }
    }

    /// 对于关键字进行断言。
    fn expect_keyword(&mut self, keyword: Keyword) {
        match self.peek() {
            TokenType::Keyword(kw) if keyword == kw => self.advance(),
            _ => {
                panic!("Expected keyword \"{:#?}\"", keyword);
            }
        }
    }
}

impl Parser {
    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();

        while self.peek() == TokenType::Operater(Operator::LogicOr) {
            self.advance();
            let right = self.parse_and();
            left = Expr::BinaryExp(BinaryOp::Or, Box::new(left), Box::new(right));
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
    fn parse_eq(&mut self) -> Expr {
        let mut left = self.parse_rel();
        loop {
            /*
            一直遍历，直至遇见非法字符（也就是遍历结束）
             */
            match self.peek() {
                TokenType::Operater(Operator::Eq) => {
                    self.advance();
                    let right = self.parse_rel();
                    left = Expr::BinaryExp(BinaryOp::Eq, Box::new(left), Box::new(right));
                }
                TokenType::Operater(Operator::Neq) => {
                    self.advance();
                    let right = self.parse_rel();
                    left = Expr::BinaryExp(BinaryOp::Ne, Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_rel(&mut self) -> Expr {
        let mut left = self.parse_add();

        loop {
            match self.peek() {
                TokenType::Operater(Operator::Lt) => {
                    self.advance();
                    let right = self.parse_add();
                    left = Expr::BinaryExp(BinaryOp::Lt, Box::new(left), Box::new(right));
                }
                TokenType::Operater(Operator::Gt) => {
                    self.advance();
                    let right = self.parse_add();
                    left = Expr::BinaryExp(BinaryOp::Gt, Box::new(left), Box::new(right));
                }
                TokenType::Operater(Operator::Le) => {
                    self.advance();
                    let right = self.parse_add();
                    left = Expr::BinaryExp(BinaryOp::Le, Box::new(left), Box::new(right));
                }
                TokenType::Operater(Operator::Ge) => {
                    self.advance();
                    let right = self.parse_add();
                    left = Expr::BinaryExp(BinaryOp::Ge, Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_add(&mut self) -> Expr {
        let mut left = self.parse_mul();
        loop {
            match self.peek() {
                TokenType::Operater(Operator::Plus) => {
                    self.advance();
                    let right = self.parse_mul();
                    left = Expr::BinaryExp(BinaryOp::Add, Box::new(left), Box::new(right));
                }
                TokenType::Operater(Operator::Minus) => {
                    self.advance();
                    let right = self.parse_mul();
                    left = Expr::BinaryExp(BinaryOp::Sub, Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_mul(&mut self) -> Expr {
        let mut left = self.parse_unary();
        loop {
            match self.peek() {
                TokenType::Operater(Operator::Star) => {
                    self.advance();
                    let right = self.parse_unary();
                    left = Expr::BinaryExp(BinaryOp::Mul, Box::new(left), Box::new(right));
                }
                TokenType::Operater(Operator::Slash) => {
                    self.advance();
                    let right = self.parse_unary();
                    left = Expr::BinaryExp(BinaryOp::Div, Box::new(left), Box::new(right));
                }
                TokenType::Operater(Operator::Percent) => {
                    self.advance();
                    let right = self.parse_unary();
                    left = Expr::BinaryExp(BinaryOp::Mod, Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            TokenType::Operater(Operator::LogicNot) => {
                self.advance();
                Expr::UnaryExp(UnaryOp::Not, Box::new(self.parse_unary()))
            }
            TokenType::Operater(Operator::Minus) => {
                self.advance();
                Expr::UnaryExp(UnaryOp::Neg, Box::new(self.parse_unary()))
            }
            TokenType::Operater(Operator::Tilde) => {
                self.advance();
                Expr::UnaryExp(UnaryOp::BitNot, Box::new(self.parse_unary()))
            }
            TokenType::Operater(Operator::Star) => {
                self.advance();
                Expr::UnaryExp(UnaryOp::Deref, Box::new(self.parse_unary()))
            }
            TokenType::Operater(Operator::And) => {
                self.advance();
                Expr::UnaryExp(UnaryOp::AddressOf, Box::new(self.parse_unary()))
            }
            TokenType::Operater(Operator::PlusPlus) => {
                //显然，此时必为前缀表达式。（因为先匹配的是++/--，并且匹配到了。）
                self.advance();
                Expr::UnaryExp(UnaryOp::PreInc, Box::new(self.parse_unary()))
            }
            TokenType::Operater(Operator::MinusMinus) => {
                self.advance();
                Expr::UnaryExp(UnaryOp::PreDec, Box::new(self.parse_unary()))
            }
            _ => self.parse_postfix(), //   后缀。
                                       // 与前缀表达式同一优先级。
        }
    }

    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        loop {
            match self.peek() {
                // 后缀加
                TokenType::Operater(Operator::PlusPlus) => {
                    self.advance();
                    expr = Expr::UnaryExp(UnaryOp::PostInc, Box::new(expr));
                }
                // 后缀减
                TokenType::Operater(Operator::MinusMinus) => {
                    self.advance();
                    expr = Expr::UnaryExp(UnaryOp::PostDec, Box::new(expr));
                }

                TokenType::Symbol(Symbol::LParen) => {
                    self.advance();
                    let args = self.parse_args();
                    self.expect(Symbol::RParen);
                    match expr {
                        Expr::Identifier(function) => {
                            expr = Expr::CallFunction(function, args);
                        }
                        _ => panic!(
                            "Invalid function calling : {:#?}",
                            self.tokens[self.position]
                        ),
                    };
                }
                TokenType::Symbol(Symbol::LBracket) => {
                    self.advance();
                    let index = self.parse_expr();
                    self.expect(Symbol::RBracket);
                    expr = Expr::Index(Box::new(expr), Box::new(index));
                }
                TokenType::Operater(Operator::Arrow) => {
                    // ->
                    self.advance();
                    let member = self.expect_identifier();
                    expr = Expr::PointerMember(Box::new(expr), member);
                }
                TokenType::Operater(Operator::Dot) => {
                    self.advance();
                    let member = self.expect_identifier();
                    expr = Expr::Member(Box::new(expr), member);
                }
                _ => break,
            }
        }
        expr
    }
    /// 表达式最基本的单位。
    fn parse_primary(&mut self) -> Expr {
        match self.peek() {
            TokenType::Constant(Literal::Interage(int)) => {
                self.advance();
                Expr::Integer(int)
            }
            TokenType::Constant(Literal::Float(float)) => {
                self.advance();
                Expr::Float(float)
            }
            TokenType::Constant(Literal::Char(ch)) => {
                self.advance();
                Expr::Char(ch)
            }
            TokenType::Constant(Literal::String(string)) => {
                self.advance();
                Expr::String(string)
            }
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
            }
            TokenType::Symbol(Symbol::LParen) => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(Symbol::RParen);
                expr
            }
            _ => panic!("Unexpected token: {:#?}", self.tokens[self.position]), // 其他鬼魂直接报错。什么魑魅魍魉管他呢。
        }
    }
}


#[test]
fn test_ast() {
    use crate::lexer::Lexer;
    let mut lexer = Lexer::new(r#"
    function main(){
        int a= 9;
        print(a);
    }
    "#);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.get_next_token();
        tokens.push(token.clone());
        if token.get_token_type() == TokenType::EOF {
            break;
        }
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse();
    println!("{:#?}", program);
}