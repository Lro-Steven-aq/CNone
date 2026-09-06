pub mod keywords;
pub mod literalvalue;
pub mod operators;
pub mod symbols;
pub mod types;

use keywords::Keyword;
use literalvalue::Literal;
use operators::Operator;
use symbols::Symbol;
use types::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    //
    Keyword(Keyword),
    Type(Type),
    Constant(Literal),
    Identifer(String),
    Symbol(Symbol),
    Operater(Operator),
    EOF, // end of file.
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    typ: TokenType,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    source: Vec<char>, //源。
    position: usize,   //当前位置。
    current: char,     //当前字符。
    //current == source[position]恒成立。
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let chars = source.chars().collect::<Vec<char>>();
        let _first = chars.get(0).copied().unwrap_or('\0');
        Self {
            source: chars,
            position: 0,
            current: _first,
            line: 1,
            column: 1,
        }
    }

    /**
     * 注意： 去走到下一个字符的位置，不返回。也不是返回Token。
     */
    fn go_to_next(&mut self) {
        if self.current == '\n' {
            //如果是行末，那么就加一行，竖向变为1。
            self.line += 1; //
            self.column = 1; //
        } else {
            // self.line不变。
            self.column += 1;
        }

        self.position += 1;
        self.current = self.source.get(self.position).copied().unwrap_or('\0');
    }

    /**
     * ### 依据当前行号和TokenType的来生成Token
     */
    fn generate_token(&self, typ: TokenType) -> Token {
        Token {
            typ: typ,
            line: self.line,
            column: self.column,
        }
    }

    /**
     * 瞥一眼下一个字符。（不跳到下一个字符）
     * position
     */
    fn peek(&self) -> char {
        self.source.get(self.position + 1).copied().unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.current.is_whitespace() {
            //  如果是空白，
            self.go_to_next(); //  那么就跳过。
        }
    }

    fn read_word(&mut self) -> TokenType {
        let start = self.position;

        while self.current.is_alphanumeric() || self.current == '_' {
            self.go_to_next();
        }
        let word: String = self.source[start..self.position].iter().collect();
        match word.as_str() {
            //先看类型。
            "int" => TokenType::Type(Type::Int),
            "char" => TokenType::Type(Type::Char),
            "float" => TokenType::Type(Type::Float),
            "double" => TokenType::Type(Type::Double),
            "short" => TokenType::Type(Type::Short),
            "long" => TokenType::Type(Type::Long),
            // "unsigned" => TokenType::Type(Type::Unsigned),
            // "signed" => TokenType::Type(Type::Signed),
            "void" => TokenType::Type(Type::Noreturn),  // void
            "Noreturn" => TokenType::Type(Type::Noreturn),
            //再看是不是关键字
            "if" => TokenType::Keyword(Keyword::If),
            "else" => TokenType::Keyword(Keyword::Else),
            "for" => TokenType::Keyword(Keyword::For),
            "while" => TokenType::Keyword(Keyword::While),
            "do" => TokenType::Keyword(Keyword::Do),
            "return" => TokenType::Keyword(Keyword::Return),
            "break" => TokenType::Keyword(Keyword::Break),
            "continue" => TokenType::Keyword(Keyword::Continue),
            "switch" => TokenType::Keyword(Keyword::Switch),
            "case" => TokenType::Keyword(Keyword::Case),
            "default" => TokenType::Keyword(Keyword::Default),
            "goto" => TokenType::Keyword(Keyword::Goto),
            "sizeof" => TokenType::Keyword(Keyword::Sizeof),
            // "struct" => TokenType::Keyword(Keyword::Struct),
            // "typedef" => TokenType::Keyword(Keyword::Typedef),
            "function" => TokenType::Keyword(Keyword::Function),
            //如果什么都不是，那么将被认定为合法标识符。
            _ => TokenType::Identifer(word),
        }
    }

    /**
     * *技术、时间问题，现在仅做整数支持。*
     * 不考虑二进制、八进制、十六进制等等的支持。
     */
    fn read_number(&mut self) -> TokenType {
        let start = self.position;
        let mut is_float = false;
        //仅支持十进制数。
        //8月7日，14:56（星期五）增加了对小数的支持。（科学计数法除外）

        //这是整数部分/整数
        while self.current.is_numeric() {
            self.go_to_next();
        }
        if self.current == '.' && self.peek().is_numeric() {
            is_float = true;
            self.go_to_next(); // skip "."

            //小数部分。尽管，我们还是把整数部分和小数部分一起解析
            while self.current.is_numeric() {
                self.go_to_next();
            }
        }
        let number: String = self.source[start..self.position].iter().collect();
        if is_float {
            TokenType::Constant(Literal::Float(number.parse().unwrap_or(0.0)))
        } else {
            TokenType::Constant(Literal::Interage(number.parse().unwrap_or(0)))
        }
    }

    /// 没有\x十六进制转义支持，也没有\o八进制转义支持和\u Unicode支持。
    fn read_string(&mut self) -> TokenType {
        //此时，其current依旧是'"'，所以需要先跳过这个字符。
        self.go_to_next();
        //使用result来存储最后的结果。
        //不能使用read_word(&self)和read_number(&self)的截取法，因为需要处理转义。
        let mut result = String::new();
        while self.current != '"' && self.current != '\0' {
            //扫描字符串。
            if self.current == '\\' {
                //因为转义字符需要特殊处理。
                //先转义处理。\\与其后一个字符连为同一个。
                //例如：\\n -> \n     \\r -> \r
                //读取其后一个。
                self.go_to_next();
                match self.current {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    '0' => result.push('\0'),
                    // 没有\x十六进制转义支持，也没有\o八进制转义支持和\u Unicode支持。
                    _ => result.push(self.current),
                }
            } else {
                //不是转义字符。
                result.push(self.current);
            }
            self.go_to_next();
        }
        //同理，也跳过结尾的'"'。
        self.go_to_next();
        TokenType::Constant(Literal::String(result))
    }

    /// 没有\x十六进制转义支持，也没有\o八进制转义支持和\u Unicode支持。
    fn read_char(&mut self) -> TokenType {
        self.go_to_next(); //skip '
        let _char = if self.current == '\\' {
            self.go_to_next();
            match self.current {
                // return to variable '_char'
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '0' => '\0',
                // 没有\x十六进制转义支持，也没有\o八进制转义支持和\u Unicode支持。
                _char_ => _char_,
            }
        } else {
            self.current
        };
        self.go_to_next();
        self.go_to_next();

        TokenType::Constant(Literal::Char(_char))
    }

    fn read_operator_or_symbol(&mut self) -> TokenType {
        let _char = self.current;
        // let next = self.peek();

        match _char {
            // 先处理operators
            '+' => {
                self.go_to_next();
                match self.current {
                    '+' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::PlusPlus)
                    }
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::PlusAssign)
                    }
                    _ => TokenType::Operater(Operator::Plus),
                }
            }
            '-' => {
                self.go_to_next();
                match self.current {
                    '-' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::MinusMinus)
                    }
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::MinusAssign)
                    }
                    '>' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::Arrow)
                    }
                    _ => TokenType::Operater(Operator::Minus),
                }
            }
            '*' => {
                self.go_to_next();
                match self.current {
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::StarAssign)
                    }
                    _ => TokenType::Operater(Operator::Star),
                }
            }
            '/' => {
                self.go_to_next();
                match self.current {
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::SlashAssign)
                    }
                    _ => TokenType::Operater(Operator::Slash),
                }
            }
            '%' => {
                self.go_to_next();
                match self.current {
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::PercentAssign)
                    }
                    _ => TokenType::Operater(Operator::Percent),
                }
            }
            '=' => {
                self.go_to_next();
                match self.current {
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::Eq)
                    }
                    _ => TokenType::Operater(Operator::Assign),
                }
            }
            '!' => {
                self.go_to_next();
                match self.current {
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::Neq)
                    }
                    _ => TokenType::Operater(Operator::LogicNot),
                }
            }
            '<' => {
                self.go_to_next();
                match self.current {
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::Le)
                    }
                    '<' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::Shl)
                    }
                    _ => TokenType::Operater(Operator::Lt),
                }
            }
            '>' => {
                self.go_to_next();
                match self.current {
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::Ge)
                    }
                    '>' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::Shr)
                    }
                    _ => TokenType::Operater(Operator::Gt),
                }
            }
            '&' => {
                self.go_to_next();
                match self.current {
                    '&' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::LogicAnd)
                    }
                    _ => TokenType::Operater(Operator::And),
                }
            }
            '|' => {
                self.go_to_next();
                match self.current {
                    '|' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::LogicOr)
                    }
                    _ => TokenType::Operater(Operator::Or),
                }
            }
            '.' => {
                self.go_to_next();
                TokenType::Operater(Operator::Dot)
            }
            '^' => {
                self.go_to_next();
                match self.current {
                    '=' => {
                        self.go_to_next();
                        TokenType::Operater(Operator::XorAssign)
                    }
                    _ => TokenType::Operater(Operator::Xor),
                }
            }
            '~' => {
                self.go_to_next();
                TokenType::Operater(Operator::LogicNot)
            }
            //再处理symbol
            '(' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::LParen)
            }
            ')' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::RParen)
            }
            '{' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::LBrace)
            }
            '}' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::RBrace)
            }
            '[' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::LBracket)
            }
            ']' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::RBracket)
            }
            ';' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::Semicolon)
            }
            ',' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::Comma)
            }
            ':' => {
                self.go_to_next();
                TokenType::Symbol(Symbol::Colon)
            }
            _ => {
                self.go_to_next();
                panic!(
                    "Unknown TOKEN {} character line: {} column: {}",
                    self.current, self.line, self.column
                );
            }
        }
    }

    fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();

        let typ = match self.current {
            '\0' => TokenType::EOF,
            _char if _char.is_alphabetic() || _char == '_' => self.read_word(),
            _char if _char.is_numeric() => self.read_number(),
            '"' => self.read_string(),
            '\'' => self.read_char(),
            _ => self.read_operator_or_symbol(),
        };

        self.generate_token(typ)
    }
}

impl Token {
    pub fn get_token_type(&self) -> TokenType {
        self.typ.clone()
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}

pub fn get_tokens(lexer: &mut Lexer) -> Result<Vec<Token>, ()> {
    let mut tokens = Vec::new();
    loop {
        let token = lexer.get_next_token();
        tokens.push(token.clone());
        if token.get_token_type() == TokenType::EOF {
            break;
        }
    }
    Ok(tokens)
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new(r#"
    function main () {
        int a = 9;
        print(a);
    }
    function print(int args){
        if (a<0){echo("a>0");}
        else{}
        echo(a);
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
    println!("{:#?}", tokens);
}