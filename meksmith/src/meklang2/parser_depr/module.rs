use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::{Definition, Metadata, Module};
use crate::meklang2::parser_depr::bitenum::bit_enum;
use crate::meklang2::parser_depr::bitstruct::bit_struct;
use crate::meklang2::parser_depr::token::{
    DESCRIPTION, DOCS, LBRACE, METADATA, MODULE, RBRACE, VERSION,
};

pub(crate) fn metadata<'src>() -> impl Parser<'src, &'src str, Metadata, ErrType<'src>> {
    let any_string = any()
        .filter(|c: &char| *c != '\n')
        .repeated()
        .collect::<Vec<char>>()
        .then_ignore(just("\n"));

    let field = |field_name| {
        just(field_name)
            .padded()
            .ignore_then(any_string)
            .map(|chars| chars.into_iter().collect::<String>())
    };

    let fields = field(MODULE)
        .then(field(VERSION).or_not())
        .then(field(DESCRIPTION).or_not())
        .then(field(DOCS).or_not())
        .delimited_by(just(LBRACE).padded(), just(RBRACE).padded());

    just(METADATA)
        .ignore_then(fields)
        .map(|(((module_name, version), description), docs)| Metadata {
            module_name,
            version,
            description,
            docs,
        })
}

pub(crate) fn definition<'src>() -> impl Parser<'src, &'src str, Definition, ErrType<'src>> {
    choice((
        bit_enum().map(Definition::Enum),
        bit_struct().map(Definition::Struct),
    ))
}

pub(crate) fn module<'src>() -> impl Parser<'src, &'src str, Module, ErrType<'src>> {
    metadata()
        .then(definition().repeated().collect())
        .map(|(metadata, definitions)| Module {
            metadata,
            definitions,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::meklang2::ast::{
        BitEnum, BitEnumField, BitStruct, BitStructBitOrder, BitStructBuiltinType, BitStructField,
        BitStructFieldBitsType, BitStructFieldType, Identifier,
    };

    const STR_MODULE: &str = "light_proto";
    const STR_VERSION: &str = "1.2.3-rev3";
    const STR_DESCRIPTION: &str = "small protocol to manage lightbulb behavior";
    const STR_DOCS: &str = "https://light-proto.meksmith.rs/docs/1.2.3-rev3.pdf";

    const METADATA_MODULE: &str = "module light_proto";
    const METADATA_VERSION: &str = "version 1.2.3-rev3";
    const METADATA_DESCRIPTION: &str = "description small protocol to manage lightbulb behavior";
    const METADATA_DOCS: &str = "docs https://light-proto.meksmith.rs/docs/1.2.3-rev3.pdf";

    fn str_metadata() -> String {
        format!(
            "metadata {{ \n\t{}\n\t{}\n\t{}\n\t{}\n }}",
            METADATA_MODULE, METADATA_VERSION, METADATA_DESCRIPTION, METADATA_DOCS
        )
    }

    fn expected_metadata() -> Metadata {
        Metadata {
            module_name: String::from(STR_MODULE),
            version: Some(String::from(STR_VERSION)),
            description: Some(String::from(STR_DESCRIPTION)),
            docs: Some(String::from(STR_DOCS)),
        }
    }

    #[test]
    fn test_metadata_empty() {
        let input = "metadata { }";
        let result = metadata().parse(input);
        assert!(result.has_errors());
    }

    #[rstest]
    #[case::only_module_name(vec![], None, None, None)]
    #[case::with_version(vec![METADATA_VERSION], Some(STR_VERSION.to_string()), None, None)]
    #[case::with_description(vec![METADATA_DESCRIPTION], None, Some(STR_DESCRIPTION.to_string()), None)]
    #[case::with_docs(vec![METADATA_DOCS], None, None, Some(STR_DOCS.to_string()))]
    #[case::with_version_and_description(vec![METADATA_VERSION, METADATA_DESCRIPTION], Some(STR_VERSION.to_string()), Some(STR_DESCRIPTION.to_string()), None)]
    #[case::with_version_and_docs(vec![METADATA_VERSION, METADATA_DOCS], Some(STR_VERSION.to_string()), None, Some(STR_DOCS.to_string()))]
    #[case::with_description_and_docs(vec![METADATA_DESCRIPTION, METADATA_DOCS], None, Some(STR_DESCRIPTION.to_string()), Some(STR_DOCS.to_string()))]
    #[case::with_everything(vec![METADATA_VERSION, METADATA_DESCRIPTION, METADATA_DOCS], Some(STR_VERSION.to_string()), Some(STR_DESCRIPTION.to_string()), Some(STR_DOCS.to_string()))]
    fn test_metadata(
        #[case] input_fields: Vec<&str>,
        #[case] version: Option<String>,
        #[case] description: Option<String>,
        #[case] docs: Option<String>,
    ) {
        let fields = input_fields.join("\n\t");
        let input = format!("metadata {{ \n\t{}\n\t{}\n}}", METADATA_MODULE, fields);
        let expected = Metadata {
            module_name: String::from(STR_MODULE),
            version,
            description,
            docs,
        };

        let result = metadata().parse(input.as_str());
        assert_eq!(result.into_output().unwrap(), expected);
    }

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
            metadata: expected_metadata(),
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
            metadata: expected_metadata(),
            definitions: vec![
                expected_def_color(),
                expected_def_enum_brightness(),
                expected_def_light(),
                expected_def_enum_switch(),
            ],
        }
    )]
    fn test_module(#[case] input_vec: Vec<&str>, #[case] expected: Module) {
        let input_defs = input_vec.join("\n");
        let input = format!("{}\n{}", str_metadata(), input_defs);
        let input_ref = &input;

        let result = module().parse(input_ref);
        assert_eq!(result.into_output().unwrap(), expected);
    }
}
