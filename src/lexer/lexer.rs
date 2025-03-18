// Lexer module
pub mod lexer{
    use lexer::tokens;
    use std::fs;
    use regex::Regex;
    // Function takes file adress as input and returns a vector of tokens. 
    pub fn lex(file: &str) -> Vec![tokens::Token] {
        static mut cursor: usize;
        let re_comment = Regex::new(r"^(#.*\n)");
        let re_delim = Regex::new(r#"^("|\[{1,2}|\]{1,2}|\(|\)|\{|\}|,|:|;)"#);
        let re_operator = Regex::new(r"^(=|\+|\-|/|\*|<|>|\$)");
        
        let re_num_lit = Regex::new(r"^([0-9]+)");
        let re_float_lit = Regex::new(r"^([0-9]+.[0-9]+)");
        let re_string_lit = Regex::new(r#"^"(.*)""#);
        let re_bool_lit = Regex::new(r"^(True|False)");

        let keyword_identifier = Regex::new(r"^([^\W0-9]*)");
    }
}
