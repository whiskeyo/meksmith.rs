/// Represents an identifier, which is a name used to refer to types, fields, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier {
            name: name.to_string(),
        }
    }
}

/// Represents a type identifier, which can be a built-in type, a user-defined type,
/// a static array, or a dynamic array.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeIdentifier {
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    UnsignedInteger8,
    UnsignedInteger16,
    UnsignedInteger32,
    UnsignedInteger64,
    Float32,
    Float64,
    Bit,
    Byte,
    UserDefined(Identifier),
    StaticArray {
        r#type: Box<TypeIdentifier>,
        size: u64,
    },
    DynamicArray {
        r#type: Box<TypeIdentifier>,
    },
}

/// Represents a single field in an enumeration, which can either be a single value
/// or a range of values. Each field has a name and either a single value or a start
/// and end value for the range.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnumerationField {
    SingleValue {
        name: Identifier,
        value: u64,
    },
    RangeOfValues {
        name: Identifier,
        start: u64,
        end: u64,
    },
}

/// Represents an enumeration, which is a user-defined type that consists of
/// a set of named values, each of which can be a single value or a range of values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumerationDefinition {
    pub name: Identifier,
    pub fields: Vec<EnumerationField>,
}

/// Represents a single attribute of a field in a structure or union.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    DiscriminatedBy { field: Identifier },
    BitsSize { size: u64 },
    BytesSize { size: u64 },
}

/// Represents a single field in a structure, which consists of an attribute list, name and a type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructureField {
    pub name: Identifier,
    pub r#type: TypeIdentifier,
    pub attributes: Vec<Attribute>,
}

/// Represents a structure, which is a user-defined type that consists of
/// a collection of fields, each with a name and a type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructureDefinition {
    pub name: Identifier,
    pub fields: Vec<StructureField>,
}

/// Represents a single field in a union, which consists of a name, type, and
/// a discriminator value that identifies which type the field holds.
/// The discriminator is an integer value that is unique for each field in the union.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnionField {
    SingleValue {
        name: Identifier,
        r#type: TypeIdentifier,
        discriminator: u64,
    },
    RangeOfValues {
        name: Identifier,
        r#type: TypeIdentifier,
        start_discriminator: u64,
        end_discriminator: u64,
    },
}

/// Represents a union, which is a user-defined type that can hold one of several
/// values, each identified by a discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnionDefinition {
    pub name: Identifier,
    pub fields: Vec<UnionField>,
}

/// Represents a type definition, which is a user-defined type that can be
/// an alias for a built-in type or another user-defined type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDefinition {
    pub new_type: Identifier,
    pub r#type: TypeIdentifier,
}

/// Represents a single definition in the protocol, which can be an [`EnumerationDefinition`],
/// [`StructureDefinition`], [`UnionDefinition`], or [`TypeDefinition`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Definition {
    Enumeration(EnumerationDefinition),
    Structure(StructureDefinition),
    Union(UnionDefinition),
    Type(TypeDefinition),
}

/// Represents the entire protocol, which consists of multiple definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct Protocol {
    pub definitions: Vec<Definition>,
}

/// Extracts the name of a custom type identifier from a [`TypeIdentifier`].
/// If the type identifier is a user-defined type, it returns the name.
/// If it is a static or dynamic array, it recursively extracts the name from the contained type.
/// If it is a built-in type, it returns `None`.
fn extract_custom_type_identifier_name(type_identifier: &TypeIdentifier) -> Option<String> {
    match type_identifier {
        TypeIdentifier::UserDefined(id) => Some(id.name.clone()),
        TypeIdentifier::StaticArray { r#type, .. } => extract_custom_type_identifier_name(r#type),
        TypeIdentifier::DynamicArray { r#type } => extract_custom_type_identifier_name(r#type),
        _ => None,
    }
}

/// Extracts the names of all custom type identifiers from a structure definition.
fn extract_structure_subtypes(structure_def: &StructureDefinition) -> Vec<String> {
    structure_def
        .fields
        .iter()
        .filter_map(|field| extract_custom_type_identifier_name(&field.r#type))
        .collect()
}

/// Extracts the names of all custom type identifiers from a union definition.
fn extract_union_subtypes(union_def: &UnionDefinition) -> Vec<String> {
    union_def
        .fields
        .iter()
        .filter_map(|field| match field {
            UnionField::SingleValue { r#type, .. } => extract_custom_type_identifier_name(r#type),
            UnionField::RangeOfValues { r#type, .. } => extract_custom_type_identifier_name(r#type),
        })
        .collect()
}

/// Sorts the protocol definitions using their dependencies, meaning that if
/// a type `A` depends on type `B`, then `B` should appear before `A` in the sorted list.
/// This function returns a new `Protocol` with the definitions sorted accordingly.
/// If a circular dependency is detected, it returns an error.
pub(crate) fn sort_protocol_by_dependencies(protocol: &Protocol) -> Result<Protocol, String> {
    use std::collections::{HashMap, HashSet};

    let mut sorted_definitions = Vec::new();
    let mut visited = HashSet::new();
    let mut temp_mark = HashSet::new();

    fn visit(
        def: &Definition,
        visited: &mut HashSet<String>,
        temp_mark: &mut HashSet<String>,
        sorted_definitions: &mut Vec<Definition>,
        definitions_map: &HashMap<String, Definition>,
    ) -> Result<(), String> {
        let name = match def {
            Definition::Enumeration(enumeration_def) => enumeration_def.name.name.clone(),
            Definition::Structure(structure_def) => structure_def.name.name.clone(),
            Definition::Union(union_def) => union_def.name.name.clone(),
            Definition::Type(type_def) => type_def.new_type.name.clone(),
        };

        if temp_mark.contains(&name) {
            return Err(format!("Circular dependency detected for {name}"));
        }
        if visited.contains(&name) {
            return Ok(());
        }

        temp_mark.insert(name.clone());

        match def {
            Definition::Enumeration(_) => {}
            Definition::Structure(structure_def) => {
                for subtype in extract_structure_subtypes(structure_def) {
                    if let Some(subtype_def) = definitions_map.get(&subtype) {
                        visit(
                            subtype_def,
                            visited,
                            temp_mark,
                            sorted_definitions,
                            definitions_map,
                        )?;
                    }
                }
            }
            Definition::Union(union_def) => {
                for subtype in extract_union_subtypes(union_def) {
                    if let Some(subtype_def) = definitions_map.get(&subtype) {
                        visit(
                            subtype_def,
                            visited,
                            temp_mark,
                            sorted_definitions,
                            definitions_map,
                        )?;
                    }
                }
            }
            Definition::Type(type_def) => {
                if let Some(type_name) = extract_custom_type_identifier_name(&type_def.r#type) {
                    if let Some(type_def_ref) = definitions_map.get(&type_name) {
                        visit(
                            type_def_ref,
                            visited,
                            temp_mark,
                            sorted_definitions,
                            definitions_map,
                        )?;
                    }
                }
            }
        }

        temp_mark.remove(&name);
        visited.insert(name.clone());
        sorted_definitions.push(def.clone());

        Ok(())
    }

    let definitions_map: HashMap<String, Definition> = protocol
        .definitions
        .iter()
        .cloned()
        .map(|def| match def {
            Definition::Enumeration(enumeration_def) => (
                enumeration_def.name.name.clone(),
                Definition::Enumeration(enumeration_def),
            ),
            Definition::Structure(structure_def) => (
                structure_def.name.name.clone(),
                Definition::Structure(structure_def),
            ),
            Definition::Union(union_def) => {
                (union_def.name.name.clone(), Definition::Union(union_def))
            }
            Definition::Type(type_def) => {
                (type_def.new_type.name.clone(), Definition::Type(type_def))
            }
        })
        .collect();

    for def in &protocol.definitions {
        let name = match def {
            Definition::Enumeration(enumeration_def) => enumeration_def.name.name.clone(),
            Definition::Structure(structure_def) => structure_def.name.name.clone(),
            Definition::Union(union_def) => union_def.name.name.clone(),
            Definition::Type(type_def) => type_def.new_type.name.clone(),
        };
        if !visited.contains(&name) {
            visit(
                def,
                &mut visited,
                &mut temp_mark,
                &mut sorted_definitions,
                &definitions_map,
            )?;
        }
    }

    Ok(Protocol {
        definitions: sorted_definitions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_protocol_to_ast;
    use rstest::rstest;

    #[test]
    fn test_extract_custom_type_identifier_name_user_defined() {
        let type_id = TypeIdentifier::UserDefined(Identifier::new("CustomType"));
        assert_eq!(
            extract_custom_type_identifier_name(&type_id),
            Some("CustomType".to_string())
        );
    }

    #[test]
    fn test_extract_custom_type_identifier_name_static_array() {
        let type_id = TypeIdentifier::StaticArray {
            r#type: Box::new(TypeIdentifier::UserDefined(Identifier::new("CustomType"))),
            size: 10,
        };
        assert_eq!(
            extract_custom_type_identifier_name(&type_id),
            Some("CustomType".to_string())
        );
    }

    #[test]
    fn test_extract_custom_type_identifier_name_dynamic_array() {
        let type_id = TypeIdentifier::DynamicArray {
            r#type: Box::new(TypeIdentifier::UserDefined(Identifier::new("CustomType"))),
        };
        assert_eq!(
            extract_custom_type_identifier_name(&type_id),
            Some("CustomType".to_string())
        );
    }

    #[rstest]
    #[case(TypeIdentifier::Integer8)]
    #[case(TypeIdentifier::Integer16)]
    #[case(TypeIdentifier::Integer32)]
    #[case(TypeIdentifier::Integer64)]
    #[case(TypeIdentifier::UnsignedInteger8)]
    #[case(TypeIdentifier::UnsignedInteger16)]
    #[case(TypeIdentifier::UnsignedInteger32)]
    #[case(TypeIdentifier::UnsignedInteger64)]
    #[case(TypeIdentifier::Float32)]
    #[case(TypeIdentifier::Float64)]
    #[case(TypeIdentifier::Bit)]
    #[case(TypeIdentifier::Byte)]
    fn test_extract_custom_type_identifier_with_builtin_type(#[case] type_id: TypeIdentifier) {
        assert_eq!(extract_custom_type_identifier_name(&type_id), None);
    }

    #[test]
    fn test_extract_structure_subtypes() {
        let structure_def = StructureDefinition {
            name: Identifier::new("TestStructure"),
            fields: vec![
                StructureField {
                    name: Identifier::new("field1"),
                    r#type: TypeIdentifier::UserDefined(Identifier::new("SubType1")),
                    attributes: vec![],
                },
                StructureField {
                    name: Identifier::new("field2"),
                    r#type: TypeIdentifier::Integer32,
                    attributes: vec![],
                },
                StructureField {
                    name: Identifier::new("field3"),
                    r#type: TypeIdentifier::UserDefined(Identifier::new("SubType2")),
                    attributes: vec![],
                },
            ],
        };

        let subtypes = extract_structure_subtypes(&structure_def);
        assert_eq!(
            subtypes,
            vec!["SubType1".to_string(), "SubType2".to_string()]
        );
    }

    #[test]
    fn test_extract_union_subtypes() {
        let union_def = UnionDefinition {
            name: Identifier::new("TestUnion"),
            fields: vec![
                UnionField::SingleValue {
                    name: Identifier::new("field1"),
                    r#type: TypeIdentifier::UserDefined(Identifier::new("SubType1")),
                    discriminator: 0,
                },
                UnionField::RangeOfValues {
                    name: Identifier::new("field2"),
                    r#type: TypeIdentifier::UserDefined(Identifier::new("SubType2")),
                    start_discriminator: 1,
                    end_discriminator: 5,
                },
                UnionField::SingleValue {
                    name: Identifier::new("field3"),
                    r#type: TypeIdentifier::Integer32,
                    discriminator: 6,
                },
            ],
        };

        let subtypes = extract_union_subtypes(&union_def);
        assert_eq!(
            subtypes,
            vec!["SubType1".to_string(), "SubType2".to_string()]
        );
    }

    fn find_definition_index_by_name(protocol: &Protocol, name: &str) -> usize {
        protocol
            .definitions
            .iter()
            .position(|def| match def {
                Definition::Enumeration(enumeration_def) => enumeration_def.name.name == name,
                Definition::Structure(structure_def) => structure_def.name.name == name,
                Definition::Union(union_def) => union_def.name.name == name,
                Definition::Type(type_def) => type_def.new_type.name == name,
            })
            .expect("Definition not found")
    }

    fn assert_def_is_before_another_def(
        sorted_protocol: &Protocol,
        first_name: &str,
        second_name: &str,
    ) {
        let first_index = find_definition_index_by_name(sorted_protocol, first_name);
        let second_index = find_definition_index_by_name(sorted_protocol, second_name);

        assert!(
            first_index < second_index,
            "Expected {first_name} to be before {second_name}",
        );
    }

    #[test]
    fn test_sort_protocol_by_dependencies_with_ping_pong_example() {
        let ping_pong = include_str!("../examples/data/ping-pong.mek");
        let parsed = parse_protocol_to_ast(ping_pong).expect("Parsing failed");
        let sorted = sort_protocol_by_dependencies(&parsed);
        let sorted = sorted.expect("Sorting failed");

        assert_eq!(sorted.definitions.len(), 8);

        assert_def_is_before_another_def(&sorted, "IpAddress", "Ping");
        assert_def_is_before_another_def(&sorted, "DeviceName", "Pong");
        assert_def_is_before_another_def(&sorted, "DeviceStatus", "Pong");

        assert_def_is_before_another_def(&sorted, "Ping", "PingPong");
        assert_def_is_before_another_def(&sorted, "Pong", "PingPong");

        assert_def_is_before_another_def(&sorted, "MessageType", "Message");
        assert_def_is_before_another_def(&sorted, "PingPong", "Message");
    }

    #[test]
    fn test_sort_protocol_by_dependencies_with_circular_dependency() {
        let code = r#"
struct A {
    field1: B;
};

struct B {
    field1: A;
};
"#;
        let parsed = parse_protocol_to_ast(code).expect("Parsing failed");
        let sorted = sort_protocol_by_dependencies(&parsed);

        assert!(sorted.is_err(), "Failed to detect circular dependency");
        assert_eq!(sorted.err().unwrap(), "Circular dependency detected for A");
    }
}
