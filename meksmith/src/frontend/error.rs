use chumsky::error::{Rich, Simple};

use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::frontend::span::SimpleSpan;

use super::{span::Span, token::TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub span: Option<Span<usize>>,
    pub reason: Reason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    Unexpected { found: String },
    UnclosedDelimiter { delimiter: char },
}

impl Error {
    pub fn from_simple(source: &str, error: &Simple<'_, char>) -> Self {
        let span = Span::new(error.span().start, error.span().end);
        let found = token_text(source, span);

        if found.is_empty()
            && let Some((delimiter, _)) = unclosed_delimiter(source, span.start)
        {
            return Error {
                span: Some(span),
                reason: Reason::UnclosedDelimiter { delimiter },
            };
        }

        Error {
            span: Some(span),
            reason: Reason::Unexpected { found },
        }
    }

    pub fn from_diagnostic(diagnostic: Diagnostic) -> Self {
        Error {
            span: Some(Span::new(diagnostic.span.start, diagnostic.span.end)),
            reason: Reason::Unexpected {
                found: diagnostic.message,
            },
        }
    }

    pub fn from_rich(error: &Rich<'_, TokenKind, SimpleSpan>) -> Self {
        Self::from_diagnostic(Diagnostic::from_rich(error))
    }

    pub fn message(&self) -> String {
        match &self.reason {
            Reason::Unexpected { found } if found.is_empty() => "unexpected end of input".into(),
            Reason::Unexpected { found } => format!("unexpected token `{found}`"),
            Reason::UnclosedDelimiter { delimiter } => {
                format!("unclosed `{delimiter}`")
            }
        }
    }

    pub fn code(&self) -> DiagnosticCode {
        match self.reason {
            Reason::UnclosedDelimiter { .. } => DiagnosticCode::E0002UnclosedDelimiter,
            Reason::Unexpected { .. } => DiagnosticCode::E0001Unexpected,
        }
    }
}

impl From<Error> for Diagnostic {
    fn from(error: Error) -> Self {
        let span = error
            .span
            .map(|s| SimpleSpan::new(s.start, s.end))
            .unwrap_or(SimpleSpan::new(0, 0));
        Diagnostic::error(error.code(), span, error.message())
    }
}

fn token_text(source: &str, span: Span<usize>) -> String {
    let char_start = source[..span.start].chars().count();
    let char_end = source[..span.end].chars().count();
    source
        .chars()
        .skip(char_start)
        .take(char_end.saturating_sub(char_start))
        .collect()
}

fn unclosed_delimiter(source: &str, error_start: usize) -> Option<(char, usize)> {
    let before = &source[..error_start.min(source.len())];
    let pairs = [('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')];
    let mut stack = Vec::new();

    for ch in before.chars() {
        if let Some((_, open)) = pairs.iter().find(|(open, _)| *open == ch) {
            stack.push(*open);
        } else if let Some((open, _)) = pairs.iter().find(|(_, close)| *close == ch)
            && stack.last() == Some(open)
        {
            stack.pop();
        }
    }

    stack
        .last()
        .copied()
        .map(|delimiter| (delimiter, stack.len()))
}
