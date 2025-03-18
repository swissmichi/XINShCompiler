// Lexer module
pub mod lexer{
    use lexer::tokens;
    use std::fs;
    use regex::Regex;
    // Function takes file adress as input and returns a vector of tokens. 
    pub fn lex(file: &str) -> Vec![tokens::Token] {
        static mut cursor: usize;
        let re_comment = Regex::new(r"(#.*\n)");
        let re_delim = Regex::new(r#"("|\[{1,2}|\]{1,2}|\(|\)|\{|\}|,|:|;)"#);
    }
}
