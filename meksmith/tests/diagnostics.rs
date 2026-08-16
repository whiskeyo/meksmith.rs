//! Diagnostic rendering and error-code coverage for invalid `.mek` fixtures.

use meksmith::{DiagnosticCode, check};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/errors/{name}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap_or_else(|err| panic!("read fixture {name}: {err}"))
}

fn assert_check_fails_with_code(source: &str, code: DiagnosticCode) {
    let diagnostics = check(source).expect_err("expected check to fail");
    assert!(
        diagnostics.has_errors(),
        "expected at least one error diagnostic"
    );
    assert!(
        diagnostics.items.iter().any(|item| item.code == code),
        "expected diagnostic code {code}, got: {:?}",
        diagnostics
            .items
            .iter()
            .map(|item| item.code)
            .collect::<Vec<_>>()
    );
}

fn assert_render_contains(source: &str, file_id: &str, needle: &str) {
    let diagnostics = check(source).expect_err("expected check to fail");
    let rendered = diagnostics.render(source, file_id);
    assert!(
        rendered.contains(needle),
        "expected rendered diagnostics to contain `{needle}`\n---\n{rendered}"
    );
}

#[test]
fn duplicate_definition_reports_s0001() {
    let source = fixture("duplicate_def.mek");
    assert_check_fails_with_code(&source, DiagnosticCode::S0001DuplicateDefinition);
    assert_render_contains(&source, "duplicate_def.mek", "S0001");
}

#[test]
fn unknown_type_reports_s0002() {
    let source = fixture("unknown_type.mek");
    assert_check_fails_with_code(&source, DiagnosticCode::S0002UnknownType);
    assert_render_contains(&source, "unknown_type.mek", "S0002");
}

#[test]
fn non_exhaustive_choice_reports_s0304() {
    let source = fixture("non_exhaustive_choice.mek");
    assert_check_fails_with_code(&source, DiagnosticCode::S0304NonExhaustiveChoice);
    assert_render_contains(&source, "non_exhaustive_choice.mek", "S0304");
    assert_render_contains(&source, "non_exhaustive_choice.mek", "beta");
}

#[test]
fn lexer_error_reports_e0001() {
    let source = fixture("lex_error.mek");
    assert_check_fails_with_code(&source, DiagnosticCode::E0001Unexpected);
    assert_render_contains(&source, "lex_error.mek", "E0001");
}

#[test]
fn plain_render_has_no_ansi_escape_codes() {
    let source = fixture("lex_error.mek");
    let diagnostics = check(&source).expect_err("expected check to fail");
    let rendered = diagnostics.render_plain(&source, "lex_error.mek");
    assert!(
        !rendered.contains('\u{1b}'),
        "plain render should not contain ANSI escapes:\n{rendered}"
    );
    assert!(rendered.contains("E0001"));
}

#[test]
fn unclosed_brace_reports_e0002() {
    let source = fixture("unclosed_brace.mek");
    assert_check_fails_with_code(&source, DiagnosticCode::E0002UnclosedDelimiter);
    assert_render_contains(&source, "unclosed_brace.mek", "E0002");
}

#[test]
fn ecpri_example_still_checks_cleanly() {
    let source = include_str!("../examples/ecpri.mek");
    check(source).expect("ecpri.mek should analyze cleanly");
}

#[test]
fn synthetic_protocol_fixtures_check_cleanly() {
    for name in [
        "bitpack_header.mek",
        "pc_seq_payload.mek",
        "choice_message.mek",
        "greedy_pdu.mek",
    ] {
        let path = format!("{}/tests/protocols/{name}", env!("CARGO_MANIFEST_DIR"));
        let source = std::fs::read_to_string(&path).expect("read protocol fixture");
        check(&source).unwrap_or_else(|_| panic!("{name} should analyze cleanly"));
    }
}
