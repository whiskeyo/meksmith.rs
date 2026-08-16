//! eCPRI remains an integration example — not the default smith codec test bed.
mod support;
use support::{cc_available, compile_and_run, generated_header_from_source, harness_dir};

#[test]
fn ecpri_analyzes_and_emits_c() {
    let source = include_str!("../examples/ecpri.mek");
    let (prefix, header) = generated_header_from_source(source);
    assert_eq!(prefix, "ecpri");
    assert!(header.contains("typedef struct"));
    assert!(header.contains("ecpri_message_encode"));
}

#[test]
fn ecpri_generated_header_compiles_and_decodes_known_header() {
    if !cc_available() {
        eprintln!(
            "skipping ecpri_generated_header_compiles_and_decodes_known_header: cc not found"
        );
        return;
    }

    let source = include_str!("../examples/ecpri.mek");
    let (prefix, header) = generated_header_from_source(source);
    let harness = harness_dir().join("ecpri_smoke.c");
    compile_and_run(&harness, &prefix, &header, None);
}

#[test]
fn ecpri_c_types_match_golden() {
    let source = include_str!("../examples/ecpri.mek");
    let golden = include_str!("golden/ecpri_types.h");
    let (_, output) = generated_header_from_source(source);

    assert_eq!(
        output, golden,
        "C output changed — re-run:\n  \
         cargo run -p meksmith --example emit_c -- meksmith/examples/ecpri.mek \
         > meksmith/tests/golden/ecpri_types.h"
    );
}

#[test]
fn golden_file_exists_on_disk() {
    let path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden/ecpri_types.h");
    assert!(path.is_file(), "missing golden file at {}", path.display());
}
