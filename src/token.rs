use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Keyword(Keyword),
    Ident(&'src str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Select,
    From,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(kw) => write!(f, "{:?}", kw),
            Token::Ident(ident) => write!(f, "{}", ident),
        }
    }
}
