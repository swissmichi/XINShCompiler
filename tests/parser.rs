#[cfg(test)]

pub mod parser_test{
    use xinsh::lexer::tokens::Token;
    use xinsh::parser::parser;
    use xinsh::parser::ast::*;

    #[test]
    fn defining_vars(){
        let tokens = vec![
            Token::Identifier("a".to_string()),
            Token::Delim(":".to_string()),
            Token::Keyword("num".to_string()),
            Token::Delim(";".to_string()),
            
            Token::Identifier("b".to_string()),
            Token::Operator("=".to_string()),
            Token::BoolLiteral(true),


            Token::Delim(";".to_string()),
            Token::Identifier("c".to_string()),
            Token::Delim(":".to_string()),
            Token::Keyword("list".to_string()),
            Token::Operator("=".to_string()),
            Token::Delim("(".to_string()),
            Token::NumLiteral(1),
            Token::Delim(",".to_string()),
            Token::NumLiteral(2),
            Token::Delim(")".to_string()),

            Token::Delim(";".to_string())
        ];

        let ast = parser::parse(tokens); 

        assert_eq!(ast, vec![
            Stmt::Assign(Assign{
                name: "a".to_string(),
                val: Expr::Lit(Lit::Int(0))
            }),
            Stmt::Assign(Assign{
                name: "b".to_string(),
                val: Expr::Lit(Lit::Bool(true))
            }),
            Stmt::Assign(Assign{
                name: "c".to_string(),
                val: Expr::Lit(Lit::List(vec![Lit::Int(1), Lit::Int(2)]))
            })
        ]);
    }

    #[test]
    fn simple_ops(){
        let tokens = vec![
            Token::Identifier("a".to_string()),
            Token::Operator("=".to_string()),
            Token::NumLiteral(1),
            Token::Operator("+".to_string()),
            Token::NumLiteral(2),
            Token::Delim(";".to_string()),

            Token::Identifier("a".to_string()),
            Token::Operator("=".to_string()),
            Token::NumLiteral(1),
            Token::Operator("-".to_string()),
            Token::NumLiteral(2),
            Token::Delim(";".to_string()),

            Token::Identifier("a".to_string()),
            Token::Operator("=".to_string()),
            Token::NumLiteral(1),
            Token::Operator("*".to_string()),
            Token::NumLiteral(2),
            Token::Delim(";".to_string()),

            
            Token::Identifier("a".to_string()),
            Token::Operator("=".to_string()),
            Token::NumLiteral(1),
            Token::Operator("/".to_string()),
            Token::NumLiteral(2),
            Token::Delim(";".to_string()),

            Token::Identifier("a".to_string()),
            Token::Operator("=".to_string()),
            Token::NumLiteral(1),
            Token::Operator("%".to_string()),
            Token::NumLiteral(2),
            Token::Delim(";".to_string()),

            Token::Identifier("a".to_string()),
            Token::Operator("=".to_string()),
            Token::Operator("!".to_string()),
            Token::BoolLiteral(true),
            Token::Delim(";".to_string()),
        ];

        let ast = parser::parse(tokens);

        assert_eq!(ast, vec![
            Stmt::Assign(Assign{
                name: "a".to_string(),
                val: Expr::Bin(
                    Box::new(Expr::Lit(Lit::Int(1))),
                    BinOp::Add,
                    Box::new(Expr::Lit(Lit::Int(2)))
                )
            }),
            Stmt::Assign(Assign{
                name: "a".to_string(),
                val: Expr::Bin(
                    Box::new(Expr::Lit(Lit::Int(1))),
                    BinOp::Sub,
                    Box::new(Expr::Lit(Lit::Int(2)))
                )
            }),
            Stmt::Assign(Assign{
                name: "a".to_string(),
                val: Expr::Bin(
                    Box::new(Expr::Lit(Lit::Int(1))),
                    BinOp::Mul,
                    Box::new(Expr::Lit(Lit::Int(2)))
                )
            }),
            Stmt::Assign(Assign{
                name: "a".to_string(),
                val: Expr::Bin(
                    Box::new(Expr::Lit(Lit::Int(1))),
                    BinOp::Div,
                    Box::new(Expr::Lit(Lit::Int(2)))
                )
            }),
            Stmt::Assign(Assign{
                name: "a".to_string(),
                val: Expr::Un(
                    UnOp::Not,
                    Box::new(Expr::Lit(Lit::Bool(true)))
                )
            })
        ])
    }
}
