mod ast;
mod parser;
pub mod smith;

use crate::ast::*;
use crate::parser::protocol;

use chumsky::Parser;

/// Parses a protocol from a string input and returns the resulting AST.
pub fn parse_protocol_to_ast(input: &str) -> Result<Protocol, String> {
    let result = protocol().parse(input);
    if result.has_errors() {
        Err("Parsing errors occurred".to_string())
    } else {
        Ok(result.into_output().unwrap())
    }
}

/// Parses a protocol from a file and returns the resulting AST. Similar to `parse_protocol_to_ast`,
/// but reads the input from a file instead of a string.
pub fn parse_protocol_from_file_to_ast(file_path: &str) -> Result<Protocol, String> {
    let input =
        std::fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    parse_protocol_to_ast(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_protocol_from_file_to_ast() {
        let file_path = "test_protocol.txt";
        if std::fs::exists(file_path).expect("Failure in checking file existence") {
            std::fs::remove_file(file_path).expect("Failure in removing existing file");
        }

        assert!(
            std::fs::write(
                file_path,
                r#"
using MyType = int32[10];
"#,
            )
            .is_ok()
        );
        let result = parse_protocol_from_file_to_ast(file_path);
        assert!(!result.is_err());
        let protocol = result.unwrap();
        assert_eq!(protocol.definitions.len(), 1);
        if let Definition::TypeDefinition(type_def) = &protocol.definitions[0] {
            assert_eq!(type_def.new_type.name, "MyType");
            assert_eq!(
                type_def.r#type,
                TypeIdentifier::StaticArray {
                    r#type: Box::new(TypeIdentifier::Integer32),
                    size: 10,
                }
            );
        } else {
            panic!("Expected a TypeDefinition");
        }
        std::fs::remove_file(file_path).expect("Failure in removing test file");
    }
}
