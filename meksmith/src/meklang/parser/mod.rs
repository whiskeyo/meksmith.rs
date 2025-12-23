#![allow(unused)] // temporary, before everything is done

pub mod tokens;

pub mod attributes;
pub mod base;
pub mod types;

pub mod enumerations;
pub mod structures;
pub mod unions;

pub mod definitions;
pub mod module;

pub(crate) type RichErr<'src> = chumsky::error::Rich<'src, char>;
pub(crate) type ErrType<'src> = chumsky::extra::Err<RichErr<'src>>;
