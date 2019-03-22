#[derive(Debug)]
pub enum Token {
    Break,
    ArrayOpen,
    ArrayValue(String),
    ArrayWhitespace,
    StringValue(String),
    Key(String),
    None,
}