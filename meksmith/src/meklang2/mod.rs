#![allow(unused)] // temporary, before everything is done

pub mod ast;
pub mod lexer;
pub mod parser;

use chumsky::Parser;

use crate::meklang2::ast::Module;
use crate::meklang2::parser::module::module;

pub(crate) type RichErr<'src> = chumsky::error::Rich<'src, char>;
pub(crate) type ErrType<'src> = chumsky::extra::Err<RichErr<'src>>;

pub(crate) type SimpleError<'src> = chumsky::error::Simple<'src, char>;
pub(crate) type SimpleErrorType<'src> = chumsky::extra::Err<SimpleError<'src>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputType {
    String { protocol: String },
    File { file_path: String },
}

pub fn parse_module(input: InputType) -> Result<Module, String> {
    let input_str = match input {
        InputType::String { protocol } => protocol.clone(),
        InputType::File { file_path } => match std::fs::read_to_string(file_path.clone()) {
            Ok(contents) => contents,
            Err(error) => return Err(format!("Failed to read file {file_path}: {error}")),
        },
    };

    let parsed_module = module().parse(input_str.as_str());
    match parsed_module.into_result() {
        Ok(module) => Ok(module),
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .into_iter()
                .map(|e| {
                    let (line, column) = get_error_location(&input_str, e.clone());
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

fn get_error_location(input: &str, error: RichErr) -> (usize, usize) {
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
