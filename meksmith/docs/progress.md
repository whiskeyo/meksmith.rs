# meksmith implementation progress

Living checklist for compiler milestones. Updated 2026-08-16.

See also [implementation-plan.md](./implementation-plan.md) for architecture and design decisions.

---

## Summary (recent work)

- **M0–M4** — Parser frontend, diagnostics, analyzer, dependency graph, C type emission
- **M5** — Pattern-based composite codecs + packed-prefix wire-tail layout (`RemoteMemoryAccess`, `EventIndication`)
- **Test refactor** — Synthetic protocol fixtures under `tests/protocols/`; eCPRI is integration-only
- **CLI** — `meksmith check` and `meksmith emit -t c|cpp [-o out.h]`
- **Diagnostics** — Structured codes, insta snapshot tests, choice exhaustiveness fix

---

## Milestones

### M0 — Parser bootstrap
- [x] `frontend/` module with chumsky lexer/parser
- [x] `examples/ecpri.mek` parses

### M1 — AST spans + diagnostics
- [x] Spans on AST nodes (`Spanned<T>`)
- [x] Unified `Diagnostic` / `Diagnostics` type
- [x] ariadne rendering (`Diagnostic::render`)
- [x] Lexer errors `E0001` / `E0002`
- [x] Parser error classification (`classify_parse_error`)
- [x] Snapshot tests for error messages (`tests/diagnostic_snapshots.rs` + insta)

### M2 — Semantic analysis
- [x] Symbol table + type resolution
- [x] Core semantic checks (`S0001`–`S0109` range)
- [x] Choice exhaustiveness (`S0304`) — includes uncovered enum variants
- [x] `expr_mentions_ident()` / `field_mentions_param()` for unused-parameter warnings
- [x] Const expression evaluator (`analyze/const_expr.rs`) for `param ± N`, `(param - N) / M`

### M3 — Dependency graph
- [x] `analyze/graph.rs` topological sort
- [x] `CheckedFile` export

### M4 — C type emission
- [x] `smith` trait + `CSmith`
- [x] Structures, enums, type aliases, choices (tagged union)
- [x] `DynamicArray<T,N>` → `{ T *data; size_t len; }`
- [x] `[computed]` fields omitted from user structs
- [x] Sub-byte fields via mask/shift (`layout.rs`)

### M5 — C encode/decode
- [x] Scalar / leaf structure codecs (`codec.rs` + `layout.rs`)
- [x] Parametric blob: `prefix… + DynamicArray<byte, param - N>`
- [x] Parametric opaque: `DynamicArray<byte, param>`
- [x] Length-prefixed: `u16 len + DynamicArray<byte, len>` (`Frame`)
- [x] Message wrapper: `header + Choice(disc, size)`
- [x] Generic `Choice` encode/decode
- [x] `PduMessage` + `pad_to_align_if` padding (pattern-matched)
- [x] `GreedyArray<T>` root encode/decode loops
- [x] Pattern detection in `patterns.rs`
- [x] 32-bit scalar prefix fields (`GenericDataTransfer`)
- [x] Nested structure prefix fields (`OneWayDelayMeasurement` + `PtpTimestamp`)
- [x] Packed prefix via wire layout (`RemoteMemoryAccess`)
- [x] `DynamicArray<Struct,N>` tail (`EventIndication` + `FaultNotificationRecord`)
- [x] Multi-byte bitfield encode/decode (12-bit fields, etc.)
- [x] Wire-format IR entry point (`smith/wire_ir.rs` re-exports `StructureLayout`)
- [ ] General `size()` call evaluation in arbitrary expressions (padding uses fixed patterns today)

### M6 — C++ + website
- [x] C++ backend (`CppSmith` — C++23, `enum class`, `std::vector`, `std::variant`, `std::expected`, `std::span`)
- [x] C++ encode/decode emission (layout + composite codecs)
- [x] `smith_cpp::generate_cpp_code_from_string` wrapper
- [x] Website code generator wired to `smith_c::generate_c_code_from_string` (ariadne errors)
- [x] Website C++ target toggle / `smith_cpp` integration in editor
- [ ] Website WASM-targeted diagnostic renderer (shared `Diagnostic` struct, lighter than full ariadne)

---

## Test architecture

- [x] Synthetic fixtures: `bitpack_header.mek`, `pc_seq_payload.mek`, `choice_message.mek`, `greedy_pdu.mek`
- [x] Per-fixture C harnesses in `tests/c/`
- [x] `codec_roundtrip.rs` — hex snapshots + C roundtrip per synthetic protocol
- [x] `ecpri_integration.rs` — analyze/emit smoke, golden header compare, compile smoke
- [x] `diagnostics.rs` — error-code fixtures
- [x] `diagnostic_snapshots.rs` — insta snapshots of rendered diagnostics
- [x] `cpp_emit.rs` — C++ type + codec emission smoke + g++-23 compile test

---

## Tooling

- [x] `meksmith check <file.mek>`
- [x] `meksmith emit -t c|cpp [-o out.h] <file.mek>`
- [x] `-o` write path verified via `cargo run -- emit -t c -o …`

---

## eCPRI payload codec status

| Payload | Status |
|---------|--------|
| `IQData` | Done (parametric blob) |
| `BitSequence` | Done (parametric blob) |
| `RealTimeControlData` | Done (parametric blob) |
| `GenericDataTransfer` | Done (32-bit prefix fields) |
| `RemoteReset` | Done (parametric blob) |
| `OneWayDelayMeasurement` | Done (nested `PtpTimestamp` prefix) |
| `RemoteMemoryAccess` | Done (packed wire-layout prefix) |
| `EventIndication` | Done (struct array tail) |
| `OpaquePayload` | Done (parametric opaque) |

---

## Remaining / future

1. General `size(x)` expression evaluation in analyzer + smith
2. ~~Website C++ output toggle~~ (done)
4. LSP + lighter WASM diagnostic renderer for website
5. Fuzzing round-trip harness
