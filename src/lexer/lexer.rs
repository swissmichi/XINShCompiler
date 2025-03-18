// Lexer module
mod lexer{
    use lexer::tokens;
    use std::fs;
    use regex::Regex;
    
    fn lex(file: &str) -> Vec![tokens::Token] {
        static mut cursor: usize;
        let re_comment = Regex::new(r"(#.*\n)");
        let re_delim = Regex::new(r#"("|\[{1,2}|\]{1,2}|\(|\)|\{|\}|,|:|;)"#);
    }
}
