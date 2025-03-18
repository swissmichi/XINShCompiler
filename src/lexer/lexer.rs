// Lexer module
pub mod lexer{
    use lexer::tokens;
    use std::fs;
    use regex::Regex;
    // Function takes file adress as input and returns a vector of tokens. 
    pub fn lex(file_path: &str) -> Vec![tokens::Token] {
        let mut cursor: usize = 0;
        let mut tokens_vec: Vec<tokens::Token> = Vec::new();
        let re_comment = Regex::new(r"^(#.*\n)").unwrap();
        let re_delim = Regex::new(r#"^("|\[{1,2}|\]{1,2}|\(|\)|\{|\}|,|:|;)"#).unwrap();
        let re_operator = Regex::new(r"^(=|\+|\-|/|\*|<|>|\$)").unwrap();
        
        let re_num_lit = Regex::new(r"^([0-9]+)").unwrap();
        let re_float_lit = Regex::new(r"^([0-9]+.[0-9]+)").unwrap();
        let re_string_lit = Regex::new(r#"^"(.*)""#).unwrap();
        let re_bool_lit = Regex::new(r"^(True|False)").unwrap();

        let keyword_identifier = Regex::new(r"^([^\W0-9]*)").unwrap();
        
        let source = fs::read_to_string(file_path)
            .expect("Should have been able to read file");
        
        while cursor < source.len() {
            if re_comment.is_match(source[cursor..]){
                cursor += re_comment.captures(source[cursor..])
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .len();
                continue
            } else if re_delim.is_match(source[cursor..]){
                 tokens_vec.push(tokens::Token::Delim(re_delim.captures(source[cursor..])
                         .unwrap()
                         .get(1)
                         .unwrap()
                         .as_str()));           
                cursor += re_delim.captures(source[cursor..])
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .len();
                continue
            } else if re_operator.is_match(source[cursor..]){
                tokens_vec.push(tokens::Token::Operator(re_delim(source[cursor..])
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()));
                cursor += re_operator.captures(source[cursor..])
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .len();
                conrinue
            } else if re_num_lit.is_match(source[cursor..]){
                    tokens_vec.push(tokens::Token::NumLiteral(re_num_lit.captures(source[cursor..])
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse::<i32>()
                            .expect("Should have been an integer")));
                    cursor += re_num_lit.captures(source[cursor..])
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .as_str()
                        .len();
                    continue
            } else if re_float_lit.is_match(source[cursor..]){
                tokens_vec.push(tokens::Token::FloatLiteral(re_float_lit.captures(source[cursor..])
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<f32>()
                        .expect("Should have been a float")));
                cursor += re_float_lit.captures(source[cursor..])
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .len();
                continue
            } // TODO: Continue the pattern with StringLiteral. Test
        }
    }
}
