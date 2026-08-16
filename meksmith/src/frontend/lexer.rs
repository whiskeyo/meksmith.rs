use chumsky::error::Simple;
use chumsky::primitive::{any, choice, just, none_of, one_of};
use chumsky::text::{ascii, whitespace};
use chumsky::{IterParser, Parser, extra};

use super::{
    error::Error,
    span::Span,
    token::{Delimiter, Keyword, Literal, Operator, Token, TokenKind, Tokens},
};

type ParserInput<'src> = &'src str;
type ParserError<'src> = extra::Err<Simple<'src, char>>;

pub fn lex_source(source: &str) -> Result<Tokens, Vec<Error>> {
    let result = lexer().parse(source).into_result();

    match result {
        Ok(tokens) => Ok(Tokens(tokens)),
        Err(errors) => {
            let error_list = errors
                .into_iter()
                .map(|e| Error::from_simple(source, &e))
                .collect();

            Err(error_list)
        }
    }
}

pub fn lexer<'src>() -> impl Parser<'src, ParserInput<'src>, Vec<Token>, ParserError<'src>> {
    lex_token().repeated().collect().then_ignore(trivia())
}

pub fn lex_token<'src>() -> impl Parser<'src, ParserInput<'src>, Token, ParserError<'src>> {
    trivia().ignore_then(token().map_with(|token_kind, extra| {
        let span = extra.span();

        Token {
            kind: token_kind,
            span: Span::new(span.start, span.end),
        }
    }))
}

fn trivia<'src>() -> impl Parser<'src, ParserInput<'src>, (), ParserError<'src>> {
    choice((whitespace().at_least(1).ignored(), line_comment()))
        .repeated()
        .ignored()
}

fn line_comment<'src>() -> impl Parser<'src, ParserInput<'src>, (), ParserError<'src>> {
    choice((just("//"), just("#")))
        .then(none_of('\n').repeated())
        .ignored()
}

fn token<'src>() -> impl Parser<'src, ParserInput<'src>, TokenKind, ParserError<'src>> {
    choice((literal(), ident_or_keyword(), operator(), delimiter()))
}

fn ident_or_keyword<'src>() -> impl Parser<'src, ParserInput<'src>, TokenKind, ParserError<'src>> {
    ascii::ident().map(|ident: &str| {
        if let Some(keyword) = Keyword::from_ident(ident) {
            TokenKind::Keyword(keyword)
        } else {
            match ident {
                "true" => TokenKind::Literal(Literal::Boolean(true)),
                "false" => TokenKind::Literal(Literal::Boolean(false)),
                "null" => TokenKind::Literal(Literal::Null),
                _ => TokenKind::Identifier(ident.to_string()),
            }
        }
    })
}

fn literal<'src>() -> impl Parser<'src, ParserInput<'src>, TokenKind, ParserError<'src>> {
    integer().map(TokenKind::Literal)
}

fn integer<'src>() -> impl Parser<'src, ParserInput<'src>, Literal, ParserError<'src>> {
    choice((
        parse_number("0x", 16, 16, |c| c.is_ascii_hexdigit()),
        parse_number("0b", 2, 64, |c| *c == '0' || *c == '1'),
        parse_number("", 10, 20, |c| c.is_ascii_digit()),
    ))
}

fn operator<'src>() -> impl Parser<'src, ParserInput<'src>, TokenKind, ParserError<'src>> {
    choice((
        just("..").to(Operator::DoubleDot),
        just("::").to(Operator::DoubleColon),
        just("=>").to(Operator::FatArrow),
        just('.').to(Operator::Dot),
        just('+').to(Operator::Plus),
        just('-').to(Operator::Minus),
        just('*').to(Operator::Star),
        just('/').to(Operator::Slash),
        just(',').to(Operator::Comma),
        just(';').to(Operator::Semicolon),
        just(':').to(Operator::Colon),
        just('=').to(Operator::Equal),
    ))
    .map(TokenKind::Operator)
}

fn delimiter<'src>() -> impl Parser<'src, ParserInput<'src>, TokenKind, ParserError<'src>> {
    one_of("(){}[]<>").map(|c| {
        TokenKind::Delimiter(Delimiter::from_char(c).expect("delimiter charset is exhaustive"))
    })
}

fn parse_number<'src>(
    prefix: &'static str,
    radix: u32,
    max_digits_count: usize,
    valid_digit_filter: impl Fn(&char) -> bool + 'static,
) -> impl Parser<'src, ParserInput<'src>, Literal, ParserError<'src>> {
    just(prefix).ignore_then(
        any()
            .filter(valid_digit_filter)
            .repeated()
            .at_least(1)
            .at_most(max_digits_count)
            .to_slice()
            .map(move |digits: &str| {
                i64::from_str_radix(digits, radix)
                    .map(|value| Literal::new_integer(value, radix))
                    .unwrap_or(Literal::new_integer(0, radix))
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::super::error::Reason;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::structure("structure", TokenKind::Keyword(Keyword::Structure))]
    #[case::enumerated("enumerated", TokenKind::Keyword(Keyword::Enumerated))]
    #[case::choice("choice", TokenKind::Keyword(Keyword::Choice))]
    #[case::protocol("protocol", TokenKind::Keyword(Keyword::Protocol))]
    #[case::type_("type", TokenKind::Keyword(Keyword::Type))]
    #[case::msb0("msb0", TokenKind::Keyword(Keyword::Msb0))]
    #[case::lsb0("lsb0", TokenKind::Keyword(Keyword::Lsb0))]
    #[case::on("on", TokenKind::Keyword(Keyword::On))]
    #[case::bit("bit", TokenKind::Keyword(Keyword::Bits))]
    #[case::bits("bits", TokenKind::Keyword(Keyword::Bits))]
    fn test_keyword(#[case] input: &str, #[case] expected: TokenKind) {
        let keyword_result = ident_or_keyword().parse(input).unwrap();
        assert_eq!(keyword_result, expected);

        let token_result = token().parse(input).unwrap();
        assert_eq!(token_result, expected);
    }

    #[test]
    fn test_keyword_is_not_a_prefix() {
        let token_result = token().parse("structures").unwrap();
        assert_eq!(
            token_result,
            TokenKind::Identifier("structures".to_string())
        );
    }

    #[rstest]
    #[case::boolean_true("true", TokenKind::Literal(Literal::Boolean(true)))]
    #[case::boolean_false("false", TokenKind::Literal(Literal::Boolean(false)))]
    #[case::null("null", TokenKind::Literal(Literal::Null))]
    #[case::decimal("123", TokenKind::Literal(Literal::new_integer(123, 10)))]
    #[case::decimal_padded("00042", TokenKind::Literal(Literal::new_integer(42, 10)))]
    #[case::hexadecimal("0x1A3F", TokenKind::Literal(Literal::new_integer(0x1A3F, 16)))]
    #[case::hexadecimal_lowercase("0xbeef", TokenKind::Literal(Literal::new_integer(0xBEEF, 16)))]
    #[case::hexadecimal_padded("0x0001", TokenKind::Literal(Literal::new_integer(0x1, 16)))]
    #[case::binary("0b101010", TokenKind::Literal(Literal::new_integer(0b101010, 2)))]
    #[case::binary_padded("0b0001", TokenKind::Literal(Literal::new_integer(0b1, 2)))]
    fn test_literal(#[case] input: &str, #[case] expected: TokenKind) {
        let token_result = token().parse(input).unwrap();
        assert_eq!(token_result, expected);
    }

    #[rstest]
    #[case::identifier("myVar", TokenKind::Identifier("myVar".to_string()))]
    #[case::identifier_with_underscore("my_var", TokenKind::Identifier("my_var".to_string()))]
    #[case::identifier_with_numbers("var123", TokenKind::Identifier("var123".to_string()))]
    fn test_identifier(#[case] input: &str, #[case] expected: TokenKind) {
        let identifier_result = ident_or_keyword().parse(input).unwrap();
        assert_eq!(identifier_result, expected);

        let token_result = token().parse(input).unwrap();
        assert_eq!(token_result, expected);
    }

    #[rstest]
    #[case::operator_dot(".", TokenKind::Operator(Operator::Dot))]
    #[case::operator_plus("+", TokenKind::Operator(Operator::Plus))]
    #[case::operator_minus("-", TokenKind::Operator(Operator::Minus))]
    #[case::operator_star("*", TokenKind::Operator(Operator::Star))]
    #[case::operator_slash("/", TokenKind::Operator(Operator::Slash))]
    #[case::operator_comma(",", TokenKind::Operator(Operator::Comma))]
    #[case::operator_semicolon(";", TokenKind::Operator(Operator::Semicolon))]
    #[case::operator_colon(":", TokenKind::Operator(Operator::Colon))]
    #[case::operator_equal("=", TokenKind::Operator(Operator::Equal))]
    #[case::operator_double_dot("..", TokenKind::Operator(Operator::DoubleDot))]
    #[case::operator_double_colon("::", TokenKind::Operator(Operator::DoubleColon))]
    #[case::operator_fat_arrow("=>", TokenKind::Operator(Operator::FatArrow))]
    fn test_operator(#[case] input: &str, #[case] expected: TokenKind) {
        let operator_result = operator().parse(input).unwrap();
        assert_eq!(operator_result, expected);

        let token_result = token().parse(input).unwrap();
        assert_eq!(token_result, expected);
    }

    #[rstest]
    #[case::delimiter_lparen("(", TokenKind::Delimiter(Delimiter::LParen))]
    #[case::delimiter_rparen(")", TokenKind::Delimiter(Delimiter::RParen))]
    #[case::delimiter_lbrace("{", TokenKind::Delimiter(Delimiter::LBrace))]
    #[case::delimiter_rbrace("}", TokenKind::Delimiter(Delimiter::RBrace))]
    #[case::delimiter_lbracket("[", TokenKind::Delimiter(Delimiter::LBracket))]
    #[case::delimiter_rbracket("]", TokenKind::Delimiter(Delimiter::RBracket))]
    #[case::delimiter_langle("<", TokenKind::Delimiter(Delimiter::LAngle))]
    #[case::delimiter_rangle(">", TokenKind::Delimiter(Delimiter::RAngle))]
    fn test_delimiter(#[case] input: &str, #[case] expected: TokenKind) {
        let delimiter_result = delimiter().parse(input).unwrap();
        assert_eq!(delimiter_result, expected);

        let token_result = token().parse(input).unwrap();
        assert_eq!(token_result, expected);
    }

    #[test]
    fn test_lex_source() {
        let source = "structure MyStruct { field1: bits, field2: 0x1A3F }";
        let tokens_result = lex_source(source).unwrap();

        let expected_tokens = vec![
            Token {
                kind: TokenKind::Keyword(Keyword::Structure),
                span: Span::new(0, 9),
            },
            Token {
                kind: TokenKind::Identifier("MyStruct".to_string()),
                span: Span::new(10, 18),
            },
            Token {
                kind: TokenKind::Delimiter(Delimiter::LBrace),
                span: Span::new(19, 20),
            },
            Token {
                kind: TokenKind::Identifier("field1".to_string()),
                span: Span::new(21, 27),
            },
            Token {
                kind: TokenKind::Operator(Operator::Colon),
                span: Span::new(27, 28),
            },
            Token {
                kind: TokenKind::Keyword(Keyword::Bits),
                span: Span::new(29, 33),
            },
            Token {
                kind: TokenKind::Operator(Operator::Comma),
                span: Span::new(33, 34),
            },
            Token {
                kind: TokenKind::Identifier("field2".to_string()),
                span: Span::new(35, 41),
            },
            Token {
                kind: TokenKind::Operator(Operator::Colon),
                span: Span::new(41, 42),
            },
            Token {
                kind: TokenKind::Literal(Literal::new_integer(0x1A3F, 16)),
                span: Span::new(43, 49),
            },
            Token {
                kind: TokenKind::Delimiter(Delimiter::RBrace),
                span: Span::new(50, 51),
            },
        ];

        assert_eq!(tokens_result.0, expected_tokens);
    }

    #[test]
    fn test_lex_source_skips_comments() {
        let source = "protocol Foo; // comment\n# another\nstructure(msb0) Bar {}";
        let tokens = lex_source(source).unwrap();
        let kinds: Vec<_> = tokens.0.into_iter().map(|token| token.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::Keyword(Keyword::Protocol),
                TokenKind::Identifier("Foo".to_string()),
                TokenKind::Operator(Operator::Semicolon),
                TokenKind::Keyword(Keyword::Structure),
                TokenKind::Delimiter(Delimiter::LParen),
                TokenKind::Keyword(Keyword::Msb0),
                TokenKind::Delimiter(Delimiter::RParen),
                TokenKind::Identifier("Bar".to_string()),
                TokenKind::Delimiter(Delimiter::LBrace),
                TokenKind::Delimiter(Delimiter::RBrace),
            ]
        );
    }

    #[test]
    fn test_lex_source_with_unexpected_token() {
        let source = "structure MyStruct { % }";
        let tokens_result = lex_source(source);

        assert!(tokens_result.is_err());
        let err = tokens_result.unwrap_err();
        assert_eq!(err.len(), 1);
        assert_eq!(
            err[0],
            Error {
                span: Some(Span::new(21, 22)),
                reason: Reason::Unexpected {
                    found: "%".to_string()
                }
            }
        );
    }
}
