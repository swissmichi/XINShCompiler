// TODO Add more tests
#[cfg(test)]
mod lexer_tests {
    mod keywords_and_identifiers{
        use XINSh::lexer;
        use XINSh::tokens::Token;

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
            ])
        }
    }
}
