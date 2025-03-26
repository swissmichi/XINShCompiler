// TODO Add more tests
#[cfg(test)]
mod lexer_tests {
    mod keywords_and_identifiers{
        use XINSh::lexer::lexer;
        use XINSh::lexer::tokens::Token;

        #[test]
        fn lex_single_keywords(){
            let source = "echo read num";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::Keyword("echo".to_string()),
                Token::Keyword("read".to_string()),
                Token::Keyword("num".to_string())
            ]);
        }

        #[test]
        fn lex_single_identifier(){
            let source = "name x y hellooooo";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::Identifier("name".to_string()),
                Token::Identifier("x".to_string()),
                Token::Identifier("y".to_string()),
                Token::Identifier("hellooooo".to_string())
            ]);
        }

        #[test]
        fn diff_keywords_vs_identifiers(){
            let source = "num2text xyz bool2num hiii";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::Keyword("num2text".to_string()),
                Token::Identifier("xyz".to_string()),
                Token::Keyword("bool2num".to_string()),
                Token::Identifier("hiii".to_string())
            ]);
        }

        #[test]
        fn lex_stupid_identifiers(){
            let source = "Helloo_world fdkksafdsjosjisajidsajisa hUHudhUssIJduisaiJHjdnadhsuwjnsajduH99";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::Identifier("Helloo_world".to_string()),
                Token::Identifier("fdkksafdsjosjisajidsajisa".to_string()),
                Token::Identifier("hUHudhUssIJduisaiJHjdnadhsuwjnsajduH99".to_string())
            ]);
        }

        #[test]
        fn case_sensitive_keywords(){
            let source = "file File fiLe dir dIr dIR";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::Keyword("file".to_string()),
                Token::Identifier("File".to_string()),
                Token::Identifier("fiLe".to_string()),
                Token::Keyword("dir".to_string()),
                Token::Identifier("dIr".to_string()),
                Token::Identifier("dIR".to_string())
            ]);
        }
    }

    mod literals{
        use XINSh::lexer::lexer;
        use XINSh::lexer::tokens::Token;

        #[test]
        fn single_string_literals(){
            let source = r#""hello" 'there'"#;
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::StringLiteral("hello".to_string()),
                Token::StringLiteral("there".to_string())
            ]);
        }

        #[test]
        fn single_num_literals(){
            let source = "9 000 -893";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::NumLiteral(9),
                Token::NumLiteral(0),
                Token::NumLiteral(-893)
            ]);
        }

        #[test]
        fn single_float_literals(){
            let source = "-8.33 29.100 0.0";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::FloatLiteral(-8.33),
                Token::FloatLiteral(29.100),
                Token::FloatLiteral(0.0)
            ]);
        }

        #[test]
        fn single_bool_literals(){
            let source = "True False";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::BoolLiteral(true),
                Token::BoolLiteral(false)
            ]);
        }

        #[test]
        fn diff_num_vs_float(){
            let source = "899 0 0.0 -899.0 -899";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::NumLiteral(899),
                Token::NumLiteral(0),
                Token::FloatLiteral(0.0),
                Token::FloatLiteral(-899.0),
                Token::NumLiteral(-899)
            ]);
        }
    }
}
