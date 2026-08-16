//! Shared helpers for compiling generated C test harnesses.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;

use meksmith::{
    check,
    smith::{CSmith, Smith},
};

pub fn cc_available() -> bool {
    Command::new("cc")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn protocol_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/protocols")
        .join(name)
}

pub fn generated_header_from_mek(fixture_name: &str) -> (String, String) {
    let path = protocol_fixture(fixture_name);
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    generated_header_from_source(&source)
}

pub fn generated_header_from_source(source: &str) -> (String, String) {
    let checked = check(source).expect("protocol fixture should analyze");
    let prefix = checked
        .file
        .protocol
        .as_ref()
        .map(|protocol| protocol.node.to_ascii_lowercase())
        .unwrap_or_else(|| "mek".into());
    let header = CSmith
        .generate(&checked)
        .expect("protocol fixture should emit C");
    (prefix, header)
}

pub fn harness_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/c")
}

pub fn compile_and_run(
    harness: &Path,
    header_prefix: &str,
    header: &str,
    fixture_include: Option<&Path>,
) {
    let dir = tempfile::tempdir().expect("tempdir");
    let header_name = format!("{header_prefix}.h");
    let header_path = dir.path().join(&header_name);
    std::fs::write(&header_path, header).expect("write generated header");

    let binary = dir
        .path()
        .join(harness.file_stem().expect("harness filename"));
    let mut command = Command::new("cc");
    command
        .arg(harness)
        .arg("-o")
        .arg(&binary)
        .arg("-I")
        .arg(dir.path());
    if let Some(fixture_include) = fixture_include {
        command.arg("-I").arg(fixture_include);
    }
    let status = command
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Werror")
        .status()
        .expect("compile generated C codecs");
    assert!(
        status.success(),
        "failed to compile {} against generated {header_name}",
        harness.display()
    );

    let run = Command::new(&binary)
        .status()
        .expect("run codec roundtrip binary");
    assert!(
        run.success(),
        "encode/decode assertions failed in {}",
        harness.display()
    );
}

pub fn compile_emit_main(
    header_prefix: &str,
    header: &str,
    include_line: &str,
    body: &str,
) -> Vec<u8> {
    let dir = tempfile::tempdir().expect("tempdir");
    let header_name = format!("{header_prefix}.h");
    let header_path = dir.path().join(&header_name);
    std::fs::write(&header_path, header).expect("write header");

    let main_src = format!(
        "#include <assert.h>\n#include <stdio.h>\n{include_line}\nint main(void) {{ {body} return 0; }}\n"
    );
    let main_path = dir.path().join("emit_main.c");
    std::fs::write(&main_path, main_src).expect("write emitter");

    let binary = dir.path().join("emit");
    let status = Command::new("cc")
        .arg(&main_path)
        .arg("-o")
        .arg(&binary)
        .arg("-I")
        .arg(dir.path())
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Werror")
        .status()
        .expect("compile emitter");
    assert!(status.success(), "failed to compile wire emitter");

    let output = Command::new(&binary).output().expect("run emitter");
    assert!(output.status.success(), "wire emitter failed");
    output.stdout
}
