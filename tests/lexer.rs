// TODO Add more tests
#[cfg(test)]
mod tests {
    use XINSh::lexer;
    use XINSh::tokens::Token;

    #[test]
    fn lex_simple_keywords(){
        let source = "echo read num";
        let tokens = lexer::lex(source.to_string());
        assert_eq!(tokens, vec![
            Token::Keyword("echo".to_string()),
            Token::Keyword("read".to_string()),
            Token::Keyword("num".to_string())
        ]);
    }
    #[test]
    fn lex_simple_identifier(){
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
    fn lex_simple_keywords_vs_identifiers(){
        let source = "num2text xyz bool2num hiii";
        let tokens = lexer::lex(source.to_string());
        assert_eq!(tokens, vec![
            Token::Keyword("num2text".to_string()),
            Token::Identifier("xyz".to_string()),
            Token::Keyword("bool2num".to_string()),
            Token::Identifier("hiii".to_string())
        ]);
    }
}
