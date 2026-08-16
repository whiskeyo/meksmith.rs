use meksmith::{CppSmith, Smith, check, smith_cpp};

#[test]
fn cpp_backend_emits_types_and_codecs_for_ecpri() {
    let source = include_str!("../examples/ecpri.mek");
    let checked = check(source).expect("ecpri.mek should analyze cleanly");
    let output = CppSmith
        .generate(&checked)
        .expect("ecpri.mek should emit C++");

    assert!(output.contains("#include <version>"));
    assert!(output.contains("__cpp_lib_expected"));
    assert!(output.contains("namespace ecpri"));
    assert!(output.contains("enum class MessageType"));
    assert!(output.contains("std::vector<"));
    assert!(output.contains("std::variant<"));
    assert!(output.contains("std::expected<std::size_t, int>"));
    assert!(output.contains("header_encode"));
    assert!(output.contains("remote_memory_access_encode"));
    assert!(output.contains("event_indication_encode"));
    assert!(output.contains("pdu_encode"));
}

#[test]
fn cpp_backend_emits_bitpack_fixture() {
    let path = format!(
        "{}/tests/protocols/bitpack_header.mek",
        env!("CARGO_MANIFEST_DIR")
    );
    let source = std::fs::read_to_string(&path).expect("read bitpack fixture");
    let output = smith_cpp::generate_cpp_code_from_string(&source).expect("emit C++");
    assert!(output.contains("namespace bitpackdemo"));
    assert!(output.contains("header_encode"));
    assert!(output.contains("std::span<std::uint8_t>"));
}

#[test]
fn cpp_generated_bitpack_compiles_with_gxx23() {
    if !gxx_available() {
        eprintln!("skipping cpp_generated_bitpack_compiles_with_gxx23: g++ not found");
        return;
    }

    let path = format!(
        "{}/tests/protocols/bitpack_header.mek",
        env!("CARGO_MANIFEST_DIR")
    );
    let source = std::fs::read_to_string(&path).expect("read bitpack fixture");
    let header = smith_cpp::generate_cpp_code_from_string(&source).expect("emit C++");

    let dir = tempfile::tempdir().expect("tempdir");
    let header_path = dir.path().join("bitpackdemo.hpp");
    std::fs::write(&header_path, &header).expect("write header");

    let main_src = r#"
#include "bitpackdemo.hpp"
#include <cassert>
#include <cstdint>
#include <vector>

int main() {
    bitpackdemo::Header in{};
    in.version = bitpackdemo::Version::one;
    in.flag = true;
    in.kind = bitpackdemo::Kind::alpha;
    std::vector<std::uint8_t> wire(4);
    auto encoded = bitpackdemo::header_encode(in, 6, std::span<std::uint8_t>(wire));
    assert(encoded);
    assert(*encoded == 4);
    bitpackdemo::Header out{};
    std::uint16_t body_size = 0;
    auto decoded = bitpackdemo::header_decode(std::span<const std::uint8_t>(wire), body_size, out);
    assert(decoded);
    assert(body_size == 6);
    assert(out.version == in.version);
    assert(out.kind == in.kind);
    return 0;
}
"#;
    let main_path = dir.path().join("main.cpp");
    std::fs::write(&main_path, main_src).expect("write main");

    let binary = dir.path().join("roundtrip");
    let status = std::process::Command::new("g++")
        .arg("-std=c++23")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Werror")
        .arg(&main_path)
        .arg("-I")
        .arg(dir.path())
        .arg("-o")
        .arg(&binary)
        .status()
        .expect("compile C++ roundtrip");
    assert!(status.success(), "failed to compile generated C++ header");

    let run = std::process::Command::new(&binary)
        .status()
        .expect("run binary");
    assert!(run.success());
}

fn gxx_available() -> bool {
    std::process::Command::new("g++")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
