use std::ops::Range;

use chumsky::pratt::*;
use chumsky::prelude::*;

use crate::meklang2::lexer::{RichTokenErrorType, Token};

type TokenInputType<'src> =
    chumsky::input::MappedInput<'src, Token, SimpleSpan, &'src [Spanned<Token>]>;

// --- parsed AST nodes ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSyntax {
    name: String,
    version: Option<String>,
    description: Option<String>,
    documentation: Option<String>,
    definitions: Vec<DefinitionSyntax>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefinitionSyntax {
    Bitenum(Spanned<BitenumSyntax>),
    Bitstruct(Spanned<BitstructSyntax>),
}

pub type FieldReference = Vec<String>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Integer {
    value: u64,
    radix: Radix,
}

impl Integer {
    pub fn new(value: u64, radix: Radix) -> Self {
        Self { value, radix }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Radix {
    Decimal,
    Binary,
    Hexadecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitenumSyntax {
    name: Spanned<String>,
    size_in_bits: Spanned<Integer>,
    fields: Vec<Spanned<BitenumFieldSyntax>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitenumFieldSyntax {
    SingleValue { name: String, value: Integer },
    RangeOfValues { name: String, range: Range<Integer> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeSyntax {
    KeyOnly {
        key: String,
    },
    KeyValue {
        key: String,
        value: AttributeValueSyntax,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeValueSyntax {
    Integer(Integer),
    FieldReference(FieldReference),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitstructSyntax {
    name: Spanned<String>,
    bit_order: Spanned<BitstructBitOrder>,
    fields: Vec<Spanned<BitstructFieldSyntax>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitstructBitOrder {
    MostSignificantBitIsBit0,
    LeastSignificantBitIsBit0,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitstructFieldSyntax {
    Ordinary {
        name: Spanned<String>,
        typ: Spanned<String>,
        size: Spanned<BitstructFieldSizeSyntax>,
        attributes: Vec<Spanned<AttributeSyntax>>,
    },
    Union {
        name: Spanned<String>,
        size: Spanned<BitstructFieldSizeSyntax>,
        on: Spanned<FieldReference>,
        when: Option<Spanned<ExpressionSyntax>>,
        variants: Vec<Spanned<BitstructFieldUnionVariant>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitstructFieldSizeSyntax {
    Range(Range<Integer>),
    Increment(Integer),
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionSyntax {
    NumericLiteral(Integer),
    FieldReference(FieldReference),
    UnaryOperator {
        operator: UnaryOperatorSyntax,
        expr: Box<ExpressionSyntax>,
    },
    BinaryOperator {
        operator: BinaryOperatorSyntax,
        lhs: Box<ExpressionSyntax>,
        rhs: Box<ExpressionSyntax>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperatorSyntax {
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperatorSyntax {
    And,
    Or,

    Equals,
    NotEquals,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitstructFieldUnionVariant {
    pub discriminator: Spanned<Integer>,
    pub name: Spanned<String>,
    pub typ: Spanned<String>,
}

pub(crate) fn ident<'src>()
-> impl Parser<'src, TokenInputType<'src>, String, RichTokenErrorType<'src>> {
    select! { Token::Ident(name) => name.clone() }.labelled("identifier")
}

pub(crate) fn string<'src>()
-> impl Parser<'src, TokenInputType<'src>, String, RichTokenErrorType<'src>> {
    select! { Token::String(str_) => str_.clone() }.labelled("string")
}

pub(crate) fn field_reference<'src>()
-> impl Parser<'src, TokenInputType<'src>, FieldReference, RichTokenErrorType<'src>> {
    ident().separated_by(just(Token::Dot)).at_least(1).collect()
}

pub(crate) fn integer<'src>()
-> impl Parser<'src, TokenInputType<'src>, Integer, RichTokenErrorType<'src>> {
    select! {
        Token::DecimalInteger(value) => Integer::new(value, Radix::Decimal),
        Token::HexadecimalInteger(value) => Integer::new(value, Radix::Hexadecimal),
        Token::BinaryInteger(value) => Integer::new(value, Radix::Binary),
    }
}

pub(crate) fn integer_range<'src>()
-> impl Parser<'src, TokenInputType<'src>, Range<Integer>, RichTokenErrorType<'src>> {
    integer()
        .then_ignore(just(Token::DotDot))
        .then(integer())
        .map(|(start, end)| Range { start, end })
}

pub(crate) fn bitenum<'src>()
-> impl Parser<'src, TokenInputType<'src>, BitenumSyntax, RichTokenErrorType<'src>> {
    let bitenum_size = integer().spanned().labelled("bitenum size in bits");

    let bitenum_name = ident().spanned().labelled("bitenum name");

    let bitenum_field_single_value = ident()
        .then_ignore(just(Token::Equal))
        .then(integer())
        .map(|(name, value)| BitenumFieldSyntax::SingleValue { name, value });

    let bitenum_field_range_of_values = ident()
        .then_ignore(just(Token::Equal))
        .then(integer_range())
        .map(|(name, range)| BitenumFieldSyntax::RangeOfValues { name, range });

    let bitenum_fields = choice((bitenum_field_range_of_values, bitenum_field_single_value))
        .labelled("bitenum field (e.g. x = 1 or y = 5..8)")
        .spanned()
        .separated_by(just(Token::Comma))
        .at_least(1)
        .collect::<Vec<Spanned<BitenumFieldSyntax>>>();

    just(Token::KeywordBitenum)
        .ignore_then(bitenum_size.delimited_by(just(Token::LeftParen), just(Token::RightParen)))
        .then(bitenum_name)
        .then(bitenum_fields.delimited_by(just(Token::LeftBrace), just(Token::RightBrace)))
        .map(|((size_in_bits, name), fields)| BitenumSyntax {
            name,
            size_in_bits,
            fields,
        })
}

pub(crate) fn attributes<'src>()
-> impl Parser<'src, TokenInputType<'src>, Vec<Spanned<AttributeSyntax>>, RichTokenErrorType<'src>>
{
    let attribute_value = choice((
        integer().map(AttributeValueSyntax::Integer),
        field_reference().map(AttributeValueSyntax::FieldReference),
    ));

    let attribute = choice((
        ident()
            .then_ignore(just(Token::Equal))
            .then(attribute_value)
            .map(|(key, value)| AttributeSyntax::KeyValue { key, value }),
        ident().map(|key| AttributeSyntax::KeyOnly { key }),
    ))
    .spanned()
    .labelled("attribute");

    attribute
        .separated_by(just(Token::Comma))
        .at_least(1)
        .collect()
        .delimited_by(just(Token::LeftBracket), just(Token::RightBracket))
        .labelled("attributes list")
}

pub(crate) fn bitstruct_field_size<'src>()
-> impl Parser<'src, TokenInputType<'src>, BitstructFieldSizeSyntax, RichTokenErrorType<'src>> {
    choice((
        // derived
        just(Token::KeywordDerived).to(BitstructFieldSizeSyntax::Derived),
        // range, e.g. 0..15
        integer_range().map(BitstructFieldSizeSyntax::Range),
        // increment, e.g. +16
        just(Token::Plus)
            .ignore_then(integer())
            .map(BitstructFieldSizeSyntax::Increment),
    ))
}

pub(crate) fn bitstruct<'src>()
-> impl Parser<'src, TokenInputType<'src>, BitstructSyntax, RichTokenErrorType<'src>> {
    let bitstruct_name = ident().spanned().labelled("bitstruct name");

    let bitstruct_bit_order = choice((
        just(Token::KeywordMsb0).to(BitstructBitOrder::MostSignificantBitIsBit0),
        just(Token::KeywordLsb0).to(BitstructBitOrder::LeastSignificantBitIsBit0),
    ))
    .spanned()
    .labelled("bitstruct bit order (msb0 or lsb0)");

    let bitstruct_field_ordinary_size = bitstruct_field_size()
        .spanned()
        .labelled("bitstruct field size (derived, range or increment)");
    let bitstruct_field_ordinary_name = ident().spanned().labelled("bitstruct field name");
    let bitstruct_field_ordinary_type = ident().spanned().labelled("bitstruct field type");

    let bitstruct_field_ordinary = just(Token::KeywordBit)
        .ignore_then(bitstruct_field_ordinary_size)
        .then(bitstruct_field_ordinary_name)
        .then_ignore(just(Token::Colon))
        .then(bitstruct_field_ordinary_type)
        .then(attributes().or_not())
        .map(
            |(((size, name), typ), attrs)| BitstructFieldSyntax::Ordinary {
                name,
                typ,
                size,
                attributes: attrs.map_or(vec![], |attrs| attrs),
            },
        )
        .spanned();

    let bitstruct_field_union_variant_discriminator = integer()
        .spanned()
        .labelled("bitstruct union's variant discriminator");
    let bitstruct_field_union_variant_name =
        ident().spanned().labelled("bitstruct union's variant name");
    let bitstruct_field_union_variant_type =
        ident().spanned().labelled("bitstruct union's variant type");

    let bitstruct_field_union_variant = bitstruct_field_union_variant_discriminator
        .then_ignore(just(Token::MapsTo))
        .then(bitstruct_field_union_variant_name)
        .then_ignore(just(Token::Colon))
        .then(bitstruct_field_union_variant_type)
        .map(|((discriminator, name), typ)| BitstructFieldUnionVariant {
            discriminator,
            name,
            typ,
        });

    let bitstruct_field_union_size = bitstruct_field_size()
        .spanned()
        .labelled("bitstruct union field size (derived, range or increment)");
    let bitstruct_field_union_name = ident().spanned().labelled("bitstruct union field name");
    let bitstruct_field_union_on = field_reference()
        .spanned()
        .labelled("bitstruct union field's discriminator (on)");
    let bitstruct_field_union_when = expr()
        .spanned()
        .labelled("bitstruct union field expression");
    let bitstruct_field_union_variants = bitstruct_field_union_variant
        .spanned()
        .separated_by(just(Token::Comma))
        .at_least(1)
        .collect::<Vec<Spanned<BitstructFieldUnionVariant>>>()
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));

    let bitstruct_field_union = (just(Token::KeywordBit).ignore_then(bitstruct_field_union_size))
        .then(just(Token::KeywordUnion).ignore_then(bitstruct_field_union_name))
        .then(just(Token::KeywordOn).ignore_then(bitstruct_field_union_on))
        .then(
            just(Token::KeywordWhen)
                .ignore_then(bitstruct_field_union_when)
                .or_not(),
        )
        .then(bitstruct_field_union_variants)
        .map(
            |((((size, name), on), when), variants)| BitstructFieldSyntax::Union {
                name,
                size,
                on,
                when,
                variants,
            },
        )
        .spanned();

    let bitstruct_field = choice((bitstruct_field_ordinary, bitstruct_field_union));

    just(Token::KeywordBitstruct)
        .ignore_then(bitstruct_name)
        .then(bitstruct_bit_order.delimited_by(just(Token::LeftParen), just(Token::RightParen)))
        .then(
            bitstruct_field
                .separated_by(just(Token::Comma))
                .at_least(1)
                .collect()
                .delimited_by(just(Token::LeftBrace), just(Token::RightBrace)),
        )
        .map(|((name, bit_order), fields)| BitstructSyntax {
            name,
            bit_order,
            fields,
        })
}

pub(crate) fn expr<'src>()
-> impl Parser<'src, TokenInputType<'src>, ExpressionSyntax, RichTokenErrorType<'src>> {
    recursive(|expr| {
        let atom = choice((
            integer().map(ExpressionSyntax::NumericLiteral).boxed(),
            field_reference()
                .map(ExpressionSyntax::FieldReference)
                .boxed(),
            expr.delimited_by(just(Token::LeftParen), just(Token::RightParen))
                .boxed(),
        ));

        atom.pratt((
            // comparison operators have the highest precedence,
            // chaining is forbidden thus Associativity::None is used
            infix(
                Associativity::None(4),
                just(Token::EqualsTo),
                |lhs, _, rhs, _| ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::Equals,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                just(Token::NotEqual),
                |lhs, _, rhs, _| ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::NotEquals,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                just(Token::LessThan),
                |lhs, _, rhs, _| ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::LessThan,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                just(Token::LessEqual),
                |lhs, _, rhs, _| ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::LessEqual,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                just(Token::GreaterThan),
                |lhs, _, rhs, _| ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::GreaterThan,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                just(Token::GreaterEqual),
                |lhs, _, rhs, _| ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::GreaterEqual,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            // negation has smaller precedence, it's prefix operator with right associativity
            prefix(3, just(Token::Not), |_, expr, _| {
                ExpressionSyntax::UnaryOperator {
                    operator: UnaryOperatorSyntax::Not,
                    expr: Box::new(expr),
                }
            }),
            // and should be done after comparisons/negations, it's infix operator with left associativity
            infix(left(2), just(Token::And), |lhs, _, rhs, _| {
                ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::And,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }),
            // or comes at the end, similar to and
            infix(Associativity::Left(1), just(Token::Or), |lhs, _, rhs, _| {
                ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::Or,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }),
        ))
    })
}

pub(crate) fn definition<'src>()
-> impl Parser<'src, TokenInputType<'src>, DefinitionSyntax, RichTokenErrorType<'src>> {
    choice((
        bitenum().spanned().map(DefinitionSyntax::Bitenum),
        bitstruct().spanned().map(DefinitionSyntax::Bitstruct),
    ))
}

pub(crate) fn module<'src>()
-> impl Parser<'src, TokenInputType<'src>, ModuleSyntax, RichTokenErrorType<'src>> {
    // module metadata
    let module = just(Token::KeywordModule)
        .ignore_then(just(Token::Colon))
        .ignore_then(string().labelled("module name"));
    let version = just(Token::KeywordVersion)
        .ignore_then(just(Token::Colon))
        .ignore_then(string().labelled("module version"))
        .or_not();
    let description = just(Token::KeywordDescription)
        .ignore_then(just(Token::Colon))
        .ignore_then(string().labelled("module description"))
        .or_not();
    let documentation = just(Token::KeywordDocumentation)
        .ignore_then(just(Token::Colon))
        .ignore_then(string().labelled("module documentation link"))
        .or_not();

    let metadata_fields = module.then(version).then(description).then(documentation);
    let metadata = just(Token::KeywordMetadata)
        .ignore_then(metadata_fields.delimited_by(just(Token::LeftBrace), just(Token::RightBrace)));

    metadata
        .then(definition().repeated().at_least(1).collect())
        .map(
            |((((name, version), description), documentation), definitions)| ModuleSyntax {
                name,
                version,
                description,
                documentation,
                definitions,
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::meklang2::lexer::lexer_tokens;

    fn lexed_input<'src>(input: &'src str) -> TokenInputType<'src> {
        let (tokens, errs) = lexer_tokens().parse(input).into_output_errors();
        assert!(tokens.is_some());

        if !errs.is_empty() {
            panic!("Found errors: {:#?}", errs);
        }

        let leaked: &'src [Spanned<Token>] = Box::leak(tokens.unwrap().into_boxed_slice());
        leaked.map((0..input.len()).into(), |t| (&t.inner, &t.span))
    }

    fn check_spanned_inner<T>(item: Spanned<T>, expected: T)
    where
        T: std::fmt::Debug + Eq,
    {
        assert_eq!(item.inner, expected);
    }

    fn check_spanned_inner_items<T>(items: &[Spanned<T>], expected_items: Vec<T>)
    where
        T: std::fmt::Debug + Eq,
    {
        assert_eq!(items.len(), expected_items.len());

        for (index, expected_item) in expected_items.into_iter().enumerate() {
            assert_eq!(items.get(index).unwrap().inner, expected_item);
        }
    }

    fn check_spanned_span<T>(item: &Spanned<T>, span_range: Range<usize>) {
        assert_eq!(item.span.start, span_range.start);
        assert_eq!(item.span.end, span_range.end);
    }

    fn check_spanned_span_items<T>(items: &[Spanned<T>], spans: Vec<Range<usize>>) {
        assert_eq!(items.len(), spans.len());

        for item in items {
            for (index, expected_item) in spans.clone().into_iter().enumerate() {
                let extracted = items.get(index).unwrap();
                check_spanned_span(extracted, expected_item);
            }
        }
    }

    #[test]
    fn test_ident_parser() {
        let input = "xxx";
        let result = ident().parse(lexed_input(input));
        assert_eq!(result.unwrap(), "xxx".to_string());
    }

    #[test]
    fn test_string_parser() {
        let input = "\"xxx\"";
        let result = string().parse(lexed_input(input));
        assert_eq!(result.unwrap(), "xxx".to_string());
    }

    #[rstest]
    #[case::decimal("19", Radix::Decimal)]
    #[case::hexadecimal("0x13", Radix::Hexadecimal)]
    #[case::binary("0b10011", Radix::Binary)]
    fn test_integer_parser(#[case] input: &str, #[case] expected_radix: Radix) {
        let result = integer().parse(lexed_input(input)).unwrap();
        assert_eq!(result.value, 19);
        assert_eq!(result.radix, expected_radix);
    }

    #[rstest]
    #[case::decimal("1..19")]
    #[case::hexadecimal("0x01..0x13")]
    #[case::binary("0b00001..0b10011")]
    #[case::mixed("0b01..19")]
    fn test_integer_range_parser(#[case] input: &str) {
        let result = integer_range().parse(lexed_input(input)).unwrap();
        assert_eq!(result.start.value, 1);
        assert_eq!(result.end.value, 19);
    }

    #[rstest]
    #[case::single("xyz", vec!["xyz"])]
    #[case::double("xyz.abc", vec!["xyz", "abc"])]
    #[case::triple("xyz.abc.def", vec!["xyz", "abc", "def"])]
    #[case::quarduple("xyz.abc.def.ghi", vec!["xyz", "abc", "def", "ghi"])]
    fn test_field_reference(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = field_reference().parse(lexed_input(input)).unwrap();
        let expected: FieldReference = expected.into_iter().map(String::from).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitenum_parser() {
        let input = "bitenum(6) test { single = 1, range = 2..3, another = 4 }";
        let result = bitenum().parse(lexed_input(input)).unwrap();

        check_spanned_inner(result.name, "test".to_string());
        check_spanned_inner(result.size_in_bits, Integer::new(6, Radix::Decimal));
        check_spanned_inner_items(
            &result.fields,
            vec![
                BitenumFieldSyntax::SingleValue {
                    name: "single".into(),
                    value: Integer::new(1, Radix::Decimal),
                },
                BitenumFieldSyntax::RangeOfValues {
                    name: "range".into(),
                    range: (Integer::new(2, Radix::Decimal)..Integer::new(3, Radix::Decimal)),
                },
                BitenumFieldSyntax::SingleValue {
                    name: "another".into(),
                    value: Integer::new(4, Radix::Decimal),
                },
            ],
        );
    }

    #[test]
    fn test_attributes_parser() {
        let input = "[just_key, with_int = 15, with_field_ref = x.y.z]";
        let result = attributes().parse(lexed_input(input)).unwrap();

        check_spanned_inner_items(
            &result,
            vec![
                AttributeSyntax::KeyOnly {
                    key: "just_key".into(),
                },
                AttributeSyntax::KeyValue {
                    key: "with_int".into(),
                    value: AttributeValueSyntax::Integer(Integer::new(15, Radix::Decimal)),
                },
                AttributeSyntax::KeyValue {
                    key: "with_field_ref".into(),
                    value: AttributeValueSyntax::FieldReference(vec![
                        "x".into(),
                        "y".into(),
                        "z".into(),
                    ]),
                },
            ],
        );

        check_spanned_span_items(&result, vec![1..9, 11..24, 26..48]);
    }

    fn check_bitstruct_field_ordinary(
        field: &BitstructFieldSyntax,
        expected_name: &str,
        expected_typ: &str,
        expected_size: BitstructFieldSizeSyntax,
        expected_attrs: Vec<AttributeSyntax>,
    ) {
        if let BitstructFieldSyntax::Ordinary {
            name,
            typ,
            size,
            attributes,
        } = field
        {
            assert_eq!(name.inner, expected_name.to_string());
            assert_eq!(typ.inner, expected_typ.to_string());
            assert_eq!(size.inner, expected_size);
            check_spanned_inner_items(attributes, expected_attrs);
        } else {
            panic!("Expected BitstructFieldSyntax::Ordinary");
        }
    }

    fn check_bitstruct_field_union(
        field: &BitstructFieldSyntax,
        expected_name: &str,
        expected_size: BitstructFieldSizeSyntax,
        expected_on: FieldReference,
        expected_when_expr: Option<ExpressionSyntax>,
        expected_variants: Vec<(Integer, String, String)>,
    ) {
        if let BitstructFieldSyntax::Union {
            name,
            size,
            on,
            when,
            variants,
        } = field
        {
            assert_eq!(name.inner, expected_name.to_string());
            assert_eq!(size.inner, expected_size);
            assert_eq!(on.inner, expected_on);
            match when {
                Some(expr) => assert_eq!(expr.inner, expected_when_expr.unwrap()),
                None => assert!(expected_when_expr.is_none()),
            };

            assert_eq!(variants.len(), expected_variants.len());
            for (index, expected_variant) in expected_variants.into_iter().enumerate() {
                let variant = &variants.get(index).unwrap().inner;
                let expected_discriminator = expected_variant.0;
                let expected_name = expected_variant.1;
                let expected_typ = expected_variant.2;
                assert_eq!(variant.discriminator.inner, expected_discriminator);
                assert_eq!(variant.name.inner, expected_name);
                assert_eq!(variant.typ.inner, expected_typ);
            }
        } else {
            panic!("Expected BitstructFieldSyntax::Union");
        }
    }

    #[test]
    fn test_bitstruct_parser() {
        let input = r#"
            bitstruct Xyz(msb0) {
                bit 0..7 n1: t1,
                bit 8..15 n2: t2,
                bit +4 _n3: t3,
                bit derived union n4 on _n3 when n2 and n1 > 10 {
                    0x01 => v1: t4,
                    0x02 => v2: t5
                },
                bit derived n5: t4 [attr],
                bit derived union n6 on n5 {
                    0x01 => v1: t1
                }
            }
        "#;
        let result = bitstruct().parse(lexed_input(input)).unwrap();

        check_spanned_inner(result.name, "Xyz".to_string());
        check_spanned_inner(
            result.bit_order,
            BitstructBitOrder::MostSignificantBitIsBit0,
        );

        check_bitstruct_field_ordinary(
            &result.fields[0].inner,
            "n1",
            "t1",
            BitstructFieldSizeSyntax::Range(
                Integer::new(0, Radix::Decimal)..Integer::new(7, Radix::Decimal),
            ),
            vec![],
        );

        check_bitstruct_field_ordinary(
            &result.fields[1].inner,
            "n2",
            "t2",
            BitstructFieldSizeSyntax::Range(
                Integer::new(8, Radix::Decimal)..Integer::new(15, Radix::Decimal),
            ),
            vec![],
        );

        check_bitstruct_field_ordinary(
            &result.fields[2].inner,
            "_n3",
            "t3",
            BitstructFieldSizeSyntax::Increment(Integer::new(4, Radix::Decimal)),
            vec![],
        );

        check_bitstruct_field_union(
            &result.fields[3].inner,
            "n4",
            BitstructFieldSizeSyntax::Derived,
            vec!["_n3".to_string()],
            Some(ExpressionSyntax::BinaryOperator {
                operator: BinaryOperatorSyntax::And,
                lhs: Box::new(ExpressionSyntax::FieldReference(vec!["n2".to_string()])),
                rhs: Box::new(ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::GreaterThan,
                    lhs: Box::new(ExpressionSyntax::FieldReference(vec!["n1".to_string()])),
                    rhs: Box::new(ExpressionSyntax::NumericLiteral(Integer::new(
                        10,
                        Radix::Decimal,
                    ))),
                }),
            }),
            vec![
                (
                    Integer::new(0x01, Radix::Hexadecimal),
                    "v1".into(),
                    "t4".into(),
                ),
                (
                    Integer::new(0x02, Radix::Hexadecimal),
                    "v2".into(),
                    "t5".into(),
                ),
            ],
        );

        check_bitstruct_field_ordinary(
            &result.fields[4].inner,
            "n5",
            "t4",
            BitstructFieldSizeSyntax::Derived,
            vec![AttributeSyntax::KeyOnly { key: "attr".into() }],
        );

        check_bitstruct_field_union(
            &result.fields[5].inner,
            "n6",
            BitstructFieldSizeSyntax::Derived,
            vec!["n5".to_string()],
            None,
            vec![(
                Integer::new(0x01, Radix::Hexadecimal),
                "v1".into(),
                "t1".into(),
            )],
        );
    }

    #[rstest]
    #[case::equals(
        "x.z == 0x1337",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::Equals,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["x".to_string(), "z".to_string()])),
            rhs: Box::new(ExpressionSyntax::NumericLiteral(Integer::new(0x1337, Radix::Hexadecimal)))
        }
    )]
    #[case::not_equals(
        "y != 0b1010",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::NotEquals,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["y".to_string()])),
            rhs: Box::new(ExpressionSyntax::NumericLiteral(Integer::new(0b1010, Radix::Binary)))
        }
    )]
    #[case::less_than(
        "a < b",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::LessThan,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
            rhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
        }
    )]
    #[case::less_equal(
        "a <= b",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::LessEqual,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
            rhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
        }
    )]
    #[case::greater_than(
        "a > b",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::GreaterThan,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
            rhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
        }
    )]
    #[case::greater_equal(
        "a >= b",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::GreaterEqual,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
            rhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
        }
    )]
    #[case::not(
        "not a",
        ExpressionSyntax::UnaryOperator {
            operator: UnaryOperatorSyntax::Not,
            expr:  Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
        }
    )]
    #[case::greater_equal(
        "a and b",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::And,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
            rhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
        }
    )]
    #[case::greater_equal(
        "a or b",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::Or,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
            rhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
        }
    )]
    #[case::parentheses_on_the_left(
        "(a and b) and c",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::And,
            lhs: Box::new(
                ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::And,
                    lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
                    rhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
                }
            ),
            rhs: Box::new(ExpressionSyntax::FieldReference(vec!["c".to_string()])),
        }
    )]
    #[case::parentheses_on_the_right(
        "a and (b and c)",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::And,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
            rhs: Box::new(
                ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::And,
                    lhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
                    rhs: Box::new(ExpressionSyntax::FieldReference(vec!["c".to_string()])),
                }
            ),
        }
    )]
    #[case::parentheses_nested_on_the_left(
        "(((a and b) and c) or d)",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::Or,
            lhs: Box::new(
                ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::And,
                    lhs: Box::new(ExpressionSyntax::BinaryOperator {
                        operator: BinaryOperatorSyntax::And,
                        lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
                        rhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
                    }),
                    rhs: Box::new(ExpressionSyntax::FieldReference(vec!["c".to_string()])),
                }
            ),
            rhs: Box::new(ExpressionSyntax::FieldReference(vec!["d".to_string()])),
        }
    )]
    #[case::parentheses_nested_on_the_right(
        "(a and (b and (c or d)))",
        ExpressionSyntax::BinaryOperator {
            operator: BinaryOperatorSyntax::And,
            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
            rhs: Box::new(
                ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::And,
                    lhs: Box::new(ExpressionSyntax::FieldReference(vec!["b".to_string()])),
                    rhs: Box::new(ExpressionSyntax::BinaryOperator {
                        operator: BinaryOperatorSyntax::Or,
                        lhs: Box::new(ExpressionSyntax::FieldReference(vec!["c".to_string()])),
                        rhs: Box::new(ExpressionSyntax::FieldReference(vec!["d".to_string()])),
                    }),
                }
            ),
        }
    )]
    #[case::multiple_comparisons(
        "not (a > 10 and a <= 20 or a == 30)",
        ExpressionSyntax::UnaryOperator {
            operator: UnaryOperatorSyntax::Not,
            expr: Box::new(
                ExpressionSyntax::BinaryOperator {
                    operator: BinaryOperatorSyntax::Or,
                    lhs: Box::new(
                        ExpressionSyntax::BinaryOperator {
                            operator: BinaryOperatorSyntax::And,
                            lhs: Box::new(
                                ExpressionSyntax::BinaryOperator {
                                    operator: BinaryOperatorSyntax::GreaterThan,
                                    lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
                                    rhs: Box::new(ExpressionSyntax::NumericLiteral(Integer::new(10, Radix::Decimal)))
                                }
                            ),
                            rhs: Box::new(
                                ExpressionSyntax::BinaryOperator {
                                    operator: BinaryOperatorSyntax::LessEqual,
                                    lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
                                    rhs: Box::new(ExpressionSyntax::NumericLiteral(Integer::new(20, Radix::Decimal)))
                                }
                            ),
                        }
                    ),
                    rhs: Box::new(
                        ExpressionSyntax::BinaryOperator {
                            operator: BinaryOperatorSyntax::Equals,
                            lhs: Box::new(ExpressionSyntax::FieldReference(vec!["a".to_string()])),
                            rhs: Box::new(ExpressionSyntax::NumericLiteral(Integer::new(30, Radix::Decimal)))
                        }
                    ),
                }
            ),
        }
    )]
    fn test_expr_parser(#[case] input: &str, #[case] expected: ExpressionSyntax) {
        let result = expr().parse(lexed_input(input)).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_definition_and_module_parsers() {
        let input = r#"
            metadata {
                module: "light_proto"
                version: "1.2.3-rev3"
            }

            bitenum(2) my_enum {
                x = 0,
                y = 1
            }

            bitstruct aaa(lsb0) {
                bit derived x: my_enum
            }
        "#;
        let result = module().parse(lexed_input(input)).unwrap();
        println!("{:#?}", result);

        assert_eq!(result.name, "light_proto".to_string());
        assert_eq!(result.version, Some("1.2.3-rev3".to_string()));
        assert_eq!(result.description, None);
        assert_eq!(result.documentation, None);

        assert!(matches!(
            result.definitions[0],
            DefinitionSyntax::Bitenum { .. }
        ));

        assert!(matches!(
            result.definitions[1],
            DefinitionSyntax::Bitstruct { .. }
        ));
    }
}
