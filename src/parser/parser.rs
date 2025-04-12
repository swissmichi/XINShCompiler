use crate::lexer::tokens::Token;
use crate::parser::ast::*;

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut i = 0;
    let mut ast = Vec::new();
    while i < tokens.len() {
        if tokens[i] == Token::Delim(";".to_string()) {
            panic!("; is not a valid statement");
        }
        let mut search_area = Vec::new();
        'search: loop {
            match tokens[i].clone() {
                Token::Delim(semi) if semi == ";" => {
                    search_area.push(tokens[i].clone());
                    i += 1;
                    break 'search;
                }
                _ => {
                    search_area.push(tokens[i].clone());
                    i += 1;
                }
            }
        }
        match handle_var_def(search_area) {
            Some(s) => ast.push(s),
            None => {}
        }
    }
    ast
}

fn handle_var_def(search_area: Vec<Token>) -> Option<Stmt> {
    let mut j = 0;
    while j < search_area.len() {
        match search_area[j].clone() {
            Token::Identifier(name) => match search_area[j + 1].clone() {
                Token::Delim(colon) if colon == ":".to_string() => {
                    if search_area[j + 3] != Token::Operator("=".to_string()) {
                        match search_area[j + 2].clone() {
                            Token::Keyword(keyword) => match keyword.as_str() {
                                "num" => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Int(0)),
                                    }));
                                }
                                "float" => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Float(0.0)),
                                    }));
                                }
                                "str" => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Str("".to_string())),
                                    }));
                                }
                                "bool" => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Bool(false)),
                                    }));
                                }
                                "list" => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::List(Vec::new())),
                                    }));
                                }
                                "range" => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Range(
                                            Box::new(Lit::Int(0)),
                                            Box::new(Lit::Int(0)),
                                        )),
                                    }));
                                }
                                _ => panic!("Unknown Datatype: {}", keyword),
                            },
                            _ => panic!("Expected Keyword"),
                        }
                    } else if search_area[j + 3] == Token::Operator("=".to_string()) {
                        if search_area.len() == 6 {
                            match search_area[j + 4].clone() {
                                Token::NumLiteral(num) => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Int(num)),
                                    }));
                                }
                                Token::FloatLiteral(num) => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Float(num)),
                                    }));
                                }
                                Token::StringLiteral(str) => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Str(str)),
                                    }));
                                }
                                Token::BoolLiteral(bool) => {
                                    return Some(Stmt::Assign(Assign {
                                        name: name,
                                        val: Expr::Lit(Lit::Bool(bool)),
                                    }));
                                }
                                _ => panic!("Expected Literal"),
                            }
                        } else if search_area[j + 4] == Token::Delim("(".to_string()) {
                            let mut list = Vec::new();
                            for k in j + 4..search_area.len() {
                                match search_area[k].clone() {
                                    Token::Delim(rbracket) if rbracket == ")".to_string() => break,
                                    Token::Delim(comma) if comma == ",".to_string() => {}
                                    _ => list.push(search_area[k].clone()),
                                }
                            }
                            let list_expr = handle_list(list).0;
                            return Some(Stmt::Assign(Assign {
                                name: name,
                                val: list_expr,
                            }));
                        } else if matches!(search_area[j + 5], Token::Operator(_)) {
                            return Some(Stmt::Assign(Assign {
                                name: name,
                                val: handle_bin_op(search_area[4..].to_vec()).0,
                            }));
                        }
                    }
                }
                Token::Operator(op) if op == "=".to_string() => {
                    if search_area.len() == 4 {
                        match search_area[j + 2].clone() {
                            Token::NumLiteral(num) => {
                                return Some(Stmt::Assign(Assign {
                                    name: name,
                                    val: Expr::Lit(Lit::Int(num)),
                                }));
                            }
                            Token::FloatLiteral(num) => {
                                return Some(Stmt::Assign(Assign {
                                    name: name,
                                    val: Expr::Lit(Lit::Float(num)),
                                }));
                            }
                            Token::StringLiteral(str) => {
                                return Some(Stmt::Assign(Assign {
                                    name: name,
                                    val: Expr::Lit(Lit::Str(str)),
                                }));
                            }
                            Token::BoolLiteral(bool) => {
                                return Some(Stmt::Assign(Assign {
                                    name: name,
                                    val: Expr::Lit(Lit::Bool(bool)),
                                }));
                            }
                            _ => panic!("Expected Literal"),
                        }
                    } else if search_area[j + 2] == Token::Delim("(".to_string()) {
                        let mut list = Vec::new();
                        for k in j + 2..search_area.len() {
                            match search_area[k].clone() {
                                Token::Delim(rbracket) if rbracket == ")".to_string() => break,
                                Token::Delim(comma) if comma == ",".to_string() => {}
                                _ => list.push(search_area[k].clone()),
                            }
                        }
                        let list_expr = handle_list(list).0;
                        return Some(Stmt::Assign(Assign {
                            name: name,
                            val: list_expr,
                        }));
                    } else if matches!(search_area[j + 3], Token::Operator(_)) {
                        return Some(Stmt::Assign(Assign {
                            name: name,
                            val: handle_bin_op(search_area[2..].to_vec()).0,
                        }));
                    }
                }
                _ => println!("TODO"),
            },
            _ => println!("TODO"),
        }
        j += 1;
    }
    None
}

fn handle_list(list: Vec<Token>) -> (Expr, usize) {
    let mut i = 0;
    match list[i].clone() {
        Token::Delim(lbracket) if lbracket == "(".to_string() => {
            i += 1;
        }
        _ => panic!("Expected ("),
    }
    let mut list_expr = Vec::new();
    while i < list.len() {
        match list[i].clone() {
            Token::NumLiteral(num) => {
                list_expr.push(Lit::Int(num));
            }
            Token::FloatLiteral(num) => {
                list_expr.push(Lit::Float(num));
            }
            Token::StringLiteral(str) => {
                list_expr.push(Lit::Str(str));
            }
            Token::BoolLiteral(bool) => {
                list_expr.push(Lit::Bool(bool));
            }
            Token::Delim(rbracket) if rbracket == ")".to_string() => break,
            Token::Delim(comma) if comma == ",".to_string() => {}
            _ => panic!("Expected Literal"),
        }
        i += 1;
    }
    (Expr::Lit(Lit::List(list_expr)), i + 1)
}

fn handle_bin_op(tokens: Vec<Token>) -> (Expr, usize) {
    if tokens.len() < 3 {
        panic!("Expected more tokens");
    } else if tokens.len() == 4 {
        match tokens[3].clone() {
            Token::Delim(semi) if semi == ";".to_string() => {}
            _ => panic!("WEIRDDD"),
        }
        match tokens[1].clone() {
            Token::Operator(op) if op == "+" => {
                return match (&tokens[0], &tokens[2]) {
                    (Token::NumLiteral(num1), Token::NumLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Int(*num1))),
                            BinOp::Add,
                            Box::new(Expr::Lit(Lit::Int(*num2))),
                        ),
                        3,
                    ),
                    (Token::FloatLiteral(num1), Token::FloatLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Float(*num1))),
                            BinOp::Add,
                            Box::new(Expr::Lit(Lit::Float(*num2))),
                        ),
                        3,
                    ),
                    (Token::StringLiteral(str1), Token::StringLiteral(str2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Str(str1.clone()))),
                            BinOp::Add,
                            Box::new(Expr::Lit(Lit::Str(str2.clone()))),
                        ),
                        3,
                    ),
                    _ => panic!("Mismatched Types"),
                };
            }

            Token::Operator(op) if op == "-" => {
                return match (&tokens[0], &tokens[2]) {
                    (Token::NumLiteral(num1), Token::NumLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Int(*num1))),
                            BinOp::Sub,
                            Box::new(Expr::Lit(Lit::Int(*num2))),
                        ),
                        3,
                    ),
                    (Token::FloatLiteral(num1), Token::FloatLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Float(*num1))),
                            BinOp::Sub,
                            Box::new(Expr::Lit(Lit::Float(*num2))),
                        ),
                        3,
                    ),
                    _ => panic!("Mismatched Types"),
                };
            }

            Token::Operator(op) if op == "*" => {
                return match (&tokens[0], &tokens[2]) {
                    (Token::NumLiteral(num1), Token::NumLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Int(*num1))),
                            BinOp::Mul,
                            Box::new(Expr::Lit(Lit::Int(*num2))),
                        ),
                        3,
                    ),
                    (Token::FloatLiteral(num1), Token::FloatLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Float(*num1))),
                            BinOp::Mul,
                            Box::new(Expr::Lit(Lit::Float(*num2))),
                        ),
                        3,
                    ),
                    _ => panic!("Mismatched Types"),
                };
            }

            Token::Operator(op) if op == "/" => {
                return match (&tokens[0], &tokens[2]) {
                    (Token::NumLiteral(num1), Token::NumLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Int(*num1))),
                            BinOp::Div,
                            Box::new(Expr::Lit(Lit::Int(*num2))),
                        ),
                        3,
                    ),
                    (Token::FloatLiteral(num1), Token::FloatLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Float(*num1))),
                            BinOp::Div,
                            Box::new(Expr::Lit(Lit::Float(*num2))),
                        ),
                        3,
                    ),
                    _ => panic!("Mismatched Types"),
                };
            }

            Token::Operator(op) if op == "%" => {
                return match (&tokens[0], &tokens[2]) {
                    (Token::NumLiteral(num1), Token::NumLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Int(*num1))),
                            BinOp::Mod,
                            Box::new(Expr::Lit(Lit::Int(*num2))),
                        ),
                        3,
                    ),
                    (Token::FloatLiteral(num1), Token::FloatLiteral(num2)) => (
                        Expr::Bin(
                            Box::new(Expr::Lit(Lit::Float(*num1))),
                            BinOp::Mod,
                            Box::new(Expr::Lit(Lit::Float(*num2))),
                        ),
                        3,
                    ),
                    _ => panic!("Mismatched Types"),
                };
            }

            _ => panic!("Expected Operator"),
        }
    }

    panic!("SOMETHING WENT WRONG");
}
