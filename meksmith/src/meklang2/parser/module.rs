use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::{Definition, Module};
use crate::meklang2::parser::bitenum::bit_enum;
use crate::meklang2::parser::bitstruct::bit_struct;

pub(crate) fn definition<'src>() -> impl Parser<'src, &'src str, Definition, ErrType<'src>> {
    choice((
        bit_enum().map(Definition::Enum),
        bit_struct().map(Definition::Struct),
    ))
}

pub(crate) fn module<'src>() -> impl Parser<'src, &'src str, Module, ErrType<'src>> {
    definition()
        .repeated()
        .collect()
        .map(|definitions| Module { definitions })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::meklang2::ast::{
        BitEnum, BitEnumField, BitStruct, BitStructBitOrder, BitStructBuiltinType, BitStructField,
        BitStructFieldBitsType, BitStructFieldType, Identifier,
    };

    const STR_ENUM_SWITCH: &str = r#"
        bitenum(1) Switch {
            off = 0,
            on = 1
        }
    "#;

    const STR_ENUM_BRIGHTNESS: &str = r#"
        bitenum(2) Brightness {
            quarter = 0,
            half = 1,
            three_quarters = 2,
            full = 3
        }
    "#;

    const STR_STRUCT_COLOR: &str = r#"
        bitstruct Color(msb0) {
            bit +8 red: u8,
            bit +8 green: u8,
            bit +8 blue: u8,
            bit +8 alpha: u8
        }
    "#;

    const STR_STRUCT_LIGHT: &str = r#"
        bitstruct Light(msb0) {
            bit derived switch_status: Switch,
            bit derived brightness: Brightness,
            bit derived color: Color
        }
    "#;

    fn expected_def_enum_switch() -> Definition {
        Definition::Enum(BitEnum {
            size: 1,
            name: Identifier::new("Switch"),
            fields: vec![
                BitEnumField::SingleValue {
                    name: Identifier::new("off"),
                    value: 0,
                },
                BitEnumField::SingleValue {
                    name: Identifier::new("on"),
                    value: 1,
                },
            ],
        })
    }

    fn expected_def_enum_brightness() -> Definition {
        Definition::Enum(BitEnum {
            size: 2,
            name: Identifier::new("Brightness"),
            fields: vec![
                BitEnumField::SingleValue {
                    name: Identifier::new("quarter"),
                    value: 0,
                },
                BitEnumField::SingleValue {
                    name: Identifier::new("half"),
                    value: 1,
                },
                BitEnumField::SingleValue {
                    name: Identifier::new("three_quarters"),
                    value: 2,
                },
                BitEnumField::SingleValue {
                    name: Identifier::new("full"),
                    value: 3,
                },
            ],
        })
    }

    fn expected_def_color() -> Definition {
        Definition::Struct(BitStruct {
            name: Identifier::new("Color"),
            bit_order: BitStructBitOrder::MostSignificantBitIsBit0,
            fields: vec![
                BitStructField::Ordinary {
                    name: Identifier::new("red"),
                    typ: BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger8),
                    bits: BitStructFieldBitsType::Increment { amount: 8 },
                    attributes: vec![],
                },
                BitStructField::Ordinary {
                    name: Identifier::new("green"),
                    typ: BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger8),
                    bits: BitStructFieldBitsType::Increment { amount: 8 },
                    attributes: vec![],
                },
                BitStructField::Ordinary {
                    name: Identifier::new("blue"),
                    typ: BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger8),
                    bits: BitStructFieldBitsType::Increment { amount: 8 },
                    attributes: vec![],
                },
                BitStructField::Ordinary {
                    name: Identifier::new("alpha"),
                    typ: BitStructFieldType::Builtin(BitStructBuiltinType::UnsignedInteger8),
                    bits: BitStructFieldBitsType::Increment { amount: 8 },
                    attributes: vec![],
                },
            ],
        })
    }

    fn expected_def_light() -> Definition {
        Definition::Struct(BitStruct {
            name: Identifier::new("Light"),
            bit_order: BitStructBitOrder::MostSignificantBitIsBit0,
            fields: vec![
                BitStructField::Ordinary {
                    name: Identifier::new("switch_status"),
                    typ: BitStructFieldType::UserDefined(Identifier::new("Switch")),
                    bits: BitStructFieldBitsType::Derived,
                    attributes: vec![],
                },
                BitStructField::Ordinary {
                    name: Identifier::new("brightness"),
                    typ: BitStructFieldType::UserDefined(Identifier::new("Brightness")),
                    bits: BitStructFieldBitsType::Derived,
                    attributes: vec![],
                },
                BitStructField::Ordinary {
                    name: Identifier::new("color"),
                    typ: BitStructFieldType::UserDefined(Identifier::new("Color")),
                    bits: BitStructFieldBitsType::Derived,
                    attributes: vec![],
                },
            ],
        })
    }

    #[rstest]
    #[case::enum_switch(STR_ENUM_SWITCH, expected_def_enum_switch())]
    #[case::enum_brightness(STR_ENUM_BRIGHTNESS, expected_def_enum_brightness())]
    #[case::struct_color(STR_STRUCT_COLOR, expected_def_color())]
    #[case::struct_light(STR_STRUCT_LIGHT, expected_def_light())]
    fn test_definition(#[case] input: &str, #[case] expected: Definition) {
        let result = definition().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case::ordered(
        vec![
            STR_ENUM_SWITCH,
            STR_ENUM_BRIGHTNESS,
            STR_STRUCT_COLOR,
            STR_STRUCT_LIGHT,
        ],
        Module {
            definitions: vec![
                expected_def_enum_switch(),
                expected_def_enum_brightness(),
                expected_def_color(),
                expected_def_light(),
            ],
        }
    )]
    #[case::random_order(
        vec![
            STR_STRUCT_COLOR,
            STR_ENUM_BRIGHTNESS,
            STR_STRUCT_LIGHT,
            STR_ENUM_SWITCH,
        ],
        Module {
            definitions: vec![
                expected_def_color(),
                expected_def_enum_brightness(),
                expected_def_light(),
                expected_def_enum_switch(),
            ],
        }
    )]
    fn test_module(#[case] input_vec: Vec<&str>, #[case] expected: Module) {
        let input = input_vec.join("\n");
        let input_ref = &input;
        let result = module().parse(input_ref);
        assert_eq!(result.into_output().unwrap(), expected);
    }
}
