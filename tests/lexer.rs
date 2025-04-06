#[cfg(test)]
mod lexer_tests {
    mod keywords_and_identifiers{
        use xinsh::lexer::lexer;
        use xinsh::lexer::tokens::Token;

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
        use xinsh::lexer::lexer;
        use xinsh::lexer::tokens::Token;

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

        #[test]
        fn diff_text_vs_bool(){
            let source = r#""True" True "False" False"#;
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::StringLiteral("True".to_string()),
                Token::BoolLiteral(true),
                Token::StringLiteral("False".to_string()),
                Token::BoolLiteral(false)
            ]);
        }

        #[test]
        fn diff_string_keyword_identifier(){
            let source = r#""echo" echo "myvar" myvar "#;
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::StringLiteral("echo".to_string()),
                Token::Keyword("echo".to_string()),
                Token::StringLiteral("myvar".to_string()),
                Token::Identifier("myvar".to_string())
            ]);
        }

        #[test]
        fn stupid_strings(){
            let source = r#""it's cool to use this project's lexer" 'I like this "thing"'"#;
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::StringLiteral("it's cool to use this project's lexer".to_string()),
                Token::StringLiteral(r#"I like this "thing""#.to_string())
            ]);
        }
    }

    mod delims_and_operators{
        use xinsh::lexer::lexer;
        use xinsh::lexer::tokens::Token;

        #[test]
        fn single_operators(){
            let source = "+ - = *";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::Operator("+".to_string()),
                Token::Operator("-".to_string()),
                Token::Operator("=".to_string()),
                Token::Operator("*".to_string())
            ]);
        }

        #[test]
        fn single_delims(){
            let source = r"[[ , ; : { [ ( )]]";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::Delim("[[".to_string()),
                Token::Delim(",".to_string()),
                Token::Delim(";".to_string()),
                Token::Delim(":".to_string()),
                Token::Delim(r"{".to_string()),
                Token::Delim("[".to_string()),
                Token::Delim("(".to_string()),
                Token::Delim(")".to_string()),
                Token::Delim("]]".to_string())
            ]);
        }

        #[test]
        fn mixed_delims_and_operators(){
            let source = r"{ / = << >> > % }}";
            let tokens = lexer::lex(source.to_string());
            assert_eq!(tokens, vec![
                Token::Delim(r"{".to_string()),
                Token::Operator("/".to_string()),
                Token::Operator("=".to_string()),
                Token::Operator("<".to_string()),
                Token::Operator("<".to_string()),
                Token::Operator(">>".to_string()),
                Token::Operator(">".to_string()),
                Token::Operator("%".to_string()),
                Token::Delim("}".to_string()),
                Token::Delim("}".to_string())
            ]);
        }
    }

    mod real_source_code{
        use std::fs;
        use xinsh::lexer::lexer;
        use xinsh::lexer::tokens::Token;

        #[test]
        fn simple_io(){
            let source = fs::read_to_string("./tests/source-examples/simpleIO.xnsh").expect("File should exist");
            let tokens = lexer::lex(source);
            assert_eq!(tokens, vec![
                Token::Keyword("echo".to_string()),
                Token::StringLiteral(r"Hello, World\n".to_string()),

                Token::Delim(";".to_string()),

                Token::Keyword("echo".to_string()),
                Token::StringLiteral("Enter your name: ".to_string()),

                Token::Delim(";".to_string()),
                Token::Keyword("read".to_string()),
                Token::Identifier("name".to_string()),

                Token::Delim(";".to_string()),
                Token::Keyword("echo".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("name".to_string()),

                Token::Delim(";".to_string()),

                Token::Identifier("x".to_string()),
                Token::Operator("=".to_string()),
                Token::NumLiteral(9),

                Token::Delim(";".to_string()),
                Token::Identifier("y".to_string()),
                Token::Operator("=".to_string()),
                Token::FloatLiteral(9.0),

                Token::Delim(";".to_string()),
                Token::Identifier("z".to_string()),
                Token::Operator("=".to_string()),
                Token::BoolLiteral(true),

                Token::Delim(";".to_string()),

                Token::Keyword("echo".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("x".to_string()),

                Token::Delim(";".to_string()),


                Token::Keyword("echo".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("y".to_string()),

                Token::Delim(";".to_string()),

                Token::Keyword("echo".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("z".to_string()),

                Token::Delim(";".to_string())
            ]);
        }

        #[test]
        fn ctrl_flow(){
            let source = fs::read_to_string("./tests/source-examples/ctrlFlow.xnsh").expect("File should exist");
            let tokens = lexer::lex(source);
            assert_eq!(tokens, vec![
                Token::Keyword("echo".to_string()),
                Token::StringLiteral("Write a num: ".to_string()),
                Token::Delim(";".to_string()),

                Token::Keyword("read".to_string()),
                Token::Identifier("innum".to_string()),

                Token::Delim(";".to_string()),

                Token::Keyword("text2num".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("innum".to_string()),

                Token::Delim(";".to_string()),

                Token::Keyword("if".to_string()),
                Token::Delim("[".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("innum".to_string()),
                Token::Operator("=".to_string()),
                Token::NumLiteral(2),
                Token::Delim("]".to_string()),
                Token::Delim(";".to_string()),
                Token::Keyword("then".to_string()),

                Token::Keyword("echo".to_string()),
                Token::StringLiteral("Yup".to_string()),
                
                Token::Delim(";".to_string()),
                Token::Keyword("else".to_string()),

                Token::Keyword("echo".to_string()),
                Token::StringLiteral("No".to_string()),

                Token::Delim(";".to_string()),
                Token::Keyword("fi".to_string()),

                Token::Delim(";".to_string())
            ]);
        }

        #[test]
        fn while_loops(){
            let source = fs::read_to_string("./tests/source-examples/whileloop.xnsh").expect("File should exist");
            let tokens = lexer::lex(source);
            assert_eq!(tokens, vec![
                Token::Identifier("x".to_string()),
                Token::Operator("=".to_string()),
                Token::NumLiteral(1),

                Token::Delim(";".to_string()),
                Token::Keyword("while".to_string()),
                Token::Delim("[".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("x".to_string()),
                Token::Operator(">".to_string()),
                Token::NumLiteral(5),
                Token::Delim("]".to_string()),
                Token::Delim(";".to_string()),
                Token::Keyword("do".to_string()),

                Token::Keyword("echo".to_string()),
                Token::StringLiteral("x is ".to_string()),
                Token::Operator("+".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("x".to_string()),

                Token::Delim(",".to_string()),
                Token::Identifier("x".to_string()),
                Token::Operator("=".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("x".to_string()),
                Token::Operator("+".to_string()),
                Token::NumLiteral(1),

                Token::Delim(";".to_string()),
                Token::Keyword("done".to_string()),

                Token::Delim(";".to_string())
            ]);
        }

        #[test]
        fn for_loops(){
            let source = fs::read_to_string("./tests/source-examples/forloop.xnsh").expect("File should exist");
            let tokens = lexer::lex(source);
            assert_eq!(tokens, vec![
                Token::Keyword("for".to_string()),
                Token::Identifier("word".to_string()),
                Token::Keyword("in".to_string()),
                Token::Delim("(".to_string()),
                Token::StringLiteral("hello".to_string()),
                Token::Delim(",".to_string()),
                Token::StringLiteral("world".to_string()),
                Token::Delim(",".to_string()),
                Token::StringLiteral("shell".to_string()),
                Token::Delim(",".to_string()),
                Token::StringLiteral("script".to_string()),
                Token::Delim(")".to_string()),
                Token::Delim(";".to_string()),
                Token::Keyword("do".to_string()),

                Token::Keyword("echo".to_string()),
                Token::StringLiteral("Word: ".to_string()),
                Token::Operator("+".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("word".to_string()),

                Token::Delim(";".to_string()),
                Token::Keyword("done".to_string()),

                Token::Delim(";".to_string()),
                Token::Keyword("for".to_string()),
                Token::Identifier("i".to_string()),
                Token::Keyword("in".to_string()),
                Token::Delim("{".to_string()),
                Token::NumLiteral(1),
                Token::Operator("..".to_string()),
                Token::NumLiteral(5),
                Token::Delim("}".to_string()),
                Token::Delim(";".to_string()),
                Token::Keyword("do".to_string()),

                Token::Keyword("echo".to_string()),
                Token::StringLiteral("i is ".to_string()),
                Token::Operator("+".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("i".to_string()),

                Token::Delim(";".to_string()),
                Token::Keyword("done".to_string()),

                Token::Delim(";".to_string())
            ]);
        }
        
        #[test]
        fn functions(){
            let source = fs::read_to_string("./tests/source-examples/customfuncs.xnsh").expect("File should exist");
            let tokens = lexer::lex(source);
            assert_eq!(tokens, vec![
                Token::Keyword("function".to_string()),
                Token::Identifier("foo".to_string()),
                Token::Delim(r"{".to_string()),

                Token::Keyword("echo".to_string()),
                Token::StringLiteral("bar".to_string()),
                Token::Delim(";".to_string()),
                Token::Delim(r"}".to_string()),
                Token::Delim(";".to_string()),

                Token::Keyword("function".to_string()),
                Token::Identifier("bar".to_string()),
                Token::Delim("[[".to_string()),
                Token::Identifier("msg".to_string()),
                Token::Delim(":".to_string()),
                Token::Keyword("text".to_string()),
                Token::Delim("]]".to_string()),
                Token::Delim(r"{".to_string()),

                Token::Keyword("echo".to_string()),
                Token::Operator("$".to_string()),
                Token::Identifier("msg".to_string()),
                Token::Delim(";".to_string()),
                Token::Delim(r"}".to_string()),
                Token::Delim(";".to_string()),

                Token::Identifier("foo".to_string()),
                Token::Delim(";".to_string()),              
                Token::Identifier("bar".to_string()),
                Token::Delim("[[".to_string()),
                Token::StringLiteral("hi".to_string()),
                Token::Delim("]]".to_string()),
                Token::Delim(";".to_string())
            ]);
        }
        
        #[test]
        fn file_managment(){
            let source = fs::read_to_string("./tests/source-examples/file.xnsh").expect("File should exist");
            let tokens = lexer::lex(source);
            assert_eq!(tokens, vec![
                Token::Keyword("if".to_string()),
                Token::Delim("[".to_string()),
                Token::Keyword("file".to_string()),
                Token::StringLiteral("file.txt".to_string()),
                Token::Delim("]".to_string()),
                Token::Delim(";".to_string()),
                Token::Keyword("then".to_string()),

                Token::Keyword("cat".to_string()),
                Token::StringLiteral("file.txt".to_string()),
                Token::Delim(",".to_string()),              
                Token::Keyword("cat".to_string()),
                Token::StringLiteral("Hiiii".to_string()),
                Token::Operator(">>".to_string()),
                Token::StringLiteral("file.txt".to_string()),
                Token::Delim(",".to_string()),               
                Token::Keyword("cat".to_string()),
                Token::StringLiteral("Hello".to_string()),
                Token::Operator(">".to_string()),
                Token::StringLiteral("file.txt".to_string()),
                Token::Delim(",".to_string()),
                Token::Keyword("cp".to_string()),
                Token::StringLiteral("file.txt".to_string()),
                Token::StringLiteral("hello.txt".to_string()),
                Token::Delim(",".to_string()),
                Token::Keyword("mv".to_string()),
                Token::StringLiteral("hello.txt".to_string()),
                Token::StringLiteral("no.txt".to_string()),
                Token::Delim(";".to_string()),
                Token::Keyword("else".to_string()),

                Token::Keyword("echo".to_string()),
                Token::StringLiteral("File does not exist".to_string()),
                Token::Delim(";".to_string()),
                Token::Keyword("fi".to_string()),
                Token::Delim(";".to_string())
            ]);
        }

        #[test]
        fn shell(){
           let source = fs::read_to_string("./tests/source-examples/sh.xnsh").expect("File should exist");
           let tokens = lexer::lex(source);
           assert_eq!(tokens, vec![
               Token::Keyword("command".to_string()),
               Token::StringLiteral("ls".to_string()),
               
               Token::Delim(";".to_string()),
                
               Token::Keyword("command".to_string()),
               Token::StringLiteral(r#"echo "Hello""#.to_string()),

                Token::Delim(";".to_string())
           ]);
        }
    }
}
