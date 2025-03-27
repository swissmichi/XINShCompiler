/// Represents the different Tokens
///
/// # Usage
///
/// Make it easier to parse the source code
///
#[derive(Debug, PartialEq)]
pub enum Token {
    // Single Characters
    Delim(String),
    Operator(String),
    
    // Literals
    NumLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    BoolLiteral(bool),

    // Other

    Keyword(String),
    Identifier(String),
    MalformedChar,
}
