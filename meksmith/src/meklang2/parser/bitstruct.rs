use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::{
    BitStruct, BitStructBitOrder, BitStructBuiltinType, BitStructField, BitStructFieldBitsType,
    BitStructFieldType, BitStructFieldUnion, BitStructOrdinaryFieldAttribute,
};
use crate::meklang2::parser::expr::expr;
use crate::meklang2::parser::ident::{identifier, reference};
use crate::meklang2::parser::numeric::number;
use crate::meklang2::parser::token::{
    BIG_ENDIAN, BIT, BIT_STRUCT, BOOLEAN, COLON, COMMA, DERIVED, DOUBLE_DOT, DYNAMIC_ARRAY, EQUALS,
    LBRACE, LBRACKET, LEAST_SIGNIFICANT_BIT_IS_BIT_0, LITTLE_ENDIAN, LPAREN, MAPS_TO,
    MOST_SIGNIFICANT_BIT_IS_BIT_0, PLUS, RBRACE, RBRACKET, RPAREN, SIGNED_INTEGER_8,
    SIGNED_INTEGER_16, SIGNED_INTEGER_32, SIGNED_INTEGER_64, STATIC_ARRAY, UNION,
    UNSIGNED_INTEGER_8, UNSIGNED_INTEGER_16, UNSIGNED_INTEGER_32, UNSIGNED_INTEGER_64, WHEN,
};

pub(crate) fn bit_struct_ordinary_field_attribute<'src>()
-> impl Parser<'src, &'src str, BitStructOrdinaryFieldAttribute, ErrType<'src>> {
    let little_endian = just(LITTLE_ENDIAN)
        .padded()
        .to(BitStructOrdinaryFieldAttribute::LittleEndian);

    let big_endian = just(BIG_ENDIAN)
        .padded()
        .to(BitStructOrdinaryFieldAttribute::BigEndian);

    let static_array = just(STATIC_ARRAY)
        .ignore_then(just(EQUALS).padded())
        .ignore_then(number())
        .map(|size| BitStructOrdinaryFieldAttribute::StaticArray { size });

    let dynamic_array = just(DYNAMIC_ARRAY)
        .ignore_then(just(EQUALS).padded())
        .ignore_then(reference())
        .map(|reference| BitStructOrdinaryFieldAttribute::DynamicArray { reference });

    choice((little_endian, big_endian, static_array, dynamic_array))
}

pub(crate) fn bit_struct_ordinary_field_attributes<'src>()
-> impl Parser<'src, &'src str, Vec<BitStructOrdinaryFieldAttribute>, ErrType<'src>> {
    bit_struct_ordinary_field_attribute()
        .separated_by(just(COMMA).padded())
        .collect()
        .delimited_by(just(LBRACKET).padded(), just(RBRACKET).padded())
}

pub(crate) fn bit_struct_field_bits_type<'src>()
-> impl Parser<'src, &'src str, BitStructFieldBitsType, ErrType<'src>> {
    let range = just(BIT)
        .padded()
        .ignore_then(number())
        .then_ignore(just(DOUBLE_DOT).padded())
        .then(number())
        .map(|(from, to)| BitStructFieldBitsType::Range { from, to });

    let increment = just(BIT)
        .padded()
        .ignore_then(just(PLUS).padded())
        .ignore_then(number())
        .map(|amount| BitStructFieldBitsType::Increment { amount });

    let derived = just(BIT)
        .padded()
        .ignore_then(just(DERIVED).padded())
        .to(BitStructFieldBitsType::Derived);

    choice((range, increment, derived))
}

pub(crate) fn bit_struct_builtin_type<'src>()
-> impl Parser<'src, &'src str, BitStructBuiltinType, ErrType<'src>> {
    choice((
        just(UNSIGNED_INTEGER_8).to(BitStructBuiltinType::UnsignedInteger8),
        just(UNSIGNED_INTEGER_16).to(BitStructBuiltinType::UnsignedInteger16),
        just(UNSIGNED_INTEGER_32).to(BitStructBuiltinType::UnsignedInteger32),
        just(UNSIGNED_INTEGER_64).to(BitStructBuiltinType::UnsignedInteger64),
        just(SIGNED_INTEGER_8).to(BitStructBuiltinType::SignedInteger8),
        just(SIGNED_INTEGER_16).to(BitStructBuiltinType::SignedInteger16),
        just(SIGNED_INTEGER_32).to(BitStructBuiltinType::SignedInteger32),
        just(SIGNED_INTEGER_64).to(BitStructBuiltinType::SignedInteger64),
        just(BOOLEAN).to(BitStructBuiltinType::Boolean),
    ))
}

pub(crate) fn bit_struct_field_type<'src>()
-> impl Parser<'src, &'src str, BitStructFieldType, ErrType<'src>> {
    choice((
        bit_struct_builtin_type().map(BitStructFieldType::Builtin),
        identifier().map(BitStructFieldType::UserDefined),
    ))
}

pub(crate) fn bit_struct<'src>() -> impl Parser<'src, &'src str, BitStruct, ErrType<'src>> {
    // bitstruct <name>(<bit_struct_bit_order>) {
    //     <bit_struct_field>+
    // }

    just(BIT_STRUCT)
        .padded()
        .ignore_then(identifier())
        .then(bit_struct_bit_order().delimited_by(just(LPAREN).padded(), just(RPAREN).padded()))
        .then(
            bit_struct_field()
                .separated_by(just(COMMA).padded())
                .at_least(1)
                .collect()
                .delimited_by(just(LBRACE).padded(), just(RBRACE).padded()),
        )
        .map(|((name, bit_order), fields)| BitStruct {
            name,
            bit_order,
            fields,
        })
}

pub(crate) fn bit_struct_bit_order<'src>()
-> impl Parser<'src, &'src str, BitStructBitOrder, ErrType<'src>> {
    choice((
        just(MOST_SIGNIFICANT_BIT_IS_BIT_0).to(BitStructBitOrder::MostSignificantBitIsBit0),
        just(LEAST_SIGNIFICANT_BIT_IS_BIT_0).to(BitStructBitOrder::LeastSignificantBitIsBit0),
    ))
}

pub(crate) fn bit_struct_field<'src>() -> impl Parser<'src, &'src str, BitStructField, ErrType<'src>>
{
    // <bit_struct_field_bits_type> <name> : <type> [<attributes>]
    let ordinary_field = bit_struct_field_bits_type()
        .padded()
        .then(identifier())
        .padded()
        .then_ignore(just(COLON).padded())
        .then(bit_struct_field_type().padded())
        .then(bit_struct_ordinary_field_attributes().padded().or_not())
        .map(|(((bits, name), typ), attrs)| BitStructField::Ordinary {
            name,
            typ,
            bits,
            attributes: attrs.map_or(vec![], |attrs| attrs),
        });

    // <bit_struct_field_bits_type> union <name> (when <expr>)?
    // {
    //      <bit_struct_field_union>+
    // }
    let union_field = bit_struct_field_bits_type()
        .padded()
        .then_ignore(just(UNION).padded())
        .then(identifier())
        .then(just(WHEN).padded().ignore_then(expr()).or_not())
        .then(
            bit_struct_field_union()
                .separated_by(just(COMMA).padded())
                .at_least(1)
                .collect()
                .delimited_by(just(LBRACE).padded(), just(RBRACE).padded()),
        )
        .map(
            |(((bits, name), when_expr), fields)| BitStructField::Union {
                name,
                bits,
                when_expr,
                fields,
            },
        );

    choice((union_field, ordinary_field))
}

pub(crate) fn bit_struct_field_union<'src>()
-> impl Parser<'src, &'src str, BitStructFieldUnion, ErrType<'src>> {
    // 0x00 => x: Y
    number()
        .then_ignore(just(MAPS_TO).padded())
        .then(identifier())
        .then_ignore(just(COLON).padded())
        .then(bit_struct_field_type())
        .map(|((discriminator, name), typ)| BitStructFieldUnion {
            discriminator,
            name,
            typ,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::meklang2::ast::{
        BinaryOperator, Expr, Identifier, NumericLiteral, Radix, Reference,
    };

    #[rstest]
    #[case::little_endian("little endian", BitStructOrdinaryFieldAttribute::LittleEndian)]
    #[case::big_endian("big endian", BitStructOrdinaryFieldAttribute::BigEndian)]
    #[case::static_array("static array = 8", BitStructOrdinaryFieldAttribute::StaticArray { size: 8 })]
    #[case::static_array(
        "dynamic array = x.y",
        BitStructOrdinaryFieldAttribute::DynamicArray { reference: Reference { path: vec![
            Identifier::new("x"),
            Identifier::new("y"),
        ] }}
    )]
    fn test_bit_struct_ordinary_field_attribute(
        #[case] input: &str,
        #[case] expected: BitStructOrdinaryFieldAttribute,
    ) {
        let result = bit_struct_ordinary_field_attribute().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_bit_struct_ordinary_field_attributes() {
        let input = "[little endian, static array = 4]";
        let expected = vec![
            BitStructOrdinaryFieldAttribute::LittleEndian,
            BitStructOrdinaryFieldAttribute::StaticArray { size: 4 },
        ];

        let result = bit_struct_ordinary_field_attributes().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[rstest]
    #[case::range("bit 0..5", BitStructFieldBitsType::Range { from: 0, to: 5 })]
    #[case::increment("bit +6", BitStructFieldBitsType::Increment { amount: 6 })]
    #[case::derived("bit derived", BitStructFieldBitsType::Derived)]
    fn test_bit_struct_field_bits_type(
        #[case] input: &str,
        #[case] expected: BitStructFieldBitsType,
    ) {
        let result = bit_struct_field_bits_type().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[rstest]
    #[case::u8("u8", BitStructBuiltinType::UnsignedInteger8)]
    #[case::u16("u16", BitStructBuiltinType::UnsignedInteger16)]
    #[case::u32("u32", BitStructBuiltinType::UnsignedInteger32)]
    #[case::u64("u64", BitStructBuiltinType::UnsignedInteger64)]
    #[case::i8("i8", BitStructBuiltinType::SignedInteger8)]
    #[case::i16("i16", BitStructBuiltinType::SignedInteger16)]
    #[case::i32("i32", BitStructBuiltinType::SignedInteger32)]
    #[case::i64("i64", BitStructBuiltinType::SignedInteger64)]
    #[case::boolean("boolean", BitStructBuiltinType::Boolean)]
    fn test_bit_struct_builtin_type(#[case] input: &str, #[case] expected: BitStructBuiltinType) {
        let result = bit_struct_builtin_type().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[rstest]
    #[case::builtin_u8(
        "u8",
        BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger8)
    )]
    #[case::builtin_u16(
        "u16",
        BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger16)
    )]
    #[case::builtin_u32(
        "u32",
        BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger32)
    )]
    #[case::builtin_u64(
        "u64",
        BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger64)
    )]
    #[case::builtin_i8(
        "i8",
        BitStructFieldType::Builtin(BitStructBuiltinType::SignedInteger8)
    )]
    #[case::builtin_i16(
        "i16",
        BitStructFieldType::Builtin(BitStructBuiltinType::SignedInteger16)
    )]
    #[case::builtin_i32(
        "i32",
        BitStructFieldType::Builtin(BitStructBuiltinType::SignedInteger32)
    )]
    #[case::builtin_i64(
        "i64",
        BitStructFieldType::Builtin(BitStructBuiltinType::SignedInteger64)
    )]
    #[case::builtin_boolean("boolean", BitStructFieldType::Builtin(BitStructBuiltinType::Boolean))]
    #[case::user_defined("lalala", BitStructFieldType::UserDefined(Identifier::new("lalala")))]
    fn test_bit_struct_field_type(#[case] input: &str, #[case] expected: BitStructFieldType) {
        let result = bit_struct_field_type().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_bit_struct() {
        let input = r#"
            bitstruct EcpriMessage(msb0) {
                bit derived header: EcpriHeader,
                bit derived union payload when header.message_type {
                    0x00 => iq_data: EcpriIqData,
                    0x01 => bit_sequence: EcpriBitSequence
                }
            }
        "#;
        let expected = BitStruct {
            name: Identifier::new("EcpriMessage"),
            bit_order: BitStructBitOrder::MostSignificantBitIsBit0,
            fields: vec![
                BitStructField::Ordinary {
                    name: Identifier::new("header"),
                    typ: BitStructFieldType::UserDefined(Identifier::new("EcpriHeader")),
                    bits: BitStructFieldBitsType::Derived,
                    attributes: vec![],
                },
                BitStructField::Union {
                    name: Identifier::new("payload"),
                    bits: BitStructFieldBitsType::Derived,
                    when_expr: Some(Expr::Reference(Reference {
                        path: vec![Identifier::new("header"), Identifier::new("message_type")],
                    })),
                    fields: vec![
                        BitStructFieldUnion {
                            discriminator: 0x00,
                            name: Identifier::new("iq_data"),
                            typ: BitStructFieldType::UserDefined(Identifier::new("EcpriIqData")),
                        },
                        BitStructFieldUnion {
                            discriminator: 0x01,
                            name: Identifier::new("bit_sequence"),
                            typ: BitStructFieldType::UserDefined(Identifier::new(
                                "EcpriBitSequence",
                            )),
                        },
                    ],
                },
            ],
        };

        let result = bit_struct().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[rstest]
    #[case::msb0("msb0", BitStructBitOrder::MostSignificantBitIsBit0)]
    #[case::lsb0("lsb0", BitStructBitOrder::LeastSignificantBitIsBit0)]
    fn test_bit_struct_bit_order(#[case] input: &str, #[case] expected: BitStructBitOrder) {
        let result = bit_struct_bit_order().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[rstest]
    #[case::ordinary_with_attrs(
        "bit +1  my_field: u32 [little endian]",
        BitStructField::Ordinary {
            name: Identifier::new("my_field"),
            typ: BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger32),
            bits: BitStructFieldBitsType::Increment {amount: 1},
            attributes: vec![BitStructOrdinaryFieldAttribute::LittleEndian],
        }
    )]
    #[case::ordinary_without_attrs(
        "bit derived nested: SomethingNested",
        BitStructField::Ordinary {
            name: Identifier::new("nested"),
            typ: BitStructFieldType::UserDefined(Identifier::new("SomethingNested")),
            bits: BitStructFieldBitsType::Derived,
            attributes: vec![],
        }
    )]
    #[case::union_without_when_expr(
        r#"
            bit 0..16 union xxx {
                0x00 => first: boolean,
                0x01 => second: Second
            }
        "#,
        BitStructField::Union {
            name: Identifier::new("xxx"),
            bits: BitStructFieldBitsType::Range { from: 0, to: 16 },
            when_expr: None,
            fields: vec![
                BitStructFieldUnion {
                    discriminator: 0x00,
                    name: Identifier::new("first"),
                    typ: BitStructFieldType::Builtin(BitStructBuiltinType::Boolean),
                },
                BitStructFieldUnion {
                    discriminator: 0x01,
                    name: Identifier::new("second"),
                    typ: BitStructFieldType::UserDefined(Identifier::new("Second")),
                },
            ],
        }
    )]
    #[case::union_with_when_expr(
        r#"
            bit 0..16 union xxx when x == 1 or y > 2 {
                0x00 => first: boolean,
                0x01 => second: Second
            }
        "#,
        BitStructField::Union {
            name: Identifier::new("xxx"),
            bits: BitStructFieldBitsType::Range { from: 0, to: 16 },
            when_expr: Some(Expr::BinaryOperator {
                operator: BinaryOperator::Or,
                lhs: Box::new(Expr::BinaryOperator {
                    operator: BinaryOperator::Equals,
                    lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("x")] })),
                    rhs: Box::new(Expr::NumericLiteral(NumericLiteral { value: 1, radix: Radix::Decimal })),
                }),
                rhs: Box::new(Expr::BinaryOperator {
                    operator: BinaryOperator::GreaterThan,
                    lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("y")] })),
                    rhs: Box::new(Expr::NumericLiteral(NumericLiteral { value: 2, radix: Radix::Decimal })),
                }),
            }),
            fields: vec![
                BitStructFieldUnion {
                    discriminator: 0x00,
                    name: Identifier::new("first"),
                    typ: BitStructFieldType::Builtin(BitStructBuiltinType::Boolean),
                },
                BitStructFieldUnion {
                    discriminator: 0x01,
                    name: Identifier::new("second"),
                    typ: BitStructFieldType::UserDefined(Identifier::new("Second")),
                },
            ],
        }
    )]
    fn test_bit_struct_field(#[case] input: &str, #[case] expected: BitStructField) {
        let result = bit_struct_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_bit_struct_field_union() {
        let input = "0x42 => answer_to: Everything";
        let expected = BitStructFieldUnion {
            discriminator: 0x42,
            name: Identifier::new("answer_to"),
            typ: BitStructFieldType::UserDefined(Identifier::new("Everything")),
        };

        let result = bit_struct_field_union().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }
}
