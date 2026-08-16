use chumsky::error::Rich;
use chumsky::extra;
use chumsky::input::{Input, ValueInput};
use chumsky::prelude::*;

use super::{
    ast::{
        Attribute, BinOp, BitOrder, Choice, ChoiceArm, ChoicePattern, EnumValue, EnumVariant,
        Enumerated, Expr, Field, File, Item, Param, Structure, TypeAlias, TypeExpr,
    },
    error::Error,
    lexer::lex_source,
    span::{SimpleSpan, Spanned},
    token::{Delimiter, Keyword, Literal, Operator, Token, TokenKind},
};
use crate::diagnostic::{Diagnostic, Diagnostics};

pub mod common;

use common::{delimiter, ident, integer_literal, keyword, literal, operator};

pub(crate) type ParserError<'src> = extra::Err<Rich<'src, TokenKind, SimpleSpan>>;

pub fn parse(source: &str) -> Result<File, Diagnostics> {
    let tokens = lex_source(source).map_err(diagnostics_from_lex_errors)?;
    let eoi = SimpleSpan::new(source.len(), source.len());
    let input = tokens
        .0
        .as_slice()
        .map(eoi, |token: &Token| (&token.kind, &token.span));

    file()
        .parse(input)
        .into_result()
        .map_err(|errors| diagnostics_from_parse_errors(&errors))
}

pub fn parse_source(source: &str) -> Result<File, Vec<Error>> {
    parse(source).map_err(|diagnostics| {
        diagnostics
            .items
            .into_iter()
            .map(Error::from_diagnostic)
            .collect()
    })
}

fn diagnostics_from_lex_errors(errors: Vec<Error>) -> Diagnostics {
    errors.into_iter().map(Diagnostic::from).collect()
}

fn diagnostics_from_parse_errors(errors: &[Rich<'_, TokenKind, SimpleSpan>]) -> Diagnostics {
    errors.iter().map(Diagnostic::from_rich).collect()
}

fn file<'src, I>() -> impl Parser<'src, I, File, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    protocol()
        .or_not()
        .then(item().repeated().collect::<Vec<_>>())
        .then_ignore(end())
        .map_with(|(protocol, items), e| File {
            span: e.span(),
            protocol,
            items,
        })
}

fn protocol<'src, I>() -> impl Parser<'src, I, Spanned<String>, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    keyword(Keyword::Protocol)
        .ignore_then(ident())
        .then_ignore(operator(Operator::Semicolon))
        .map_with(|name, e| Spanned::new(name, e.span()))
        .labelled("protocol declaration")
}

fn item<'src, I>() -> impl Parser<'src, I, Spanned<Item>, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    choice((
        structure().map(Item::Structure),
        enumerated().map(Item::Enumerated),
        choice_def().map(Item::Choice),
        type_alias().map(Item::TypeAlias),
    ))
    .map_with(|node, e| Spanned::new(node, e.span()))
    .labelled("definition")
}

fn structure<'src, I>() -> impl Parser<'src, I, Structure, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    keyword(Keyword::Structure)
        .ignore_then(
            bit_order().delimited_by(delimiter(Delimiter::LParen), delimiter(Delimiter::RParen)),
        )
        .then(ident())
        .then(params().or_not())
        .then(
            field()
                .separated_by(operator(Operator::Comma))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(delimiter(Delimiter::LBrace), delimiter(Delimiter::RBrace)),
        )
        .map_with(|(((bit_order, name), params), fields), _e| Structure {
            bit_order,
            name,
            params: params.unwrap_or_default(),
            fields,
        })
        .labelled("structure")
}

fn enumerated<'src, I>() -> impl Parser<'src, I, Enumerated, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    keyword(Keyword::Enumerated)
        .ignore_then(
            bit_order()
                .then_ignore(operator(Operator::Comma))
                .then(expr().map_with(|node, e| Spanned::new(node, e.span())))
                .then_ignore(keyword(Keyword::Bits))
                .delimited_by(delimiter(Delimiter::LParen), delimiter(Delimiter::RParen)),
        )
        .then(ident())
        .then(
            enum_variant()
                .separated_by(operator(Operator::Comma))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(delimiter(Delimiter::LBrace), delimiter(Delimiter::RBrace)),
        )
        .map_with(
            |(((bit_order, width_bits), name), variants), _e| Enumerated {
                bit_order,
                width_bits,
                name,
                variants,
            },
        )
        .labelled("enumerated")
}

fn choice_def<'src, I>() -> impl Parser<'src, I, Choice, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    keyword(Keyword::Choice)
        .ignore_then(
            bit_order().delimited_by(delimiter(Delimiter::LParen), delimiter(Delimiter::RParen)),
        )
        .then(ident())
        .then(params().or_not())
        .then_ignore(keyword(Keyword::On))
        .then(ident())
        .then(
            choice_arm()
                .separated_by(operator(Operator::Comma))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(delimiter(Delimiter::LBrace), delimiter(Delimiter::RBrace)),
        )
        .map_with(
            |((((bit_order, name), params), discriminant), arms), _e| Choice {
                bit_order,
                name,
                params: params.unwrap_or_default(),
                discriminant,
                arms,
            },
        )
        .labelled("choice")
}

fn type_alias<'src, I>() -> impl Parser<'src, I, TypeAlias, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    keyword(Keyword::Type)
        .ignore_then(ident())
        .then(width_bits().or_not())
        .then_ignore(operator(Operator::Equal))
        .then(type_expr())
        .then_ignore(operator(Operator::Semicolon))
        .map_with(|((name, width_bits), ty), e| TypeAlias {
            name,
            width_bits: width_bits.map(|node| Spanned::new(node, e.span())),
            ty,
        })
        .labelled("type alias")
}

fn params<'src, I>() -> impl Parser<'src, I, Vec<Param>, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    param()
        .separated_by(operator(Operator::Comma))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(delimiter(Delimiter::LParen), delimiter(Delimiter::RParen))
}

fn param<'src, I>() -> impl Parser<'src, I, Param, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    ident()
        .then_ignore(operator(Operator::Colon))
        .then(ident())
        .map(|(name, ty)| Param { name, ty })
}

fn field<'src, I>() -> impl Parser<'src, I, Spanned<Field>, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    ident()
        .then(width_bits().or_not())
        .then_ignore(operator(Operator::Colon))
        .then(type_expr())
        .then(attributes().or_not())
        .map_with(|(((name, width_bits), ty), attributes), e| {
            Spanned::new(
                Field {
                    name,
                    width_bits,
                    ty,
                    attributes: attributes.unwrap_or_default(),
                },
                e.span(),
            )
        })
}

fn attributes<'src, I>() -> impl Parser<'src, I, Vec<Attribute>, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    ident()
        .separated_by(operator(Operator::Comma))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            delimiter(Delimiter::LBracket),
            delimiter(Delimiter::RBracket),
        )
        .map(|names| {
            names
                .into_iter()
                .map(|n| Attribute::from_ident(&n))
                .collect()
        })
}

fn width_bits<'src, I>() -> impl Parser<'src, I, Expr, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    expr()
        .then_ignore(keyword(Keyword::Bits))
        .delimited_by(delimiter(Delimiter::LParen), delimiter(Delimiter::RParen))
}

fn bit_order<'src, I>() -> impl Parser<'src, I, BitOrder, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    choice((
        keyword(Keyword::Msb0).to(BitOrder::Msb0),
        keyword(Keyword::Lsb0).to(BitOrder::Lsb0),
    ))
}

fn enum_variant<'src, I>() -> impl Parser<'src, I, Spanned<EnumVariant>, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    ident()
        .then_ignore(operator(Operator::Equal))
        .then(enum_value())
        .map_with(|(name, value), e| Spanned::new(EnumVariant { name, value }, e.span()))
}

fn enum_value<'src, I>() -> impl Parser<'src, I, EnumValue, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    integer_literal()
        .then(
            operator(Operator::DoubleDot)
                .ignore_then(integer_literal())
                .or_not(),
        )
        .map(|(start, end)| match end {
            Some(end) => EnumValue::Range { start, end },
            None => EnumValue::Integer(start),
        })
}

fn choice_arm<'src, I>() -> impl Parser<'src, I, Spanned<ChoiceArm>, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    choice_pattern()
        .then_ignore(operator(Operator::FatArrow))
        .then(type_expr())
        .map_with(|(pattern, ty), e| Spanned::new(ChoiceArm { pattern, ty }, e.span()))
}

fn choice_pattern<'src, I>() -> impl Parser<'src, I, ChoicePattern, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    choice((
        ident()
            .filter(|name| name == "_")
            .to(ChoicePattern::Wildcard),
        ident()
            .then_ignore(operator(Operator::DoubleColon))
            .then(ident())
            .map(|(ty, variant)| ChoicePattern::Path { ty, variant }),
        integer_literal().map(ChoicePattern::Integer),
    ))
}

fn type_expr<'src, I>() -> impl Parser<'src, I, TypeExpr, ParserError<'src>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    let null_ty = select! {
        TokenKind::Literal(Literal::Null) => TypeExpr::Null
    };

    let named = ident()
        .then(
            expr()
                .separated_by(operator(Operator::Comma))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(delimiter(Delimiter::LAngle), delimiter(Delimiter::RAngle))
                .or_not(),
        )
        .then(
            expr()
                .separated_by(operator(Operator::Comma))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(delimiter(Delimiter::LParen), delimiter(Delimiter::RParen))
                .or_not(),
        )
        .map(|((name, generics), args)| TypeExpr::Named {
            name,
            generics: generics.unwrap_or_default(),
            args: args.unwrap_or_default(),
        });

    choice((null_ty, named)).labelled("type")
}

fn expr<'src, I>() -> impl Parser<'src, I, Expr, ParserError<'src>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    recursive(|expr| {
        let atom = choice((
            expr.clone()
                .delimited_by(delimiter(Delimiter::LParen), delimiter(Delimiter::RParen)),
            literal().map(Expr::Literal),
            ident().map(Expr::Ident),
        ));

        let field = atom.foldl(
            operator(Operator::Dot).ignore_then(ident()).repeated(),
            |base, name| Expr::Field {
                base: Box::new(base),
                name,
            },
        );

        let call = field
            .then(
                expr.clone()
                    .separated_by(operator(Operator::Comma))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(delimiter(Delimiter::LParen), delimiter(Delimiter::RParen))
                    .or_not(),
            )
            .map(|(callee, args)| match args {
                Some(args) => Expr::Call {
                    callee: Box::new(callee),
                    args,
                },
                None => callee,
            });

        let factor = call.clone().foldl(
            choice((
                operator(Operator::Star).to(BinOp::Mul),
                operator(Operator::Slash).to(BinOp::Div),
            ))
            .then(call)
            .repeated(),
            |lhs, (op, rhs)| Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        factor.clone().foldl(
            choice((
                operator(Operator::Plus).to(BinOp::Add),
                operator(Operator::Minus).to(BinOp::Sub),
            ))
            .then(factor)
            .repeated(),
            |lhs, (op, rhs)| Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::super::token::Literal;
    use super::*;

    fn parse_ok(source: &str) -> File {
        parse_source(source).unwrap_or_else(|errors| {
            panic!("parse failed: {errors:?}");
        })
    }

    #[test]
    fn parse_minimal_structure() {
        let file = parse_ok(
            r#"
            protocol Demo;
            structure(msb0) Header {
                payload_size (16 bits): u16,
                flag (1 bit): boolean
            }
            "#,
        );

        assert_eq!(
            file.protocol.as_ref().map(|p| p.node.as_str()),
            Some("Demo")
        );
        assert_eq!(file.items.len(), 1);
        let Item::Structure(structure) = &file.items[0].node else {
            panic!("expected structure");
        };
        assert_eq!(structure.name, "Header");
        assert_eq!(structure.fields.len(), 2);
        assert_eq!(
            structure.fields[0].node.width_bits,
            Some(Expr::Literal(Literal::new_integer(16, 10)))
        );
    }

    #[test]
    fn parse_enumerated_ranges() {
        let file = parse_ok(
            r#"
            enumerated(msb0, 8 bits) MessageType {
                iq_data = 0,
                reserved = 12..63,
            }
            "#,
        );

        let Item::Enumerated(enumerated) = &file.items[0].node else {
            panic!("expected enumerated");
        };
        assert_eq!(enumerated.variants.len(), 2);
        assert!(matches!(
            enumerated.variants[1].node.value,
            EnumValue::Range { .. }
        ));
    }

    #[test]
    fn parse_choice_and_type_alias() {
        let file = parse_ok(
            r#"
            type PC_ID (16 bits) = u16;
            choice(msb0) Payload(message_type: MessageType, payload_size: u16) on message_type {
                MessageType::iq_data => IQData(payload_size),
            }
            "#,
        );

        assert!(matches!(file.items[0].node, Item::TypeAlias(_)));
        let Item::Choice(choice) = &file.items[1].node else {
            panic!("expected choice");
        };
        assert_eq!(choice.params.len(), 2);
        assert_eq!(choice.arms.len(), 1);
    }

    #[test]
    fn parse_dynamic_array_field() {
        let file = parse_ok(
            r#"
            structure(msb0) IQData(payload_size: u16) {
                pc_id: PC_ID,
                iq_samples: DynamicArray<byte, payload_size - size(pc_id)>
            }
            "#,
        );

        let Item::Structure(structure) = &file.items[0].node else {
            panic!("expected structure");
        };
        let TypeExpr::Named {
            name,
            generics,
            args,
        } = &structure.fields[1].node.ty
        else {
            panic!("expected named type");
        };
        assert_eq!(name, "DynamicArray");
        assert_eq!(generics.len(), 2);
        assert!(args.is_empty());
        assert!(matches!(generics[1], Expr::Binary { op: BinOp::Sub, .. }));
    }

    #[test]
    fn parse_ecpri_example() {
        let source = include_str!("../../../examples/ecpri.mek");
        let file = parse_ok(source);

        assert_eq!(
            file.protocol.as_ref().map(|p| p.node.as_str()),
            Some("eCPRI")
        );
        assert!(
            file.items
                .iter()
                .any(|item| matches!(&item.node, Item::Structure(s) if s.name == "Message"))
        );
        assert!(
            file.items
                .iter()
                .any(|item| matches!(&item.node, Item::Choice(c) if c.name == "Payload"))
        );
        assert!(
            file.items
                .iter()
                .any(|item| matches!(&item.node, Item::Structure(s) if s.name == "Pdu"))
        );
        assert_eq!(file.items.len(), 29);
    }

    #[test]
    fn parse_generics_sketch_example() {
        let source = include_str!("../../../examples/generics-sketch.mek");
        let file = parse_ok(source);
        assert_eq!(
            file.protocol.as_ref().map(|p| p.node.as_str()),
            Some("GenericsSketch")
        );
        assert_eq!(file.items.len(), 6);
    }
}
