use chumsky::prelude::*;

use crate::meklang2::SimpleErrorType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // identifiers
    Ident(String),
    String(String),

    // numbers
    DecimalInteger(u64),
    HexadecimalInteger(u64),
    BinaryInteger(u64),

    // symbols: operators, delimiters, etc.
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Equal,
    Comma,

    // keywords
    Bitstruct,
    Bit,
    Msb0,
    Lsb0,
    Bitenum,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub text: String,
    pub kind: CommentKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommentKind {
    Normal,
    Doc,
}

pub(crate) fn comment<'src>() -> impl Parser<'src, &'src str, Comment, SimpleErrorType<'src>> {
    let normal_comment = just("//")
        .ignore_then(
            any()
                .and_is(just('\n').not())
                .repeated()
                .collect::<String>(),
        )
        .map(|text| Comment {
            text: text.trim().to_string(),
            kind: CommentKind::Normal,
        });

    let doc_comment = just("///")
        .ignore_then(
            any()
                .and_is(just('\n').not())
                .repeated()
                .collect::<String>(),
        )
        .map(|text| Comment {
            text: text.trim().to_string(),
            kind: CommentKind::Doc,
        });

    choice((doc_comment, normal_comment))
}

pub(crate) fn lexer_tokens<'src>()
-> impl Parser<'src, &'src str, Vec<Spanned<Token>>, SimpleErrorType<'src>> {
    // --- identifiers & strings ---
    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "bitstruct" => Token::Bitstruct,
        "bit" => Token::Bit,
        "msb0" => Token::Msb0,
        "lsb0" => Token::Lsb0,
        "bitenum" => Token::Bitenum,
        _ => Token::Ident(ident.to_string()),
    });

    let str_ = just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(|s: &str| Token::String(s.to_string()));

    // --- numbers ---
    let decimal_integer = text::digits(10)
        .at_least(1)
        .collect::<String>()
        .map(|s| Token::DecimalInteger(s.parse::<u64>().unwrap()));

    let hexadecimal_integer = just("0x")
        .ignore_then(text::digits(16).at_least(1).collect::<String>())
        .map(|s| Token::HexadecimalInteger(u64::from_str_radix(&s, 16).unwrap()));

    let binary_integer = just("0b")
        .ignore_then(text::digits(2).at_least(1).collect::<String>())
        .map(|s| Token::BinaryInteger(u64::from_str_radix(&s, 2).unwrap()));

    let integers = choice((hexadecimal_integer, binary_integer, decimal_integer));

    // --- symbols ---
    let symbol = choice((
        just('{').to(Token::LeftBrace),
        just('}').to(Token::RightBrace),
        just('(').to(Token::LeftParen),
        just(')').to(Token::RightParen),
        just('[').to(Token::LeftBracket),
        just(']').to(Token::RightBracket),
        just('=').to(Token::Equal),
        just(',').to(Token::Comma),
    ));

    // --- main part ---
    let token = choice((integers, symbol, ident, str_)).spanned();

    token
        .padded()
        .padded_by(comment().repeated().ignored())
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
        .then_ignore(end())
}

pub(crate) fn lexer_comments<'src>()
-> impl Parser<'src, &'src str, Vec<Spanned<Comment>>, SimpleErrorType<'src>> {
    let comment_or_skip = comment().spanned().map(Some).or(any().ignored().to(None));

    comment_or_skip
        .repeated()
        .collect()
        .map(|v: Vec<Option<Spanned<Comment>>>| v.into_iter().flatten().collect())
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn check_item<T>(items: &[Spanned<T>], index: usize, expected: T)
    where
        T: std::fmt::Debug + Eq,
    {
        assert_eq!(items.get(index).unwrap().inner, expected);
    }

    fn check_items<T>(items: &[Spanned<T>], expected_items: Vec<T>)
    where
        T: std::fmt::Debug + Eq,
    {
        for (index, expected_item) in expected_items.into_iter().enumerate() {
            check_item::<T>(items, index, expected_item);
        }
    }

    #[rstest]
    #[case::ident("ident", Token::Ident("ident".to_string()))]
    #[case::keyword_bitstruct("bitstruct", Token::Bitstruct)]
    #[case::keyword_bit("bit", Token::Bit)]
    #[case::keyword_msb0("msb0", Token::Msb0)]
    #[case::keyword_lsb0("lsb0", Token::Lsb0)]
    #[case::keyword_bitenum("bitenum", Token::Bitenum)]
    #[case::string(r#""string in quotes""#, Token::String("string in quotes".to_string()))]
    #[case::string_with_newline("\"string in\nquotes\"", Token::String("string in\nquotes".to_string()))]
    #[case::decimal_integer("1234", Token::DecimalInteger(1234))]
    #[case::hexadecimal_integer("0x1234", Token::HexadecimalInteger(0x1234))]
    #[case::binary_integer("0b10101", Token::BinaryInteger(0b10101))]
    #[case::symbol_left_brace("{", Token::LeftBrace)]
    #[case::symbol_right_brace("}", Token::RightBrace)]
    #[case::symbol_left_paren("(", Token::LeftParen)]
    #[case::symbol_right_paren(")", Token::RightParen)]
    #[case::symbol_left_bracket("[", Token::LeftBracket)]
    #[case::symbol_right_bracket("]", Token::RightBracket)]
    #[case::symbol_equal("=", Token::Equal)]
    #[case::symbol_comma(",", Token::Comma)]
    fn test_lexer_tokens_single(#[case] input: &str, #[case] expected: Token) {
        let result = lexer_tokens().parse(input);
        let parsed_token = result
            .clone()
            .into_output()
            .unwrap()
            .first()
            .unwrap()
            .inner
            .clone();
        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_lexer_tokens_and_lexer_comments() {
        let input = r#"
            } 1234 { hehe bitstruct "StRinG"
            // normal comment
            /// doc comment
            {
        "#;

        let tokens_result = lexer_tokens().parse(input);
        let tokens = tokens_result.into_output().unwrap();
        check_items::<Token>(
            &tokens,
            vec![
                Token::RightBrace,
                Token::DecimalInteger(1234),
                Token::LeftBrace,
                Token::Ident("hehe".to_string()),
                Token::Bitstruct,
                Token::String("StRinG".to_string()),
                Token::LeftBrace,
            ],
        );

        let comments_result = lexer_comments().parse(input);
        let comments = comments_result.into_output().unwrap();
        check_items::<Comment>(
            &comments,
            vec![
                Comment {
                    text: "normal comment".to_string(),
                    kind: CommentKind::Normal,
                },
                Comment {
                    text: "doc comment".to_string(),
                    kind: CommentKind::Doc,
                },
            ],
        );
    }
}
