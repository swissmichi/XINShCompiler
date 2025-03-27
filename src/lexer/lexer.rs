use super::tokens;
use regex::Regex;
use std::collections::HashSet;

/// Lexing a string into tokens
///
/// # Example
/// ```
/// use XINSh::lexer::tokens::Token;
/// use XINSh::lexer::lexer;
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
    let keywords: HashSet<&str> = ["num", "float", "text", "bool", "list", "range", "function", "param", "return",
    "echo", "read", "num2float", "num2text", "num2bool", "num2list", "num2range", "float2num", "float2text", "float2list",
    "text2num", "text2float", "text2bool", "text2list", "bool2num", "bool2text", "bool2list", "if", "then",
    "else", "for", "while", "do", "done", "in", "dir", "file", "cat", "cp", "mv", "command", "fi"]
        .iter().cloned().collect();

    let re_comment = Regex::new(r"^#.*\n").unwrap();
    let re_whitespace = Regex::new(r"\s+").unwrap();
    let re_delim = Regex::new(r#"^(\[{1,2}|\]{1,2}|\(|\)|\{|\}|,|:|;)"#).unwrap();
    let re_operator = Regex::new(r"^(=|\+|\-|/|\*|<|>{1,2}|\$|%|\.\.)").unwrap();
    
    let re_num_lit = Regex::new(r"^(-?[0-9]+)").unwrap();
    let re_float_lit = Regex::new(r"^(-?[0-9]+\.[0-9]+)").unwrap();
    let re_single_string = Regex::new(r#"^'([^']*)'"#).unwrap();
    let re_double_string = Regex::new(r#"^"([^"]*)""#).unwrap();

    let re_bool_lit = Regex::new(r"^(True|False)").unwrap();

    let re_keyword_identifier = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
            
    while cursor < source.len() {
        if let Some(token) = re_comment.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            continue;
        }
        if let Some(token) = re_delim.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            tokens_vec.push(tokens::Token::Delim(
                    token.get(1).unwrap().as_str().to_string()
            ));
            continue;
        }
        if let Some(token) = re_float_lit.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            tokens_vec.push(tokens::Token::FloatLiteral(
                    token.get(1).unwrap().as_str()
                    .parse::<f32>().expect("Should have been a float"),
            ));
            continue;
            } 
        if let Some(token) = re_num_lit.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            tokens_vec.push(tokens::Token::NumLiteral(
                    token.get(1).unwrap().as_str()
                    .parse::<i32>().expect("Should have been an integer"),
            ));
            continue;
        }
        if let Some(token) = re_operator.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            tokens_vec.push(tokens::Token::Operator(
                    token.get(1).unwrap().as_str().to_string()
            ));
            continue;
        }

        if let Some(token) = re_single_string.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            tokens_vec.push(tokens::Token::StringLiteral(
                token.get(1).unwrap().as_str().to_string()
            ));
            continue;
        }
        if let Some(token) = re_bool_lit.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            match token.get(1).unwrap().as_str() {
                "True" => tokens_vec.push(tokens::Token::BoolLiteral(true)),
                "False" => tokens_vec.push(tokens::Token::BoolLiteral(false)),
                _ => panic!("Should have been a bool"),
            }
            continue;
        }

        if let Some(token) = re_double_string.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            tokens_vec.push(tokens::Token::StringLiteral(
                token.get(1).unwrap().as_str().to_string()
            ));
            continue;
        }
        if let Some(token) = re_keyword_identifier.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            if keywords.contains(token.get(1).unwrap().as_str()) {
                tokens_vec.push(tokens::Token::Keyword(
                        token.get(1).unwrap().as_str().to_string()
                ));
            } else {
                tokens_vec.push(tokens::Token::Identifier(
                        token.get(1).unwrap().as_str().to_string()
                ));
            }
            continue;
        }
        if let Some(token) = re_whitespace.captures(&source[cursor..]) {
            cursor += token.get(0).unwrap().as_str().len();
            continue;
        }
        tokens_vec.push(tokens::Token::MalformedChar);
        cursor += 1;
        continue;
    }
    tokens_vec
}


