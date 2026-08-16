use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<T> {
    pub start: T,
    pub end: T,
}

impl<T> Span<T> {
    pub fn new(start: T, end: T) -> Self {
        Span { start, end }
    }
}

pub type SimpleSpan = Span<usize>;

impl fmt::Display for SimpleSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl chumsky::span::Span for SimpleSpan {
    type Context = ();
    type Offset = usize;

    fn new(_context: Self::Context, range: Range<Self::Offset>) -> Self {
        Span {
            start: range.start,
            end: range.end,
        }
    }

    fn context(&self) -> Self::Context {}

    fn start(&self) -> Self::Offset {
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spanned<T, S = SimpleSpan> {
    pub node: T,
    pub span: S,
}

impl<T, S> Spanned<T, S> {
    pub fn new(node: T, span: S) -> Self {
        Spanned { node, span }
    }

    pub fn node(&self) -> &T {
        &self.node
    }

    pub fn span(&self) -> &S {
        &self.span
    }
}
