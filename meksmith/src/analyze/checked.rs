use crate::analyze::{DependencyGraph, SymbolTable};
use crate::frontend::ast::File;

#[derive(Debug, Clone)]
pub struct CheckedFile {
    pub file: File,
    pub symbols: SymbolTable,
    pub emit_order: Vec<String>,
    pub graph: DependencyGraph,
}
