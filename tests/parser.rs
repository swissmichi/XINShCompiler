#[cfg(test)]
mod parser_tests {

    mod variable_definitions{
        use xinsh::parser::ast;
        use xinsh::parser::parser;
        use xinsh::lexer::tokens::Token;

        #[test]
        fn var_create(){
            let tokens = vec![
                Token::Identifier("x".to_string()),
                Token::Delim(":".to_string()),
                Token::Keyword("num".to_string()),
                Token::Identifier("y".to_string()),
                Token::Delim(":".to_string()),
                Token::Keyword("float".to_string()),
                Token::Identifier("z".to_string()),
                Token::Delim(":".to_string()),
                Token::Keyword("text".to_string()),
                Token::Identifier("a".to_string()),
                Token::Delim(":".to_string()),
                Token::Keyword("bool".to_string()),
                Token::Identifier("b".to_string()),
                Token::Delim(":".to_string()),
                Token::Keyword("list".to_string()),
                Token::Identifier("c".to_string()),
                Token::Delim(":".to_string()),
                Token::Keyword("range".to_string()),
            ];
            let ast = parser::parse(tokens);
            assert_eq!(ast, vec![
                ast::Stmt::CreateVar(ast::CreateVar{
                    name: "x".to_string(),
                    wtype: "num".to_string(),
                }),
                ast::Stmt::CreateVar(ast::CreateVar{
                    name: "y".to_string(),
                    wtype: "float".to_string(),
                }),
                ast::Stmt::CreateVar(ast::CreateVar{
                    name: "z".to_string(),
                    wtype: "text".to_string(),
                }),
                ast::Stmt::CreateVar(ast::CreateVar{
                    name: "a".to_string(),
                    wtype: "bool".to_string(),
                }),
                ast::Stmt::CreateVar(ast::CreateVar{
                    name: "b".to_string(),
                    wtype: "list".to_string(),
                }),
                ast::Stmt::CreateVar(ast::CreateVar{
                    name: "c".to_string(),
                    wtype: "range".to_string(),
                }),
            ]);
        }
    }

}

