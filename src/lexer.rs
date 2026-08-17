
pub mod keywords;
pub mod types;
pub mod operators;
pub mod symbols;
pub mod literalvalue;



use keywords::Keyword;
use types::Type;
use operators::Operator;
use symbols::Symbol;
use literalvalue::Literal;

#[derive(Debug,Clone,PartialEq)]
pub enum Token{
    //
    Keyword(Keyword),
    Type(Type),
    Constant(Literal),
    Identifer(String),
    Symbol(Symbol),
    Operater(Operator),
    EOF,                   // end of file.
}

pub struct Lexer {
    source: Vec<char>,       //源。
    position: usize,         //当前位置。
    current: char,           //当前字符。
    //current == source[position]恒成立。
}

impl Lexer {
    pub fn new (source: &str) -> Self {
        let chars = source.chars().collect::<Vec<char>>();
        let _first = chars.get(0).copied().unwrap_or('\0');
        Self { source: chars, position: 0, current: _first }
    } 
    
    /**
     * 注意： 去走到下一个字符的位置，不返回。也不是返回Token。
     */
    fn go_to_next(&mut self) {
        self.position += 1;
        self.current = self.source.get(self.position).copied().unwrap_or('\0');
    }

    /**
     * 瞥一眼下一个字符。（不跳到下一个字符）
     * postion 
     */
    #[allow(unused)]
    fn peek(&self) -> char {
        self.source.get(self.position + 1).copied().unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.current.is_whitespace() {  //  如果是空白，
            self.go_to_next();                //  那么就跳过。
        }
    }

    fn read_word(&mut self) -> Token {
        let start = self.position;
        while self.current.is_alphanumeric() || self.current == '_' {
            self.go_to_next();
        }
        let word: String = self.source[start..self.position].iter().collect();
        match word.as_str() {
            //先看类型。
            "int" => return Token::Type(Type::Int),
            "char" => return Token::Type(Type::Char),
            "float" => return Token::Type(Type::Float),
            "double" => return Token::Type(Type::Double),
            "short" => return Token::Type(Type::Short),
            "long" => return Token::Type(Type::Long),
            "unsigned" => return Token::Type(Type::Unsigned),
            "signed" => return Token::Type(Type::Signed),
            "void" => return Token::Type(Type::Void),
            //再看是不是关键字
            "if" => return Token::Keyword(Keyword::If),
            "else" => return Token::Keyword(Keyword::Else),
            "for" => return Token::Keyword(Keyword::For),
            "while" => return Token::Keyword(Keyword::While),
            "do" => return Token::Keyword(Keyword::Do),
            "return" => return Token::Keyword(Keyword::Return),
            "break" => return Token::Keyword(Keyword::Break),
            "continue" => return Token::Keyword(Keyword::Continue),
            "switch" => return Token::Keyword(Keyword::Switch),
            "case" => return Token::Keyword(Keyword::Case),
            "default" => return Token::Keyword(Keyword::Default),
            "goto" => return Token::Keyword(Keyword::Goto),
            "sizeof" => return Token::Keyword(Keyword::Sizeof),
            _ => {},      
        }
        //如果都不是，那么就判定为合法标识符。
        return Token::Identifer(word);
    } 

    /**
     * *技术、时间问题，现在仅做整数支持。*
     * 不考虑二进制、八进制、十六进制等等的支持。
     */
    fn read_number(&mut self) -> Token {
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
            self.go_to_next();// skip "."

            //小数部分。尽管，我们还是把整数部分和小数部分一起解析
            while self.current.is_numeric() {
                self.go_to_next();
            }
        }
        let number: String = self.source[start..self.position].iter().collect();
        if is_float {
            Token::Constant(Literal::Float(number.parse().unwrap_or(0.0)))
        } else {
            Token::Constant(Literal::Interage(number.parse().unwrap_or(0)))
        }
    }

    /// 没有\x十六进制转义支持，也没有\o八进制转义支持和\u Unicode支持。
    fn read_string(&mut self) -> Token {
        //此时，其current依旧是'"'，所以需要先跳过这个字符。
        self.go_to_next();
        let mut result = String::new();
        while self.current != '"' && self.current != '\0' {
            if self.current == '\\' {
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
        Token::Constant(Literal::String(result))
    }

    /// 没有\x十六进制转义支持，也没有\o八进制转义支持和\u Unicode支持。
    fn read_char(&mut self) -> Token {
        self.go_to_next(); //skip '
        let _char = if self.current == '\\' {
            self.go_to_next();
            match self.current {
                // return to varible '_char'
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

        Token::Constant(Literal::Char(_char))
    }

    fn read_operator_or_symbol(&mut self) -> Token {
        let _char = self.current;
        // let next = self.peek(); 

        match _char {
            // 先处理operators
            '+' => {
                self.go_to_next();
                match self.current {
                    '+' => {self.go_to_next(); Token::Operater(Operator::PlusPlus)},
                    '=' => {self.go_to_next(); Token::Operater(Operator::PlusAssign)},
                    _ => Token::Operater(Operator::Plus),
                }
            },
            '-' => {
                self.go_to_next();
                match self.current {
                    '-' => {self.go_to_next(); Token::Operater(Operator::MinusMinus)},
                    '=' => {self.go_to_next(); Token::Operater(Operator::MinusAssign)},
                    '>' => {self.go_to_next(); Token::Operater(Operator::Arrow)},
                    _ => Token::Operater(Operator::Minus),
                }
            },
            '*' => {
                self.go_to_next();
                match self.current {
                    '=' => {self.go_to_next(); Token::Operater(Operator::StarAssign)},
                    _ => Token::Operater(Operator::Star),
                }
            },
            '/' => {
                self.go_to_next();
                match self.current {
                    '=' => {self.go_to_next();Token::Operater(Operator::SlashAssign)},
                    _ => Token::Operater(Operator::Slash),
                }
            },
            '%' => {
                self.go_to_next();
                match self.current {
                    '=' => {self.go_to_next();Token::Operater(Operator::PercentAssign)},
                    _ => Token::Operater(Operator::Percent),
                }
            },
            '=' => {
                self.go_to_next();
                match self.current {
                    '=' => {self.go_to_next(); Token::Operater(Operator::Eq)},
                    _ => Token::Operater(Operator::Assign),
                }
            },
            '!' => {
                self.go_to_next();
                match self.current {
                    '=' => {self.go_to_next(); Token::Operater(Operator::Neq)},
                    _ => Token::Operater(Operator::LogicNot),
                }
            },
            '<' => {
                self.go_to_next();
                match self.current {
                    '=' => {self.go_to_next(); Token::Operater(Operator::Le)},
                    '<' => {self.go_to_next(); Token::Operater(Operator::Shl)},
                    _ => Token::Operater(Operator::Lt),
                }
            },
            '>' => {
                self.go_to_next();
                match self.current {
                    '=' => {self.go_to_next(); Token::Operater(Operator::Ge)},
                    '>' => {self.go_to_next(); Token::Operater(Operator::Shr)},
                    _ => Token::Operater(Operator::Gt),
                }
            },
            '&' => {
                self.go_to_next();
                match self.current {
                    '&' => {self.go_to_next(); Token::Operater(Operator::LogicAnd)},
                    _ => Token::Operater(Operator::And),
                }
            },
            '|' => {
                self.go_to_next();
                match self.current {
                    '|' => {self.go_to_next(); Token::Operater(Operator::LogicOr)},
                    _ => Token::Operater(Operator::Or),
                }
            },
            //再处理symbol
            '(' => {self.go_to_next(); Token::Symbol(Symbol::LParen)},
            ')' => {self.go_to_next(); Token::Symbol(Symbol::RParen)},
            '{' => {self.go_to_next(); Token::Symbol(Symbol::LBrace)},
            '}' => {self.go_to_next(); Token::Symbol(Symbol::RBrace)},
            '[' => {self.go_to_next(); Token::Symbol(Symbol::LBracket)},
            ']' => {self.go_to_next(); Token::Symbol(Symbol::RBracket)},
            ';' => {self.go_to_next(); Token::Symbol(Symbol::Semicolon)},
            ',' => {self.go_to_next(); Token::Symbol(Symbol::Comma)},
            ':' => {self.go_to_next(); Token::Symbol(Symbol::Colon)},
            _ => {
                self.go_to_next();
                panic!("Unkonwn TOKEN {} @ character position: {}",self.current, self.position);
            },
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.current {
            '\0' => Token::EOF,
            _char if _char.is_alphabetic() || _char == '_' => self.read_word(),
            _char if _char.is_numeric() => self.read_number(),
            '"' => self.read_string(),
            '\'' => self.read_char(),
            _ => self.read_operator_or_symbol(),
            
        }
    }

}
