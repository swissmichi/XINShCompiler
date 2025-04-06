#[cfg(test)]

pub mod s_expr_test{
    use xinsh::lexer::tokens::Token;
    use xinsh::parser::parser;
    use xinsh::parser::s_expr::S;

    #[test]
    fn defining_vars(){
        let tokens = vec![
            Token::Identifier("a".to_string()),
            Token::Delim(":".to_string()),
            Token::Keyword("num".to_string()),
            
            Token::Identifier("b".to_string()),
            Token::Operator("=".to_string()),
            Token::BoolLiteral(true),

            Token::Identifier("c".to_string()),
            Token::Delim(":".to_string()),
            Token::Keyword("list".to_string()),
            Token::Operator("=".to_string()),
            Token::Delim("(".to_string()),
            Token::NumLiteral(1),
            Token::Delim(",".to_string()),
            Token::NumLiteral(2),
            Token::Delim(")".to_string())
        ];

        let sxpr = parser::sexpr(tokens); 

        assert_eq!(sxpr, S::Cons("prog".to_string(), vec![
                    S::Cons("=num".to_string(), vec![
                        S::Atom("a".to_string()),
                        S::Atom("0".to_string())
                    ]),
                    S::Cons("=bool".to_string(), vec![
                        S::Atom("b".to_string()),
                        S::Atom("true".to_string())
                    ]),
                    S::Cons("=list".to_string(), vec![
                        S::Atom("c".to_string()),
                        S::Cons("list".to_string(), vec![
                            S::Atom("1".to_string()),
                            S::Atom("2".to_string())
                        ])
                    ])
                ])
        );
    }
}
