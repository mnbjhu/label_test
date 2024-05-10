use std::fs;

use chumsky::{error::Rich, extra, input::Input, primitive::just, select, Parser};

use crate::{
    lexer::Span,
    token::{Keyword, Token},
    util::ParserInput,
};

mod lexer;
mod token;
mod util;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let len = text.len();
    let (tokens, errors) = lexer::lexer().parse(&text).into_output_errors();
    println!("Lex errors");
    for error in errors {
        println!("{}", error);
    }

    if let Some(tokens) = tokens {
        println!("Tokens");
        for token in &tokens {
            println!("{:?}", token);
        }
        let input = tokens.spanned((len..len).into());
        let (ast, errors) = basic_parser().parse(input).into_output_errors();
        println!("Parse errors");
        for error in errors {
            println!("{}", error);
        }
        if let Some(ast) = ast {
            println!("AST");
            println!("{:?}", ast);
        }
    }
}

fn basic_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, String, extra::Err<Rich<'tokens, Token<'src>, Span>>>
{
    let ident = select! {
        Token::Ident(ident) => ident.to_string(),
    };
    just(Token::Keyword(Keyword::Select)).ignore_then(ident.labelled("column"))
}

#[cfg(test)]
mod test {

    use chumsky::{error::RichPattern, input::Input, Parser};

    use crate::token::{Keyword, Token};

    #[test]
    fn test_basic_parser() {
        let input = vec![
            (Token::Keyword(Keyword::Select), (0..6).into()),
            (Token::Ident("foo"), (7..10).into()),
        ];
        let err = super::basic_parser()
            .parse(input.spanned((10..10).into()))
            .into_errors();

        assert_eq!(err.len(), 0);
    }

    #[test]
    fn error_at_eoi() {
        let input = vec![(Token::Keyword(Keyword::Select), (0..6).into())];
        let err = super::basic_parser()
            .parse(input.spanned((6..6).into()))
            .into_errors();
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].expected().len(), 1);
        assert_eq!(
            err[0].expected().last().unwrap(),
            &RichPattern::Label("column")
        );
    }

    #[test]
    fn error_at_token() {
        let input = vec![
            (Token::Keyword(Keyword::Select), (0..6).into()),
            (Token::Keyword(Keyword::Select), (7..13).into()),
        ];
        let err = super::basic_parser()
            .parse(input.spanned((6..6).into()))
            .into_errors();
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].expected().len(), 1);
    }
}
