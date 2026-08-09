
use std::collections::HashMap;
use std::fs;

#[derive(Debug,Copy,Clone,PartialEq)]
enum IfState {
    Taking,
    Skipping,
    Done,
}


#[derive(Debug,Clone,PartialEq)]
pub enum Macro {
    Object(String),
    Function(Vec<String>, String),  // 宏函数
}
#[derive(Debug,Clone,PartialEq)]
pub struct Preprocessor{
    source: String,
    macros: HashMap<String, Macro>,
    if_state: Vec<IfState>,
}

impl Preprocessor {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string(), macros: HashMap::new(), if_state: Vec::new() }
    }

    fn is_taking(&self) -> bool {
        self.if_state.iter().all(|&s| s == IfState::Taking )
    }

    pub fn preprocess(&mut self) -> &str {
        self.remove_comments();
        self.process_macros();
        return &self.source;
    }

    fn remove_comments(&mut self) {
        let content = self.source.clone();
        let mut chars = content.chars().peekable();
        // 一个与content一样大小的容器，存放结果。
        let mut result = String::with_capacity(content.len());
        while let Some(_char) = chars.next() {
            if _char == '/' {
                //先看一下是不是单行注释。"//'
                match chars.peek() {
                    Some(&'/') => { //单行注释。
                        chars.next(); //跳过第二个"/"
                        for _c_  in chars.by_ref() {
                            if _c_ == '\n' {
                                result.push('\n');
                                break;
                            }
                        }
                    },
                    Some(&'*') => { //多行注释。
                        chars.next(); //跳过。
                        let mut prev = '\0';
                        let mut is_end = false;
                        let mut newline_count = 0;
                        for _c_ in chars.by_ref() {
                            if _c_ == '\n' {
                                newline_count += 1;
                            }
                            if prev == '*' && _c_ == '/' {
                                //找到了结尾。
                                is_end = true;
                                break;
                            }
                            prev = _c_;
                        }
                        if is_end {
                            if newline_count != 0{
                                for _ in 0..newline_count{
                                    result.push('\n');//改写为空行。
                                }
                            } else {
                                result.push(' '); //改写为空格。
                            }
                        } else {
                            panic!("Unterminated comments block.");
                        }
                    },
                    _ => result.push('/'),

                }
            } else {
                result.push(_char);
            }
        }
        //////////
        self.source = result;

    }

    fn process_macros(&mut self) {
        /*
            #include
            #define
            #undef
            #ifdef #ifndef #endif
            
            #else #elif #if
            __FILE__ __LINE__ __DATA__ __TIME__
            # ##    (字符串化，标记粘贴)
            \   (行继续)
         */
        
        ////////////////先识别。//////////////////////////
        let lines = self.source
            .lines()
            .map(|mm|mm.to_string())
            .collect::<Vec<String>>();
        let mut result = String::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].clone();
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                /////////////////// 以#号开头的，就认为是预处理指令。///////////////////////
                // 是预处理指令。
                self.handle_macros(trimmed, &mut result);
            } else if self.is_taking() { // 宏允许的可用代码
                result.push_str(&self.expand_macros(&line));
                result.push('\n');
            }
            i += 1;
        }
        if !self.if_state.is_empty() {
            panic!("Unterminated #if/#ifdef/#ifndef");
        }

        self.source = result;
    }

    fn handle_macros(&mut self, line: &str, result: &mut String) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        // if,ifdef,ifndef为push.其余为handle...
        match parts.get(0) {
            Some(&"#include") => self.handle_include(parts.get(1), result),
            Some(&"#define") => self.handle_define(&parts[1..]),
            Some(&"#ifdef") => {
                let condtion = parts.get(1)
                    .map(|s| self.macros.contains_key(*s)).unwrap_or(false);
                self.push_if_state(condtion);
    //            self.handle_conditional(&parts[1..]);
            },
            Some(&"#if") => {
                panic!("暂不支持 if");
    //            self.push_if_state(false);
            },
            Some(&"#ifndef") => {
                let condtion = parts.get(1)
                    .map(|s| !self.macros.contains_key(*s)).unwrap_or(true);
                self.push_if_state(condtion);
    //            self.handle_conditional(&parts[1..]);
            },
            Some(&"#endif") => {
                self.handle_endif();
            },
            Some(&"#elif") => {
                self.handle_elif();
            },
            Some(&"#else") => {
                self.handle_else();
            },

            Some(&"#undef") => {
                if self.is_taking() {
                    self.handle_undef(parts.get(1));
                }
    //            self.handle_undef(parts.get(1))
            },
            /*
            没有实现： #else #elif #if
             */
            _ => panic!("Unknown macros: {:#?}", parts.get(0)),

        } 
    }

    fn handle_include(&self, path: Option<&&str>, result: &mut String) {
        let Some(path) = path else { return; };
        let filepath = path.trim_matches(|p| p == '"' || p == '<' || p == '>' );
        match fs::read_to_string(filepath) {
            Ok(content) => {
                let mut pp = Preprocessor::new(&content);
                let expanded_content = pp.preprocess();
                result.push_str(expanded_content);
            },
            Err(e) => panic!("Cannt open file: {}, err: {}", filepath, e),
        }
    }

    fn handle_define(&mut self, parts: &[&str]) {
        if parts.is_empty() { return; }  // #define 无参

        let name = parts[0].to_string();
        let value = parts[1..].join(" ");
        self.macros.insert(name, Macro::Object(value));
    }

    fn expand_macros(&self, line: &str) -> String {
        let mut result = line.to_string();

        for (name, _macro) in &self.macros {
            match _macro {

                Macro::Object(value) => {
                    result = self.replace_word(&result, name, value);
                },
                Macro::Function(_, _) => {
                    // TODO
                }
                
            }
        }
        result
    }

    fn replace_word(&self, text: &str, name: &str, value: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(_char) = chars.next() {
            if _char.is_alphanumeric() || _char == '_' {
                let mut word = String::new();
                word.push(_char);
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        word.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if word == name {
                    result.push_str(value);
                } else {
                    result.push_str(&word);
                }
            } else {
                result.push(_char);
            }
        }
        result
    }


    fn handle_undef(&mut self, name: Option<&&str>) {
        // TODO: Impl
        let Some(name) = name else {
            panic!("#undef missing macro name.");
        };
        self.macros.remove(*name);
    }

    /**
     * 推入新的编译状态，决定怎么编译。
     */
    fn push_if_state(&mut self, condition: bool) {
        let state = if condition {
            IfState::Taking
        } else {
            IfState::Skipping
        };
        self.if_state.push(state);
    }

    fn handle_elif(&mut self) {
        let last = self.if_state.last_mut().expect("[ERROR]  Panic( #elif without #if! )");
        match *last {
            IfState::Taking => {
                //之前在编译，那么现在就是跳过。
                *last = IfState::Done;
            },
            IfState::Skipping => {
                //之前是跳过，那么现在就是编译。
                *last = IfState::Taking;
            },
            IfState::Done => {
                //不变。
                *last = IfState::Done;
            }
        }
    }

    fn handle_else(&mut self) {
        let last = self.if_state.last_mut().expect("[ERROR]  Panic( #else without #if )");
        match *last {
            IfState::Taking => *last = IfState::Done,
            IfState::Skipping => *last = IfState::Taking,
            IfState::Done => {},
        }
    }

    fn handle_endif(&mut self) {
        self.if_state.pop().expect("[ERROR]  Panic( #endif without #if )");
    }

    /*
        缺失：#if 求值，defined()函数。
        defined()不计划支持。
     */

}
// mod mm {}