#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Char(char),
    Bool(bool),
    String(String),
}
