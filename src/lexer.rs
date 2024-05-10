use chumsky::{
    error::Rich,
    extra,
    primitive::{end, one_of},
    span::SimpleSpan,
    text, IterParser, Parser,
};

use crate::token::{Keyword, Token};

pub type Span = SimpleSpan<usize>;

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, Span)>, extra::Err<Rich<'src, char, Span>>> {
    let ident = text::ident().map(|ident| match ident {
        "select" | "SELECT" => Token::Keyword(Keyword::Select),
        "from" | "FROM" => Token::Keyword(Keyword::From),
        ident => Token::Ident(ident),
    });
    ident
        .map_with(|i, s| (i, s.span()))
        .padded_by(one_of(" \t").labelled("whitespace").repeated())
        .repeated()
        .collect::<Vec<_>>()
        .padded()
        .then_ignore(end())
}
