//! Parser frontend: lexing, parsing, and the meklang AST.

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod token;

pub use ast::File;
pub use error::{Error, Reason};
pub use lexer::{lex_source, lexer};
pub use parser::{parse, parse_source};
