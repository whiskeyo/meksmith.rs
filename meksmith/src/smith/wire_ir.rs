//! Wire-format intermediate representation shared by smith backends.
//!
//! Today this is a thin wrapper around the C layout lowering pass. A fuller IR
//! (padding nodes, nested arrays, choices) can grow here without touching the AST.

pub use super::c::layout::{StructureLayout, WireField};
