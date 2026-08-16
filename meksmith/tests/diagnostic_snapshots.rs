//! Rendered diagnostic snapshots (ariadne output).

use meksmith::{DiagnosticCode, check};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/errors/{name}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap_or_else(|err| panic!("read fixture {name}: {err}"))
}

fn rendered_snapshot(source: &str, file_id: &str) -> String {
    let diagnostics = check(source).expect_err("expected check to fail");
    diagnostics.render(source, file_id)
}

#[test]
fn snapshot_duplicate_definition() {
    let source = fixture("duplicate_def.mek");
    insta::assert_snapshot!(
        "duplicate_def",
        rendered_snapshot(&source, "duplicate_def.mek")
    );
}

#[test]
fn snapshot_unknown_type() {
    let source = fixture("unknown_type.mek");
    insta::assert_snapshot!(
        "unknown_type",
        rendered_snapshot(&source, "unknown_type.mek")
    );
}

#[test]
fn snapshot_non_exhaustive_choice() {
    let source = fixture("non_exhaustive_choice.mek");
    insta::assert_snapshot!(
        "non_exhaustive_choice",
        rendered_snapshot(&source, "non_exhaustive_choice.mek")
    );
}

#[test]
fn snapshot_lexer_error() {
    let source = fixture("lex_error.mek");
    insta::assert_snapshot!("lex_error", rendered_snapshot(&source, "lex_error.mek"));
}

#[test]
fn snapshot_unclosed_brace() {
    let source = fixture("unclosed_brace.mek");
    insta::assert_snapshot!(
        "unclosed_brace",
        rendered_snapshot(&source, "unclosed_brace.mek")
    );
}

#[test]
fn diagnostic_codes_match_existing_tests() {
    let cases = [
        (
            "duplicate_def.mek",
            DiagnosticCode::S0001DuplicateDefinition,
        ),
        ("unknown_type.mek", DiagnosticCode::S0002UnknownType),
        (
            "non_exhaustive_choice.mek",
            DiagnosticCode::S0304NonExhaustiveChoice,
        ),
        ("lex_error.mek", DiagnosticCode::E0001Unexpected),
        ("unclosed_brace.mek", DiagnosticCode::E0002UnclosedDelimiter),
    ];

    for (fixture_name, code) in cases {
        let source = fixture(fixture_name);
        let diagnostics = check(&source).expect_err(fixture_name);
        assert!(
            diagnostics.items.iter().any(|item| item.code == code),
            "{fixture_name}: expected {code}"
        );
    }
}
