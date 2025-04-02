use super::tokens;
use regex::Regex;
use std::collections::HashSet;

/// Lexing a string into tokens
///
/// # Example
/// ```
/// use xinsh::lexer::tokens::Token;
/// use xinsh::lexer::lexer;
/// let result = lexer::lex(r#"echo "Hello, world""#.to_string());
/// assert_eq!(result, vec![Token::Keyword("echo".to_string()), Token::StringLiteral("Hello, world".to_string())]);
/// ```
///
/// # Arguments
///
/// 'source' - source code as string (not str)
///
/// # Returns
///
/// lex() retunrs a vector of tokens.
///
/// # Panics
///
/// This function does not return panics
///
/// # Errors
///
/// This function does not return errors
///
/// # Safety
///
/// This function is safe
///
pub fn lex(source: String) -> Vec<tokens::Token> {
    let mut cursor: usize = 0;
let mut tokens_vec: Vec<tokens::Token> = Vec::new();

    // List of keywords
let keywords: HashSet<&str> = ["num", "float", "text", "bool", "list", "range", "function", "param", "return",
    "echo", "read", "num2float", "num2text", "num2bool", "num2list", "num2range", "float2num", "float2text", "float2list",
    "text2num", "text2float", "text2bool", "text2list", "bool2num", "bool2text", "bool2list", "if", "then",
    "else", "for", "while", "do", "done", "in", "dir", "file", "cat", "cp", "mv", "command", "fi"]
        .iter().cloned().collect();

    let re_comment = Regex::new(r"^#.*\n").unwrap(); 
    let re_whitespace = Regex::new(r"\s+").unwrap();
    let re_delim = Regex::new(r#"^(\[{1,2}|\]{1,2}|\(|\)|\{|\}|,|:|;)"#).unwrap();
    let re_operator = Regex::new(r"^(=|\+|\-|/|\*|<|>{1,2}|\$|%|\.\.|&|!=|\||&)").unwrap();
    
    let re_num_lit = Regex::new(r"^(-?[0-9]+)").unwrap();
    let re_float_lit = Regex::new(r"^(-?[0-9]+\.[0-9]+)").unwrap();
    let re_single_string = Regex::new(r#"^'([^']*)'"#).unwrap();
    let re_double_string = Regex::new(r#"^"([^"]*)""#).unwrap();

    let re_bool_lit = Regex::new(r"^(True|False)").unwrap();

    let re_keyword_identifier = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
            
    while cursor < source.len() {
        
        let result = match &source[cursor..] {
           
            s if re_comment.is_match(s) => { None }
            
            s if re_delim.is_match(s) => {
                let captures = re_delim.captures(s).unwrap();
                let value = captures.get(1).unwrap().as_str().to_string();
                let length = captures.get(0).unwrap().as_str().len();
                Some((tokens::Token::Delim(value), length))
            },

            s if re_float_lit.is_match(s) => {
                let captures = re_float_lit.captures(s).unwrap();
                let value = captures.get(1).unwrap().as_str().to_string();
                let length = captures.get(0).unwrap().as_str().len();
                Some((tokens::Token::FloatLiteral(value.parse::<f32>().unwrap()), length))
            },

            s if re_num_lit.is_match(s) => {
                let captures = re_num_lit.captures(s).unwrap();
                let value = captures.get(1).unwrap().as_str().to_string();
                let length = captures.get(0).unwrap().as_str().len();
                Some((tokens::Token::NumLiteral(value.parse::<i32>().unwrap()), length))
            },

            s if re_operator.is_match(s) => {
                let captures = re_operator.captures(s).unwrap();
                let value = captures.get(1).unwrap().as_str().to_string();
                let length = captures.get(0).unwrap().as_str().len();
                Some((tokens::Token::Operator(value), length))
            },

            s if re_single_string.is_match(s) => {
                let captures = re_single_string.captures(s).unwrap();
                let value = captures.get(1).unwrap().as_str().to_string();
                let length = captures.get(0).unwrap().as_str().len();
                Some((tokens::Token::StringLiteral(value), length))
            },

            s if re_double_string.is_match(s) => {
                let captures = re_double_string.captures(s).unwrap();
                let value = captures.get(1).unwrap().as_str().to_string();
                let length = captures.get(0).unwrap().as_str().len();
                Some((tokens::Token::StringLiteral(value), length))
            },

            s if re_bool_lit.is_match(s) => {
                let captures = re_bool_lit.captures(s).unwrap();
                let value = captures.get(1).unwrap().as_str().to_string();
                let length = captures.get(0).unwrap().as_str().len();
                Some((tokens::Token::BoolLiteral(value == "True"), length))
            },

            s if re_keyword_identifier.is_match(s) => {
                let captures = re_keyword_identifier.captures(s).unwrap();
                let value = captures.get(1).unwrap().as_str();
                let length = captures.get(0).unwrap().as_str().len();
                if keywords.contains(value) {
                    Some((tokens::Token::Keyword(value.to_string()), length))
                } else {
                    Some((tokens::Token::Identifier(value.to_string()), length))
                }
            },

            s if re_whitespace.is_match(s) => {
                None
            },

            _ => panic!("Unexpected token at {}", cursor)
        };

        match result {
            Some((token, length)) => {
                cursor += length;
                tokens_vec.push(token);
            },
            None => {
                let captures = re_comment
                    .captures(&source[cursor..])
                    .or_else(|| re_whitespace.captures(&source[cursor..]))
                    .expect("Should have matched a pattern which returns None");
                cursor += captures.get(0).unwrap().as_str().len();
            }
        }
    }
    tokens_vec
}


