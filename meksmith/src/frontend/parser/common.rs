use chumsky::{Parser, input::ValueInput, primitive::just, select};

use crate::frontend::{
    span::SimpleSpan,
    token::{Delimiter, Keyword, Literal, Operator, TokenKind},
};

use super::ParserError;

pub(crate) fn keyword<'src, I>(
    expected: Keyword,
) -> impl Parser<'src, I, (), ParserError<'src>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    just(TokenKind::Keyword(expected)).ignored()
}

pub(crate) fn ident<'src, I>() -> impl Parser<'src, I, String, ParserError<'src>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    select! {
        TokenKind::Identifier(name) => name
    }
}

pub(crate) fn literal<'src, I>() -> impl Parser<'src, I, Literal, ParserError<'src>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    select! {
        TokenKind::Literal(literal) => literal
    }
}

pub(crate) fn integer_literal<'src, I>() -> impl Parser<'src, I, Literal, ParserError<'src>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    select! {
        TokenKind::Literal(literal @ Literal::Integer { .. }) => literal
    }
}

pub(crate) fn operator<'src, I>(
    expected: Operator,
) -> impl Parser<'src, I, (), ParserError<'src>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    just(TokenKind::Operator(expected)).ignored()
}

pub(crate) fn delimiter<'src, I>(
    expected: Delimiter,
) -> impl Parser<'src, I, (), ParserError<'src>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    just(TokenKind::Delimiter(expected)).ignored()
}
