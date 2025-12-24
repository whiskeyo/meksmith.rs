use chumsky::prelude::*;

use crate::meklang::ast::{Definition, Module};
use crate::meklang::parser::ErrType;
use crate::meklang::parser::definitions::definition;

pub(crate) const MODULE: &str = "module";
pub(crate) const VERSION: &str = "version";

pub(crate) fn module<'src>() -> impl Parser<'src, &'src str, Module, ErrType<'src>> {
    definition()
        .repeated()
        .collect::<Vec<Definition>>()
        .map(|definitions| Module { definitions })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meklang::ast::{
        Attribute, BuiltinType, Enumeration, EnumerationField, Identifier, Structure,
        StructureField, Type, Union, UnionField,
    };

    #[test]
    fn test_module() {
        let input = r#"
            [bits=2]
            enum Enum1 { x = 0b00, y = 0b01..0b11 }

            struct Struct1 {
                [static_array=4]
                f1: Enum1,
                f2: u32
            }

            enum Enum2 { z = 5 }

            union Union1 {
                [bits=49]
                0 => u1: u64,
                1 => u2: Enum2,
                2 => u3: Struct1,
            }
        "#;

        let expected = Module {
            definitions: vec![
                Definition::Enumeration(Enumeration {
                    name: Identifier::new("Enum1"),
                    fields: vec![
                        EnumerationField::SingleValue {
                            name: Identifier::new("x"),
                            value: 0b00,
                        },
                        EnumerationField::RangeOfValues {
                            name: Identifier::new("y"),
                            from: 0b01,
                            to: 0b11,
                        },
                    ],
                    attributes: vec![Attribute::Bits(2)],
                }),
                Definition::Structure(Structure {
                    name: Identifier::new("Struct1"),
                    fields: vec![
                        StructureField {
                            name: Identifier::new("f1"),
                            typ: Type::UserDefined(Identifier::new("Enum1")),
                            attributes: vec![Attribute::StaticArray(4)],
                        },
                        StructureField {
                            name: Identifier::new("f2"),
                            typ: Type::Builtin(BuiltinType::UnsignedInteger32),
                            attributes: vec![],
                        },
                    ],
                }),
                Definition::Enumeration(Enumeration {
                    name: Identifier::new("Enum2"),
                    fields: vec![EnumerationField::SingleValue {
                        name: Identifier::new("z"),
                        value: 5,
                    }],
                    attributes: vec![],
                }),
                Definition::Union(Union {
                    name: Identifier::new("Union1"),
                    fields: vec![
                        UnionField::SingleValue {
                            name: Identifier::new("u1"),
                            typ: Type::Builtin(BuiltinType::UnsignedInteger64),
                            discriminator: 0,
                            attributes: vec![Attribute::Bits(49)],
                        },
                        UnionField::SingleValue {
                            name: Identifier::new("u2"),
                            typ: Type::UserDefined(Identifier::new("Enum2")),
                            discriminator: 1,
                            attributes: vec![],
                        },
                        UnionField::SingleValue {
                            name: Identifier::new("u3"),
                            typ: Type::UserDefined(Identifier::new("Struct1")),
                            discriminator: 2,
                            attributes: vec![],
                        },
                    ],
                }),
            ],
        };
    }
}
