use crate::ast::{
    Definition, EnumerationDefinition, EnumerationField, Protocol, StructureDefinition,
    TypeDefinition, TypeIdentifier, UnionDefinition, UnionField,
};

fn generate_enumeration_code(enumeration: &EnumerationDefinition) -> String {
    let mut code = String::new();
    code.push_str("typedef enum {\n");
    for field in &enumeration.fields {
        match field {
            EnumerationField::SingleValue { name, value } => {
                code.push_str(&format!(
                    "    {}_{} = {},\n",
                    enumeration.name.name, name.name, value
                ));
            }
            EnumerationField::RangeOfValues { name, start, end } => {
                if start == end {
                    code.push_str(&format!(
                        "    {}_{} = {},\n",
                        enumeration.name.name, name.name, start
                    ));
                } else {
                    for i in *start..=*end {
                        code.push_str(&format!(
                            "    {}_{}_{} = {},\n",
                            enumeration.name.name, name.name, i, i
                        ));
                    }
                }
            }
        }
    }
    code.push_str(&format!("}} {};\n\n", enumeration.name.name));
    code
}

fn generate_type_definition_code(type_definition: &TypeDefinition) -> String {
    match &type_definition.r#type {
        TypeIdentifier::StaticArray { r#type, size } => {
            format!(
                "typedef {} {}[{}];\n\n",
                generate_type_identifier_code(r#type),
                type_definition.new_type.name,
                size
            )
        }
        TypeIdentifier::DynamicArray { r#type } => {
            format!(
                "typedef {}* {};\n\n",
                generate_type_identifier_code(r#type),
                type_definition.new_type.name
            )
        }
        _ => {
            let type_code = generate_type_identifier_code(&type_definition.r#type);
            format!(
                "typedef {} {};\n\n",
                type_code, type_definition.new_type.name
            )
        }
    }
}

fn generate_type_identifier_code(type_identifier: &TypeIdentifier) -> String {
    match type_identifier {
        TypeIdentifier::Integer8 => "int8_t".to_string(),
        TypeIdentifier::Integer16 => "int16_t".to_string(),
        TypeIdentifier::Integer32 => "int32_t".to_string(),
        TypeIdentifier::Integer64 => "int64_t".to_string(),
        TypeIdentifier::UnsignedInteger8 => "uint8_t".to_string(),
        TypeIdentifier::UnsignedInteger16 => "uint16_t".to_string(),
        TypeIdentifier::UnsignedInteger32 => "uint32_t".to_string(),
        TypeIdentifier::UnsignedInteger64 => "uint64_t".to_string(),
        TypeIdentifier::Float32 => "float".to_string(),
        TypeIdentifier::Float64 => "double".to_string(),
        TypeIdentifier::Bit => "bool".to_string(),
        TypeIdentifier::Byte => "uint8_t".to_string(),
        TypeIdentifier::UserDefined(identifier) => identifier.name.clone(),
        TypeIdentifier::StaticArray { r#type, .. } => {
            // Only return the type, not the array part
            generate_type_identifier_code(r#type)
        }
        TypeIdentifier::DynamicArray { r#type } => {
            format!("{}*", generate_type_identifier_code(r#type))
        }
    }
}

fn generate_structure_code(structure: &StructureDefinition) -> String {
    let mut code = String::new();
    code.push_str("typedef struct {\n");
    for field in &structure.fields {
        match &field.r#type {
            TypeIdentifier::StaticArray { r#type, size } => {
                code.push_str(&format!(
                    "    {} {}[{}];\n",
                    generate_type_identifier_code(r#type),
                    field.name.name,
                    size
                ));
            }
            _ => {
                code.push_str(&format!(
                    "    {} {};\n",
                    generate_type_identifier_code(&field.r#type),
                    field.name.name
                ));
            }
        }
    }
    code.push_str(&format!("}} {};\n\n", structure.name.name));
    code
}

fn generate_union_code(union: &UnionDefinition) -> String {
    let mut code = String::new();
    code.push_str("typedef union {\n");
    for field in &union.fields {
        match field {
            UnionField::SingleValue { name, r#type, .. } => match r#type {
                TypeIdentifier::StaticArray {
                    r#type: inner_type,
                    size,
                } => {
                    code.push_str(&format!(
                        "    {} {}[{}];\n",
                        generate_type_identifier_code(inner_type),
                        name.name,
                        size
                    ));
                }
                _ => {
                    code.push_str(&format!(
                        "    {} {};\n",
                        generate_type_identifier_code(r#type),
                        name.name
                    ));
                }
            },
            UnionField::RangeOfValues {
                name,
                r#type,
                start_discriminator,
                end_discriminator,
            } => {
                for i in *start_discriminator..=*end_discriminator {
                    match r#type {
                        TypeIdentifier::StaticArray {
                            r#type: inner_type,
                            size,
                        } => {
                            code.push_str(&format!(
                                "    {} {}_{}[{}];\n",
                                generate_type_identifier_code(inner_type),
                                name.name,
                                i,
                                size
                            ));
                        }
                        _ => {
                            code.push_str(&format!(
                                "    {} {}_{};\n",
                                generate_type_identifier_code(r#type),
                                name.name,
                                i
                            ));
                        }
                    }
                }
            }
        }
    }
    code.push_str(&format!("}} {};\n\n", union.name.name));
    code
}

pub fn generate_c_code(protocol: &Protocol) -> String {
    let mut code = String::new();
    code.push_str("#include <stdint.h>\n#include <stdbool.h>\n\n");

    for definition in &protocol.definitions {
        match definition {
            Definition::Enumeration(enumeration) => {
                code.push_str(&generate_enumeration_code(enumeration));
            }
            Definition::Structure(structure) => {
                code.push_str(&generate_structure_code(structure));
            }
            Definition::Type(type_definition) => {
                code.push_str(&generate_type_definition_code(type_definition));
            }
            Definition::Union(union) => {
                code.push_str(&generate_union_code(union));
            }
        }
    }
    code
}

pub fn generate_c_code_from_string(input: &str) -> Result<String, String> {
    let protocol = crate::parse_protocol_to_ast(input)?;
    let sorted = crate::ast::sort_protocol_by_dependencies(&protocol)?;
    Ok(generate_c_code(&sorted))
}

pub fn generate_from_file(file_path: &str) -> Result<String, String> {
    let protocol = crate::parse_protocol_from_file_to_ast(file_path)?;
    let sorted = crate::ast::sort_protocol_by_dependencies(&protocol)?;
    Ok(generate_c_code(&sorted))
}

pub fn generate_from_file_to_file(
    input_file_path: &str,
    output_file_path: &str,
) -> Result<(), String> {
    let c_code = generate_from_file(input_file_path)?;
    std::fs::write(output_file_path, c_code)
        .map_err(|e| format!("Failed to write to file: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    static INPUT_FILE_CONTENT: &str = r#"
using BuiltInType = int32;
using UserDefinedType = MyEnum;
using StaticArrayType = uint32[10];
using DynamicArrayType = byte[];

enum MyEnum {
    Value = 1;
    Range = 2..5;
    RangeOneValue = 6..6;
};

using my_enum_alias_t = MyEnum;

struct MyStruct {
    field1: int32;
    field2: MyEnum;
    field3: uint32[10];
    field4: byte[];
    field5: my_enum_alias_t;
    field6: MyEnum[2];
    builtin1: int8;
    builtin2: int16;
    builtin3: int32;
    builtin4: int64;
    builtin5: uint8;
    builtin6: uint16;
    builtin7: uint32;
    builtin8: uint64;
    builtin9: float32;
    builtin10: float64;
    builtin11: bit;
    builtin12: byte;
};

union MyUnion {
    0 => field1: bit;
    1 => field2: MyEnum;
    2 => field3: uint64[10];
    3 => field4: MyStruct;
    4..6 => reserved: uint16;
    7..8 => static_array: uint16[10];
};
"#;

    static EXPECTED_C_OUTPUT: &str = r#"#include <stdint.h>
#include <stdbool.h>

typedef int32_t BuiltInType;

typedef enum {
    MyEnum_Value = 1,
    MyEnum_Range_2 = 2,
    MyEnum_Range_3 = 3,
    MyEnum_Range_4 = 4,
    MyEnum_Range_5 = 5,
    MyEnum_RangeOneValue = 6,
} MyEnum;

typedef MyEnum UserDefinedType;

typedef uint32_t StaticArrayType[10];

typedef uint8_t* DynamicArrayType;

typedef MyEnum my_enum_alias_t;

typedef struct {
    int32_t field1;
    MyEnum field2;
    uint32_t field3[10];
    uint8_t* field4;
    my_enum_alias_t field5;
    MyEnum field6[2];
    int8_t builtin1;
    int16_t builtin2;
    int32_t builtin3;
    int64_t builtin4;
    uint8_t builtin5;
    uint16_t builtin6;
    uint32_t builtin7;
    uint64_t builtin8;
    float builtin9;
    double builtin10;
    bool builtin11;
    uint8_t builtin12;
} MyStruct;

typedef union {
    bool field1;
    MyEnum field2;
    uint64_t field3[10];
    MyStruct field4;
    uint16_t reserved_4;
    uint16_t reserved_5;
    uint16_t reserved_6;
    uint16_t static_array_7[10];
    uint16_t static_array_8[10];
} MyUnion;

"#;

    #[test]
    fn test_generate_c_code_from_string() {
        let input = INPUT_FILE_CONTENT;
        let output = generate_c_code_from_string(input).unwrap();
        assert_eq!(output, EXPECTED_C_OUTPUT);
    }

    #[test]
    fn test_generate_from_file() {
        let input_file = NamedTempFile::new().expect("Failed to create temporary file");
        std::fs::write(input_file.path(), INPUT_FILE_CONTENT).unwrap();

        let output = generate_from_file(input_file.path().to_str().unwrap()).unwrap();

        assert_eq!(output, EXPECTED_C_OUTPUT);
        std::fs::remove_file(input_file.path().to_str().unwrap()).unwrap();
    }

    #[test]
    fn test_generate_from_file_to_file() {
        let input_file = NamedTempFile::new().expect("Failed to create temporary file");
        let output_file = NamedTempFile::new().expect("Failed to create temporary file");

        std::fs::write(input_file.path(), INPUT_FILE_CONTENT).unwrap();

        assert!(
            generate_from_file_to_file(
                input_file.path().to_str().unwrap(),
                output_file.path().to_str().unwrap()
            )
            .is_ok()
        );

        let output = std::fs::read_to_string(output_file.path().to_str().unwrap()).unwrap();
        assert_eq!(output, EXPECTED_C_OUTPUT);
        std::fs::remove_file(input_file.path().to_str().unwrap()).unwrap();
        std::fs::remove_file(output_file.path().to_str().unwrap()).unwrap();
    }
}
