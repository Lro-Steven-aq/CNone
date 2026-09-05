use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    msg: String,
    code: u8,
}
impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
    fn description(&self) -> &str {
        &self.msg
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.msg)
    }
}