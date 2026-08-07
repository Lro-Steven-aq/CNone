
#[derive(Debug,Clone,PartialEq)]
pub struct Preprocessor{
    source: String,
}

impl Preprocessor {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string() }
    }
    pub fn preprocess(&mut self) -> &str {
        self.remove_comments();
        self.process_command();
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
                        for _c_ in chars.by_ref() {
                            if prev == '*' && _c_ == '/' {
                                //找到了结尾。
                                is_end = true;
                                break;
                            }
                            prev = _c_;
                        }
                        if is_end {
                            result.push(' ');//改写为空格。
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
    fn process_command(&mut self) {
        // TODO
    }

}
// mod mm {}