// tokens.rs
// Enum with all returnable tokens

pub mod tokens{
    pub enum Token<'a> {
        // Single Characters
        Delim(&'a str),
        Operator(&'a str),
        
        // Literals
        NumLiteral(i32),
        FloatLiteral(f32),
        StringLiteral(&'a str),
        BoolLiteral(bool),

        // Other

        Keyword(&'a str),
        Identifier(&'a str),
    }
}
