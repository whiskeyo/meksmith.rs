use chumsky::input::MappedInput;
use chumsky::prelude::*;

pub(crate) type SimpleError<'src> = chumsky::error::Simple<'src, char>;
pub(crate) type SimpleErrorType<'src> = chumsky::extra::Err<SimpleError<'src>>;
pub(crate) type RichTokenError<'src> = chumsky::error::Rich<'src, Token>;
pub(crate) type RichTokenErrorType<'src> = chumsky::extra::Err<RichTokenError<'src>>;

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
    Dot,
    DotDot,
    Colon,
    MapsTo,
    Plus,
    EqualsTo,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    NotEqual,
    Not,
    And,
    Or,

    // keywords
    KeywordBitstruct,
    KeywordMsb0,
    KeywordLsb0,
    KeywordUnion,
    KeywordOn,
    KeywordWhen,
    KeywordBit,
    KeywordDerived,
    KeywordBitenum,
    KeywordMetadata,
    KeywordModule,
    KeywordVersion,
    KeywordDescription,
    KeywordDocumentation,
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
    let ident = text::ident()
        .map(|ident: &str| match ident {
            "bitstruct" => Token::KeywordBitstruct,
            "msb0" => Token::KeywordMsb0,
            "lsb0" => Token::KeywordLsb0,
            "union" => Token::KeywordUnion,
            "on" => Token::KeywordOn,
            "when" => Token::KeywordWhen,
            "bit" => Token::KeywordBit,
            "derived" => Token::KeywordDerived,
            "bitenum" => Token::KeywordBitenum,
            "metadata" => Token::KeywordMetadata,
            "module" => Token::KeywordModule,
            "version" => Token::KeywordVersion,
            "description" => Token::KeywordDescription,
            "documentation" => Token::KeywordDocumentation,
            _ => Token::Ident(ident.to_string()),
        })
        .padded();

    let str_ = just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(|s: &str| Token::String(s.to_string()));

    // --- numbers ---
    // that ugly `.then(ident.not())` is required to avoid parsing numbers followed by
    // identifiers, i.e. 0x1234aaa => (0x1234, aaa) is wrong.
    let decimal_integer = text::digits(10)
        .at_least(1)
        .collect::<String>()
        .then_ignore(
            // Negative lookahead: fail if the next char is a letter or underscore
            any()
                .filter(|c: &char| c.is_ascii_alphabetic() || *c == '_')
                .not(),
        )
        // .then(ident.not())
        // .map(|(s, _)| Token::DecimalInteger(s.parse::<u64>().unwrap()));
        .map(|s| Token::DecimalInteger(s.parse::<u64>().unwrap()));

    let hexadecimal_integer = just("0x")
        .ignore_then(text::digits(16).at_least(1).collect::<String>())
        .then_ignore(
            // Negative lookahead: fail if the next char is a letter or underscore
            any()
                .filter(|c: &char| c.is_ascii_alphabetic() || *c == '_')
                .not(),
        )
        // .then(ident.not())
        // .map(|(s, _)| Token::HexadecimalInteger(u64::from_str_radix(&s, 16).unwrap()));
        .map(|s| Token::HexadecimalInteger(u64::from_str_radix(&s, 16).unwrap()));

    let binary_integer = just("0b")
        .ignore_then(text::digits(2).at_least(1).collect::<String>())
        .then_ignore(
            // Negative lookahead: fail if the next char is a letter or underscore
            any()
                .filter(|c: &char| c.is_ascii_alphabetic() || *c == '_')
                .not(),
        )
        // .then(ident.not())
        // .map(|(s, _)| Token::BinaryInteger(u64::from_str_radix(&s, 2).unwrap()));
        .map(|s| Token::BinaryInteger(u64::from_str_radix(&s, 2).unwrap()));

    let integer = choice((hexadecimal_integer, binary_integer, decimal_integer));

    // --- symbols ---
    let symbol = choice((
        just("..").to(Token::DotDot),
        just("=>").to(Token::MapsTo),
        just("==").to(Token::EqualsTo),
        just("<=").to(Token::LessEqual),
        just(">=").to(Token::GreaterEqual),
        just("!=").to(Token::NotEqual),
        just("not").or(just("!")).to(Token::Not),
        just("and").or(just("&&")).to(Token::And),
        just("or").or(just("||")).to(Token::Or),
        just('{').to(Token::LeftBrace),
        just('}').to(Token::RightBrace),
        just('(').to(Token::LeftParen),
        just(')').to(Token::RightParen),
        just('[').to(Token::LeftBracket),
        just(']').to(Token::RightBracket),
        just('=').to(Token::Equal),
        just(',').to(Token::Comma),
        just('.').to(Token::Dot),
        just(':').to(Token::Colon),
        just("+").to(Token::Plus),
        just("<").to(Token::LessThan),
        just(">").to(Token::GreaterThan),
    ));

    // --- main part ---
    let token = choice((integer, symbol, ident, str_));

    token
        .spanned()
        .padded()
        .padded_by(comment().repeated().ignored())
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

pub(crate) fn lexer_comments<'src>()
-> impl Parser<'src, &'src str, Vec<Spanned<Comment>>, SimpleErrorType<'src>> {
    let comment_or_skip = comment().spanned().map(Some).or(any().ignored().to(None));

    comment_or_skip
        .repeated()
        .collect()
        .map(|v: Vec<Option<Spanned<Comment>>>| v.into_iter().flatten().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::container::Seq;
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
    #[case::ident("ident", Token::Ident("ident".into()))]
    #[case::keyword_bitstruct("bitstruct", Token::KeywordBitstruct)]
    #[case::keyword_msb0("msb0", Token::KeywordMsb0)]
    #[case::keyword_lsb0("lsb0", Token::KeywordLsb0)]
    #[case::keyword_union("union", Token::KeywordUnion)]
    #[case::keyword_on("on", Token::KeywordOn)]
    #[case::keyword_when("when", Token::KeywordWhen)]
    #[case::keyword_bit("bit", Token::KeywordBit)]
    #[case::keyword_derived("derived", Token::KeywordDerived)]
    #[case::keyword_bitenum("bitenum", Token::KeywordBitenum)]
    #[case::keyword_metadata("metadata", Token::KeywordMetadata)]
    #[case::keyword_module("module", Token::KeywordModule)]
    #[case::keyword_version("version", Token::KeywordVersion)]
    #[case::keyword_description("description", Token::KeywordDescription)]
    #[case::keyword_documentation("documentation", Token::KeywordDocumentation)]
    #[case::string(r#""string in quotes""#, Token::String("string in quotes".into()))]
    #[case::string_with_newline("\"string in\nquotes\"", Token::String("string in\nquotes".into()))]
    #[case::decimal_integer("1234", Token::DecimalInteger(1234))]
    #[case::decimal_integer_zero("0", Token::DecimalInteger(0))]
    #[case::decimal_integer_multiple_zeros("0000", Token::DecimalInteger(0))]
    #[case::hexadecimal_integer("0x1234", Token::HexadecimalInteger(0x1234))]
    #[case::hexadecimal_integer_zero("0x0", Token::HexadecimalInteger(0x0))]
    #[case::hexadecimal_integer_with_filling_zeros("0x0001234", Token::HexadecimalInteger(0x1234))]
    #[case::binary_integer("0b10101", Token::BinaryInteger(0b10101))]
    #[case::binary_integer_zero("0b0", Token::BinaryInteger(0b0))]
    #[case::binary_integer_with_filling_zeros("0b000010101", Token::BinaryInteger(0b10101))]
    #[case::symbol_left_brace("{", Token::LeftBrace)]
    #[case::symbol_right_brace("}", Token::RightBrace)]
    #[case::symbol_left_paren("(", Token::LeftParen)]
    #[case::symbol_right_paren(")", Token::RightParen)]
    #[case::symbol_left_bracket("[", Token::LeftBracket)]
    #[case::symbol_right_bracket("]", Token::RightBracket)]
    #[case::symbol_equal("=", Token::Equal)]
    #[case::symbol_comma(",", Token::Comma)]
    #[case::symbol_dot(".", Token::Dot)]
    #[case::symbol_double_dot("..", Token::DotDot)]
    #[case::symbol_colon(":", Token::Colon)]
    #[case::symbol_maps_to("=>", Token::MapsTo)]
    #[case::symbol_plus("+", Token::Plus)]
    #[case::symbol_equals_to("==", Token::EqualsTo)]
    #[case::symbol_less_than("<", Token::LessThan)]
    #[case::symbol_less_equal("<=", Token::LessEqual)]
    #[case::symbol_greater_than(">", Token::GreaterThan)]
    #[case::symbol_greater_equal(">=", Token::GreaterEqual)]
    #[case::symbol_not_equal("!=", Token::NotEqual)]
    #[case::symbol_not("not", Token::Not)]
    #[case::symbol_not_sign("!", Token::Not)]
    #[case::symbol_and("and", Token::And)]
    #[case::symbol_and_sign("&&", Token::And)]
    #[case::symbol_or("or", Token::Or)]
    #[case::symbol_or_sign("||", Token::Or)]
    fn test_lexer_tokens_single(#[case] input: &str, #[case] expected: Token) {
        let result = lexer_tokens().parse(input);
        let parsed_token = result.unwrap().first().unwrap().inner.clone();
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
                Token::Ident("hehe".into()),
                Token::KeywordBitstruct,
                Token::String("StRinG".into()),
                Token::LeftBrace,
            ],
        );

        let comments_result = lexer_comments().parse(input);
        let comments = comments_result.into_output().unwrap();
        check_items::<Comment>(
            &comments,
            vec![
                Comment {
                    text: "normal comment".into(),
                    kind: CommentKind::Normal,
                },
                Comment {
                    text: "doc comment".into(),
                    kind: CommentKind::Doc,
                },
            ],
        );
    }

    #[test]
    fn test_lexer_tokens_keywords_and_range() {
        let input = "bit 0..8 x: y";

        let tokens_result = lexer_tokens().parse(input);
        let tokens = tokens_result.into_output().unwrap();
        check_items::<Token>(
            &tokens,
            vec![
                Token::KeywordBit,
                Token::DecimalInteger(0),
                Token::DotDot,
                Token::DecimalInteger(8),
                Token::Ident("x".into()),
                Token::Colon,
                Token::Ident("y".into()),
            ],
        );
    }

    #[test]
    fn test_lexer_tokens_error_recovery() {
        let input = r#"0x12 0x12x 1234 1234yy 0b1010 0b1010zzz"#;
        let (tokens, errors) = lexer_tokens().parse(input).into_output_errors();
        assert!(tokens.is_some());
        assert!(!errors.is_empty());

        let valid_tokens = vec![
            (Token::HexadecimalInteger(0x12), 0..4),
            (Token::DecimalInteger(1234), 11..15),
            (Token::BinaryInteger(0b1010), 23..29),
        ];

        for valid_token in valid_tokens {
            let expected_spanned = Spanned {
                inner: valid_token.0,
                span: SimpleSpan::from(valid_token.1),
            };
            assert!(tokens.as_ref().unwrap().contains(&expected_spanned));
        }

        let expected_error_ranges = [6..11, 20..23, 31..39];
        for error in errors {
            let error_range = error.span().start..error.span().end;

            let is_expected = expected_error_ranges.iter().any(|expected| {
                expected.start <= error_range.start && error_range.end <= expected.end
            });

            assert!(is_expected, "unexpected error range: {:?}", error_range);
        }
    }
}
