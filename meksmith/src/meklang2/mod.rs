#![allow(unused)] // temporary, before everything is done

pub mod ast;
pub mod parser;

pub(crate) type RichErr<'src> = chumsky::error::Rich<'src, char>;
pub(crate) type ErrType<'src> = chumsky::extra::Err<RichErr<'src>>;
