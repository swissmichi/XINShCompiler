use crate::lexer::tokens::Token;
use crate::parser::ast;
use crate::parser::s_expr::S;

pub fn sexpr(tokens: Vec<Token>) -> S{
    let mut prog_vec = Vec::new();
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Identifier(_) => {
                    let name = match token {
                            Token::Identifier(name) => name,
                        _ => panic!("Expected Identifier token"), // Handle other variants as needed
                    };


                    let nt_one = tokens[i + 1].clone();
                    let nt_two = tokens[i + 2].clone();
                    let nt_three = tokens[i + 3].clone();

                    if nt_one == Token::Delim(":".to_string()) &&
                    matches!(nt_two, Token::Keyword(_)) {
                        if nt_three == Token::Operator("=".to_string()) {

                        } else {
                            match nt_two {
                                Token::Keyword(ref kw) => {
                                    if kw == "num" {
                                        prog_vec.push(S::Cons("=num".to_string(), vec![
                                            S::Atom(name.to_string()),    
                                            S::Atom("0".to_string())
                                        ]));
                                    } else if kw == "bool" {
                                        prog_vec.push(S::Cons("=bool".to_string(), vec![
                                            S::Atom(name.to_string()),    
                                            S::Atom("true".to_string())
                                        ]));
                                    } else if kw == "float" {
                                        prog_vec.push(S::Cons("=float".to_string(), vec![
                                            S::Atom(name.to_string()),    
                                            S::Atom("0.0".to_string())
                                        ]));
                                    } else if kw == "list" {
                                        prog_vec.push(S::Cons("=list".to_string(), vec![
                                            S::Atom(name.to_string()),    
                                            S::Cons("list".to_string(), vec![])
                                        ]));
                                    } else {
                                        panic!("Expected a datatype");
                                    }
                                },

                                _ => panic!("Expected a datatype")
                            }
                        }
                    }
                },
                _ => {
                    
                }
            }
        }

        S::Cons("prog".to_string(), prog_vec)
}

