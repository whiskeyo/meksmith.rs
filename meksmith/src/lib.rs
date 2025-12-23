pub mod meklang;

mod ast;
mod parser;
pub mod smith_c;

use crate::ast::*;
use crate::parser::protocol;

use chumsky::Parser;

/// Based on the provided input, returns the line and column number of the error encountered during parsing.
fn get_error_location(input: &str, error: crate::parser::RichError) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;

    for (i, c) in input.char_indices() {
        if i >= error.span().start && i < error.span().end {
            return (line, column);
        }
        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    (line, column)
}

/// Parses a protocol from a string input and returns the resulting AST.
pub fn parse_protocol_to_ast(input: &str) -> Result<Protocol, String> {
    let result = protocol().parse(input);

    match result.into_result() {
        Ok(ast) => Ok(ast),
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .into_iter()
                .map(|e| {
                    let (line, column) = get_error_location(input, e.clone());
                    e.to_string()
                        + " in "
                        + line.to_string().as_str()
                        + ":"
                        + column.to_string().as_str()
                })
                .collect();
            Err(format!(
                "Parsing failed. Errors: {}",
                error_messages.join(", ")
            ))
        }
    }
}

/// Parses a protocol from a file and returns the resulting AST. Similar to `parse_protocol_to_ast`,
/// but reads the input from a file instead of a string.
pub fn parse_protocol_from_file_to_ast(file_path: &str) -> Result<Protocol, String> {
    let input =
        std::fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {e}"))?;
    parse_protocol_to_ast(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_protocol_to_ast() {
        let input = r#"
using MyType = int32[10];
        "#;

        let result = parse_protocol_to_ast(input);
        assert!(result.is_ok());
        let protocol = result.unwrap();
        assert_eq!(protocol.definitions.len(), 1);
        if let Definition::Type(type_def) = &protocol.definitions[0] {
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
    }

    #[test]
    fn test_parse_protocol_to_ast_with_errors() {
        let input = r#"
using MyType = int32[10;
        "#;

        let result = parse_protocol_to_ast(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Parsing failed. Errors: found ';' expected digit, or right bracket")
        );
    }

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
        assert!(result.is_ok());
        let protocol = result.unwrap();
        assert_eq!(protocol.definitions.len(), 1);
        if let Definition::Type(type_def) = &protocol.definitions[0] {
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
