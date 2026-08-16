use std::fmt;

use chumsky::error::Rich;

use crate::frontend::span::SimpleSpan;
use crate::frontend::token::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostics {
    pub items: Vec<Diagnostic>,
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}

impl Diagnostics {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.items.push(diagnostic);
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn merge(&mut self, other: Diagnostics) {
        self.items.extend(other.items);
    }

    pub fn render(&self, source: &str, file_id: &str) -> String {
        let mut output = String::new();
        for diagnostic in &self.items {
            output.push_str(&diagnostic.render(source, file_id));
            output.push('\n');
        }
        output
    }

    /// Terminal-style rendering without ANSI color codes (for WASM / HTML surfaces).
    pub fn render_plain(&self, source: &str, file_id: &str) -> String {
        let config = ariadne::Config::default().with_color(false);
        let mut output = String::new();
        for diagnostic in &self.items {
            output.push_str(&diagnostic.render_with_config(source, file_id, config));
            output.push('\n');
        }
        output
    }
}

impl From<Diagnostic> for Diagnostics {
    fn from(diagnostic: Diagnostic) -> Self {
        Self {
            items: vec![diagnostic],
        }
    }
}

impl FromIterator<Diagnostic> for Diagnostics {
    fn from_iter<T: IntoIterator<Item = Diagnostic>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub message: String,
    pub span: SimpleSpan,
    pub labels: Vec<Label>,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn error(code: DiagnosticCode, span: SimpleSpan, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
        }
    }

    pub fn warning(code: DiagnosticCode, span: SimpleSpan, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
        }
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    pub fn render(&self, source: &str, file_id: &str) -> String {
        self.render_with_config(source, file_id, ariadne::Config::default())
    }

    pub fn render_plain(&self, source: &str, file_id: &str) -> String {
        self.render_with_config(
            source,
            file_id,
            ariadne::Config::default().with_color(false),
        )
    }

    fn render_with_config(&self, source: &str, file_id: &str, config: ariadne::Config) -> String {
        let span = (file_id, self.span.start..self.span.end);
        let mut report = ariadne::Report::build(
            match self.severity {
                Severity::Error => ariadne::ReportKind::Error,
                Severity::Warning => ariadne::ReportKind::Warning,
            },
            span,
        )
        .with_message(&self.message)
        .with_label(
            ariadne::Label::new((file_id, self.span.start..self.span.end))
                .with_message(self.code.as_str()),
        );

        for label in &self.labels {
            report = report.with_label(
                ariadne::Label::new((file_id, label.span.start..label.span.end))
                    .with_message(&label.message),
            );
        }

        if let Some(help) = &self.help {
            report = report.with_help(help);
        }

        report = report.with_config(config);

        let mut output = Vec::new();
        if report
            .finish()
            .write((file_id, ariadne::Source::from(source)), &mut output)
            .is_err()
        {
            return format!("{} at {}..{}", self.message, self.span.start, self.span.end);
        }
        String::from_utf8(output).unwrap_or_else(|_| self.message.clone())
    }

    pub fn from_rich(error: &Rich<'_, TokenKind, SimpleSpan>) -> Self {
        let span = error.span();
        let message = error.to_string();
        let (code, message) = classify_parse_error(&message);
        Self::error(code, SimpleSpan::new(span.start, span.end), message)
    }
}

fn classify_parse_error(message: &str) -> (DiagnosticCode, String) {
    if message.contains("end of input") {
        for (open, close) in [('{', '}'), ('(', ')'), ('[', ']'), ('<', '>')] {
            if message.contains(close) {
                return (
                    DiagnosticCode::E0002UnclosedDelimiter,
                    format!("unclosed `{open}`"),
                );
            }
        }
    }

    (DiagnosticCode::E0001Unexpected, message.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    E0001Unexpected,
    E0002UnclosedDelimiter,
    S0001DuplicateDefinition,
    S0002UnknownType,
    S0004UnknownVariant,
    S0101WidthExceedsType,
    S0102WidthRequired,
    S0104UnusedInvalid,
    S0106DuplicateField,
    S0110WidthNotInferable,
    S0108WidthNotInteger,
    S0304NonExhaustiveChoice,
    S0701UnknownAttribute,
    S0007CircularType,
    W0601UnusedParameter,
}

impl DiagnosticCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::E0001Unexpected => "E0001",
            Self::E0002UnclosedDelimiter => "E0002",
            Self::S0001DuplicateDefinition => "S0001",
            Self::S0002UnknownType => "S0002",
            Self::S0004UnknownVariant => "S0004",
            Self::S0101WidthExceedsType => "S0101",
            Self::S0102WidthRequired => "S0102",
            Self::S0104UnusedInvalid => "S0104",
            Self::S0106DuplicateField => "S0106",
            Self::S0110WidthNotInferable => "S0110",
            Self::S0108WidthNotInteger => "S0108",
            Self::S0304NonExhaustiveChoice => "S0304",
            Self::S0701UnknownAttribute => "S0701",
            Self::S0007CircularType => "S0007",
            Self::W0601UnusedParameter => "W0601",
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub span: SimpleSpan,
    pub message: String,
}

pub struct Emitter {
    diagnostics: Diagnostics,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            diagnostics: Diagnostics::new(),
        }
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn into_diagnostics(self) -> Diagnostics {
        self.diagnostics
    }

    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }
}

impl Default for Emitter {
    fn default() -> Self {
        Self::new()
    }
}
