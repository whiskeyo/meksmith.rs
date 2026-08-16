//! Wire-byte fixture snapshots and C codec roundtrips for synthetic protocol fixtures.
use std::path::PathBuf;

mod support;
use support::{
    cc_available, compile_and_run, compile_emit_main, generated_header_from_mek, harness_dir,
};

fn fixture_dir(protocol: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(protocol)
}

fn parse_hex_fixture(text: &str) -> Vec<u8> {
    text.lines()
        .flat_map(|line| {
            let line = line.split('#').next().unwrap_or(line).trim();
            if line.is_empty() {
                return Vec::new();
            }
            line.split_whitespace()
                .map(|byte| u8::from_str_radix(byte, 16).expect("invalid hex fixture byte"))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn bitpack_header_codecs_roundtrip() {
    if !cc_available() {
        eprintln!("skipping bitpack_header_codecs_roundtrip: cc not found");
        return;
    }

    let (prefix, header) = generated_header_from_mek("bitpack_header.mek");
    let harness = harness_dir().join("bitpack_header_roundtrip.c");
    compile_and_run(&harness, &prefix, &header, Some(&fixture_dir("bitpack")));
}

#[test]
fn pc_seq_payload_codecs_roundtrip() {
    if !cc_available() {
        eprintln!("skipping pc_seq_payload_codecs_roundtrip: cc not found");
        return;
    }

    let (prefix, header) = generated_header_from_mek("pc_seq_payload.mek");
    let harness = harness_dir().join("pc_seq_payload_roundtrip.c");
    compile_and_run(&harness, &prefix, &header, Some(&fixture_dir("pc_seq")));
}

#[test]
fn choice_message_codecs_roundtrip() {
    if !cc_available() {
        eprintln!("skipping choice_message_codecs_roundtrip: cc not found");
        return;
    }

    let (prefix, header) = generated_header_from_mek("choice_message.mek");
    let harness = harness_dir().join("choice_message_roundtrip.c");
    compile_and_run(
        &harness,
        &prefix,
        &header,
        Some(&fixture_dir("choice_message")),
    );
}

#[test]
fn greedy_pdu_codecs_roundtrip() {
    if !cc_available() {
        eprintln!("skipping greedy_pdu_codecs_roundtrip: cc not found");
        return;
    }

    let (prefix, header) = generated_header_from_mek("greedy_pdu.mek");
    let harness = harness_dir().join("greedy_pdu_roundtrip.c");
    compile_and_run(&harness, &prefix, &header, Some(&fixture_dir("greedy_pdu")));
}

#[test]
fn bitpack_wire_fixture_hex_snapshots_match_generated_codecs() {
    if !cc_available() {
        eprintln!(
            "skipping bitpack_wire_fixture_hex_snapshots_match_generated_codecs: cc not found"
        );
        return;
    }

    let (prefix, header) = generated_header_from_mek("bitpack_header.mek");
    let include = format!("#include \"{prefix}.h\"");
    let expected = compile_emit_main(
        &prefix,
        &header,
        &include,
        "uint8_t wire[4]; size_t n; Header h={.version=Version_one,.reserved=0,.flag=0,.kind=Kind_alpha}; \
         assert(bitpackdemo_header_encode(&h,6,wire,4,&n)==0); fwrite(wire,1,n,stdout);",
    );

    let path = fixture_dir("bitpack").join("header_bs6.hex");
    let golden = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing fixture {}", path.display()));
    assert_eq!(
        expected,
        parse_hex_fixture(&golden),
        "fixture {} is stale",
        path.display()
    );
}

#[test]
fn pc_seq_wire_fixture_hex_snapshots_match_generated_codecs() {
    if !cc_available() {
        eprintln!(
            "skipping pc_seq_wire_fixture_hex_snapshots_match_generated_codecs: cc not found"
        );
        return;
    }

    let (prefix, header) = generated_header_from_mek("pc_seq_payload.mek");
    let include = format!("#include \"{prefix}.h\"");
    let expected = compile_emit_main(
        &prefix,
        &header,
        &include,
        "uint8_t s[]={0xde,0xad,0xbe,0xef}; PcSeqPayload p={.lane_id=0x1234,.part_id=0x5678,.bytes={.data=s,.len=4}}; \
         uint8_t wire[16]; size_t n; assert(pcseqdemo_pc_seq_payload_encode(&p,wire,16,&n)==0); fwrite(wire,1,n,stdout);",
    );

    let path = fixture_dir("pc_seq").join("four_bytes.hex");
    let golden = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing fixture {}", path.display()));
    assert_eq!(
        expected,
        parse_hex_fixture(&golden),
        "fixture {} is stale",
        path.display()
    );
}
