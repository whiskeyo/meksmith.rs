# meksmith implementation plan

Living document for the compiler pipeline after the parser frontend reorganization.

**Current layout**

```
meksmith/src/
  lib.rs                 # pub mod frontend; pub use frontend::parse_source;
  frontend/
    mod.rs               # public API of the parser frontend
    ast.rs
    span.rs
    token.rs
    error.rs
    lexer.rs
    parser/
      mod.rs
      common.rs
```

**Planned layout**

```
meksmith/src/
  lib.rs
  frontend/              # (1) lex + parse → raw AST
  analyze/               # (2) semantic analysis → typed IR
  smith/                 # (3) code generation trait + backends
    mod.rs               # Smith trait, DependencyGraph, emit order
    c/
    cpp/
```

---

## 1. Parser frontend — keep together, improve errors

### Goals

- Single entry point: `frontend::parse(&str) -> Result<SpannedFile, Diagnostics>`
- Spans on every AST node (today only tokens carry spans; AST nodes do not)
- Rich, IDE-friendly diagnostics (label + help text + source context) via **ariadne** (native + WASM renderer sharing the same `Diagnostic` type)

> **Resolved — `File` vs plaintext** (see [Design FAQ](#design-faq) § “What does `parse` return?”)

### Implementation steps

| Step | Task | Notes |
|------|------|-------|
| 1.1 | Wrap `File` in `Spanned<File>` or add `span` to each AST node | Needed for analyzer and error reporting |
| 1.2 | Unify `frontend::Error` into `Diagnostics` / `Diagnostic` | One type for lex + parse + analyze later |
| 1.3 | Map chumsky `Rich` errors to structured `Diagnostic` | `expected`, `found`, `label`, `help` |
| 1.4 | Add `Diagnostic::render(&source) -> String` via **ariadne** | Same `Diagnostic` struct for CLI and `website/` (custom or simplified renderer over the same data) |
| 1.5 | Export `frontend::parse(&str) -> Result<Spanned<File>, Diagnostics>` | Keep `parse_source` as alias during migration |
| 1.6 | Snapshot tests for error messages | `trybuild` or insta fixtures in `frontend/tests/` |

### Planned diagnostic shape

```rust
pub struct Diagnostic {
    pub severity: Severity,           // Error | Warning
    pub code: DiagnosticCode,         // e.g. E0001 UnexpectedToken
    pub message: String,
    pub span: Span<usize>,
    pub labels: Vec<Label>,           // primary + secondary spans
    pub help: Option<String>,
}
```

### Parser error codes (frontend only)

| Code | When |
|------|------|
| `E0001` | Unexpected token / character |
| `E0002` | Unclosed delimiter (`{`, `(`, `<`, …) |
| `E0003` | Invalid numeric literal |
| `E0004` | Expected `;` after `protocol` declaration |
| `E0005` | Empty `structure` / `enumerated` / `choice` body |
| `E0006` | Duplicate field name in same structure |
| `E0007` | Invalid attribute syntax (future: when attributes are structured) |

---

## 2. AST analyzer (semantic checker)

### Goals

- Build a **symbol table** of all definitions (`structure`, `enumerated`, `choice`, `type`)
- Resolve types and compute **wire sizes** for every field
- Reject ill-formed protocols before code generation
- Output: `analyze::CheckedFile` (raw AST + resolved types + dependency graph)

### Pipeline

```
File (raw AST)
  → collect definitions (symbol table)
  → resolve names & type aliases
  → check expressions (const-eval where possible)
  → check field widths vs underlying types
  → check choice exhaustiveness / overlaps
  → check attributes ([computed], [unused], …)
  → build dependency graph
  → CheckedFile
```

> **Resolved — attributes** (see [Design FAQ](#design-faq) § “Which attributes?”)

### Symbol table entry

```rust
pub enum DefKind {
    Structure { params: Vec<Param>, fields: Vec<ResolvedField>, bit_order: BitOrder },
    Enumerated { width_bits: u32, variants: Vec<ResolvedVariant> },
    Choice { params: Vec<Param>, discriminant: TypeId, arms: Vec<ResolvedArm> },
    TypeAlias { wire_bits: Option<u32>, underlying: TypeId },
    Builtin(BuiltinType),
}
```

### Builtin types (initial set)

| Name | Wire bits | Host mapping (C) |
|------|-----------|------------------|
| `u8` … `u64`, `i8` … `i64` | 8×N | `uint8_t` … |
| `boolean` | 1 | `bool` or `uint8_t` |
| `byte` | 8 | `uint8_t` |
| `null` | *field width* | no storage / padding only |

### Builtin generic types (smith interprets, analyzer validates)

| Name | Arity | Role |
|------|-------|------|
| `DynamicArray<T, LenExpr>` | 2 | `LenExpr` must be ≥ 0 when evaluable |
| `GreedyArray<T>` | 1 | consume until EOF |
| `size(x)` | expr | encoded byte length |
| `pad_to_align_if(A, L, cond)` | expr | conditional padding |

> **Resolved — generic types** (see [Design FAQ](#design-faq) § “Are builtins enough?”)

Planned additions (not builtins on day one, add when a real protocol needs them):

| Name | Role |
|------|------|
| `StaticArray<T, N>` | fixed-length array (`N` const) |
| `Optional<T, when>` | presence bit or `when` expression |
| `BitSlice<N>` | sub-byte view inside a scalar host type |

---

### Semantic errors (full list)

#### Definitions & names

| Code | Error |
|------|-------|
| `S0001` | Duplicate definition: `{name}` already defined |
| `S0002` | Unknown type: `{name}` |
| `S0003` | Unknown identifier in expression: `{name}` |
| `S0004` | Unknown variant: `{Type}::{variant}` |
| `S0005` | Unknown field: `{type}.{field}` |
| `S0006` | Forward reference to `{name}` where not allowed |
| `S0007` | Circular type alias: `{a}` → `{b}` → `{a}` |

#### Structure fields

| Code | Error |
|------|-------|
| `S0101` | Field `{name}`: wire width ({N} bits) exceeds underlying type capacity ({M} bits) |
| `S0102` | Field `{name}`: underlying type `{ty}` has no fixed width; explicit `(N bits)` required |
| `S0103` | Field `{name}`: `null` field must have explicit bit width |
| `S0104` | Field `{name}`: `[unused]` field must be `null` or marked padding |
| `S0105` | Field `{name}`: `[computed]` field must not appear in user encode API |
| `S0110` | Field `{name}`: no explicit width and type `{ty}` has no inferable wire width — add `(N bits)` or use a sized alias |
| `S0106` | Duplicate field name `{name}` in structure `{struct}` |
| `S0107` | Structure `{name}`: total bit length not a multiple of 8 — **allowed**; smith emits sub-byte packing (see [Recorded decisions](#recorded-decisions)) |
| `S0108` | Field `{name}`: non-integer bit width expression |
| `S0109` | Field `{name}`: bit width expression negative or zero |

#### Enumerations

| Code | Error |
|------|-------|
| `S0201` | Variant `{v}` value out of range for {N}-bit enum (max {max}) |
| `S0202` | Overlapping variant ranges: `{a}` and `{b}` |
| `S0203` | Gap in enum coverage (warning `W0201` if intentional) |
| `S0204` | Empty enumeration `{name}` |
| `S0205` | Enum `{name}`: variant value not representable in {N} bits |

#### Choices

| Code | Error |
|------|-------|
| `S0301` | Choice `{name}`: discriminant `{field}` not found in scope |
| `S0302` | Choice `{name}`: discriminant type must be integer or enumeration |
| `S0303` | Overlapping choice arms for value/range |
| `S0304` | Non-exhaustive choice — **error** unless `_ => opaque` / `_ => OpaquePayload(…)` catch-all is present |
| `S0305` | Choice arm references unknown type `{ty}` |
| `S0306` | Choice parameter `{param}` unused in arm types (**see Q2**) |

#### Type aliases & generics

| Code | Error |
|------|-------|
| `S0401` | Type alias `{name}`: wire width ({N}) incompatible with underlying `{ty}` ({M} bits) |
| `S0402` | Unknown generic type: `{name}<…>` |
| `S0403` | Wrong arity: `{name}` expects {N} generic parameters, found {M} |
| `S0404` | Type argument to `{name}(…)` does not match parameter `{param}` |

#### Expressions

| Code | Error |
|------|-------|
| `S0501` | Division by zero in constant expression |
| `S0502` | Expression refers to field not yet defined (ordering) |
| `S0503` | `size(x)`: argument must be a typed value, not a bare identifier |
| `S0504` | Non-integer expression where integer required |
| `S0505` | Unknown call: `{name}(…)` is not a builtin |
| `S0506` | Wrong arity for `{name}(…)` |
| `S0507` | Expression uses structure parameter not in scope |

#### Parameters

| Code | Error |
|------|-------|
| `S0601` | Unused structure parameter `{param}` — **warning** `W0601` (see [Recorded decisions](#recorded-decisions)) |
| `S0602` | Parameter `{param}` type `{ty}` unresolved |
| `S0603` | Argument passed to `{Struct}(…)` missing parameter `{param}` |

#### Attributes

| Code | Error |
|------|-------|
| `S0701` | Unknown attribute `{attr}` |
| `S0702` | Conflicting attributes on field `{name}` |
| `S0703` | `[computed]` on field that the codec cannot derive (no rule / expression) |

#### Protocol / PDU

| Code | Error |
|------|-------|
| `S0801` | Multiple `protocol` declarations |
| `S0802` | `GreedyArray` only allowed as last field of root PDU type |
| `S0803` | PDU last message must have `concatenation = false` (encode-time check) |

---

## 3. Smith trait & dependency ordering

### Goals

- Every backend (C, C++, Rust, …) implements one `Smith` trait
- Emit order guarantees: dependencies before dependents (valid C/C++ forward declarations minimized)
- Shared encode/decode logic where possible; language-specific only for syntax

### Proposed `Smith` trait

```rust
pub trait Smith {
    type Output; // e.g. String, or SmithArtifact { header, source }

    fn name(&self) -> &'static str;

    /// Analyze already-checked file; return diagnostics on failure.
    fn generate(&self, file: &CheckedFile) -> Result<Self::Output, Diagnostics>;

    /// Optional: generate only types (no encode/decode), for header-only mode.
    fn generate_types(&self, file: &CheckedFile) -> Result<String, Diagnostics> {
        self.generate(file).map(/* extract types */)
    }
}
```

### Internal smith pipeline (shared by all backends)

```
CheckedFile
  → topological sort (dependency order)
  → for each Def in order:
       emit type definition
       emit encode function
       emit decode function
  → emit builtins (DynamicArray helpers, etc.)
  → bundle output
```

### Dependency graph

**Nodes**: each user-defined type (`Structure`, `Enumerated`, `Choice`, `TypeAlias`).

**Edges**: `A → B` if `A` directly references `B` (field type, choice arm, alias underlying, generic arg).

**Algorithm**: Kahn topological sort on DAG. Cycles reported as `S0007` / structure recursion errors.

**Tie-breaking** (stable, deterministic output):

1. Enumerations before structures that use them
2. Leaf types (aliases to builtins) before composites
3. Lexicographic by name within same depth

### C / C++ emission order example

For `ecpri.mek`:

```
ProtocolRevision, MessageType, RMAReadWrite, …   (enums)
PC_ID, SEQ_ID, …                                   (type aliases)
PtpTimestamp, FaultNotificationRecord, …           (leaf structures)
IQData, BitSequence, …                             (payload structures)
Payload                                            (choice → tagged union)
Header, Message, PduMessage, Pdu                   (outer wrappers)
```

**C mapping**

| meklang | C |
|---------|---|
| `structure` | `struct` with portable mask/shift encode/decode (host type may be wider than wire width) |
| `enumerated(N bits)` | `enum` or `uintN_t` + `#define` constants |
| `choice` | `struct { MessageType tag; union { … } u; }` |
| `DynamicArray<T,N>` | `struct { T *data; size_t len; }` or inline flexible array member |
| `[computed]` fields | omitted from **user** struct; filled only on the wire path (see FAQ) |

**C++ mapping**: same order; use `enum class`, `std::variant` for choices, `std::vector` for dynamic arrays.

### Recursion & forward declarations

| Case | Strategy |
|------|----------|
| `A` contains `B`, `B` contains `A` | **Reject** at analyze (unless explicit `forward` / opaque pointer — future) |
| Choice arm references sibling structure | OK if arm type defined earlier in sort order |
| Self-referential PDU (`GreedyArray`) | `Pdu` is root only; no nested greedy |

If sort reveals a cycle, emit error with cycle path: `Message → Payload → IQData → Message`.

---

## 4. Other improvements

| Priority | Improvement | Status |
|----------|-------------|--------|
| High | **Spans on AST** — prerequisite for good errors everywhere | done |
| High | **`meksmith` CLI** — `meksmith check file.mek`, `meksmith emit -t c file.mek` | done |
| High | **Golden tests** — parse + analyze + emit C for `examples/ecpri.mek` | done |
| High | **Synthetic protocol fixtures** — smith tests decoupled from eCPRI | done ([progress.md](./progress.md)) |
| Medium | **Structured attributes** — `[computed]`, `[unused]` as enum not `String` | partial |
| Medium | **`include` / multi-file** — merge ASTs before analyze | not started |
| Medium | **Const evaluator** for expressions (`payload_size - 4`, etc.) | not started |
| Medium | **Wire-format IR** — lower AST to flat bit/byte layout before smith | not started |
| Low | **LSP / diagnostics in website** editor | not started |
| Low | **Fuzzing** — round-trip encode/decode random valid messages | not started |

### Wire-format IR (recommended middle layer)

Between analyze and smith, lower each structure to:

```rust
pub enum WireField {
    Padding { bits: u32 },
    Scalar { name: String, bits: u32, ty: Builtin },
    Array { name: String, elem: Builtin, len: Len },
    Nested { name: String, ty: TypeId },
    Choice { … },
}
```

Smiths become simpler: they print IR, not raw AST expressions.

---

## Recorded decisions

Decisions from Q1–Q7 (2026-08-16).

| Topic | Decision |
|-------|----------|
| **Q1 Choice exhaustiveness** | **Error** (`S0304`) if no catch-all. Catch-all syntax: `_ => OpaquePayload(payload_size)` or bare `_ => opaque` (sugar). Smith lowers to a **union arm with byte buffer** — no runtime error path in C. |
| **Q2 Unused parameters** | **Warning** (`W0601`), not error — unused params are often kept for API symmetry across payload types (e.g. all arms take `payload_size`). Promote to error only with `#![deny(unused_params)]` or strict mode. |
| **Q3 Non-byte-aligned layouts** | **Allowed.** Analyzer computes exact bit layout; smith emits mask/shift (no reliance on C bitfields). |
| **Q4 Host vs wire width** | Host type (e.g. `u32`) may be **wider** than wire width (e.g. `22 bits`). Encode writes only the low `N` bits; decode reads `N` bits into the host type. Layout metadata drives both struct definition and codec. |
| **Q5 Codec-filled fields** | **`[computed]`** — omit from user struct; codec sets value (e.g. `payload_size`). Not `[derived]`. |
| **Q5b Wire width** | **Implicit from type** when `(N bits)` omitted — no attribute needed (see FAQ). |
| **Q6 Codegen root** | **No single root type.** Emit all definitions; user calls `ecpri_pdu_decode`, `ecpri_message_decode`, etc. CLI `--root` is optional convenience only. |
| **Q1b Choice catch-all** | **Opaque bytes** default. Decode always succeeds structurally; unknown/discarded variants land in `{ uint8_t *data; size_t len; }`. Optional `error("…")` arm reserved for Rust/C++ later — not default for C. |
| **Q7 Diagnostics** | **ariadne** as primary renderer; `Diagnostic` is a plain struct so `website/` can render the same data without pulling all of ariadne into WASM if needed. |

---

## Design FAQ

Answers to “My question:” items in this document.

### What does `parse` return? (`File` vs plaintext)

**`File` is not a filesystem path or raw source text.** It is the parsed AST for one `.mek` source:

```rust
pub struct File {
    pub protocol: Option<String>,   // e.g. Some("eCPRI")
    pub items: Vec<Item>,             // structures, enums, choices, type aliases
}
```

| Return type | Pros | Cons |
|-------------|------|------|
| `Result<SpannedFile, Diagnostics>` **(chosen)** | Typed tree for analyze/smith; errors separate; source kept by caller for rendering | Caller holds `&str` + AST together |
| `Result<String, …>` (plaintext only) | Simple dump | Useless for codegen; still need AST internally |
| `Result<File, String>` | Easy to print errors | No structured errors, no spans, hard for IDE |

**API shape:**

```rust
pub struct SpannedFile {
    pub file: File,
    pub span: Span<usize>,          // whole file
}

// Caller keeps source text; diagnostics reference byte offsets into it.
pub fn parse(source: &str) -> Result<SpannedFile, Diagnostics>;
```

`parse_source` remains a thin alias during migration.

### How do spans improve errors?

Today errors point at **tokens** or whole constructs. With spans on AST nodes:

```rust
pub struct Structure {
    pub span: Span<usize>,
    pub name: Spanned<String>,
    pub fields: Vec<Spanned<Field>>,
    // ...
}
```

You can report `S0101` on the **field type** (`iq_samples: …`) while labeling the **width** (`(22 bits)`) as secondary context — same pattern as Rust/ariadne.

**Adding a new error** becomes:

1. Add `DiagnosticCode::S01xx` variant.
2. In one analyzer pass, `emitter.emit(Diagnostic { span: field.span, … })`.
3. Snapshot test the rendered message.

No parser changes required for most semantic errors.

### Which attributes should the DSL support?

Split into **wire semantics** (affect encode/decode) and **codegen/debug** (affect generated API only).

#### Wire / layout (analyzer enforced)

| Attribute | Meaning |
|-----------|---------|
| `[unused]` | Reserved/padding; encode as zero, decode ignored (§4.2 type-1 bits) |
| `[computed]` | Codec fills value on encode/decode; user does not set (e.g. `payload_size`) |
| `[validate(expr)]` | Encode-time check, e.g. `[validate(length == size(data))]` for RMA |

#### Codegen / debug (smith only, ignored on wire)

| Attribute | Meaning |
|-----------|---------|
| `[debug]` / `[no_debug]` | Include/exclude field in generated `print_*` / `Debug` impl (default: **debug on** for non-`[unused]` fields) |
| `[no_encode]` / `[no_decode]` | Generate type but skip one direction (e.g. receive-only stats) |
| `[rename("c_name")]` | Override generated identifier |

**Debuggability:** generate per-type functions such as:

```c
void ecpri_header_print(const ecpri_header_t *h, ecpri_print_fn emit, void *ctx);
/* emit(ctx, "revision = …") — no FILE* required; smith can wrap fprintf for CLI debug */
```

Fields with `[no_debug]` or `[unused]` are skipped unless `--verbose`. This matches your PFCP-style `do_not_print` idea but as a first-class attribute.

Structured in AST (not `Vec<String>`):

```rust
pub enum Attribute {
    Unused,
    Computed,
    NoDebug,
    Validate(Expr),
    // ...
}
```

**Wire width without attributes:** if a field has no `(N bits)`, the analyzer uses the resolved type's width (`u16` → 16, `enumerated(…, 8 bits) E` → 8, `type T (16 bits) = u16` → 16). Error `S0110` if width is unknown. This replaces the old PFCP `[bits=derived]` idea — **no `[derived]` attribute**.

### Are `DynamicArray` / `GreedyArray` enough?

**Enough for v1 and for eCPRI/O-RAN-style protocols.** Most fronthaul stacks are: fixed headers + length-prefixed blobs + discriminated unions.

Add later when a real spec requires it:

| Construct | Example need |
|-----------|----------------|
| `StaticArray<T, N>` | fixed IE list of 4 items |
| `Optional<T, when>` | PFCP `optional=if(s)` |
| `BitSlice<N>` | bit-level views without a full structure |
| `CountedArray<T, count_field>` | sugar over `DynamicArray` |

Prefer **expressions + builtins** (`size`, `pad_to_align_if`) over many generic types.

**Parser:** syntax for `StaticArray`, `Optional`, `BitSlice`, `CountedArray`, `GreedyArray` already parses as generic type names — see `examples/generics-sketch.mek`. Analyzer + smith support is **M2/M5** work.

### Field width vs `[computed]` — two different concepts

| Concept | How to express | Needs attribute? |
|---------|----------------|------------------|
| **Wire width** from underlying type | `message_type: MessageType` or `pc_id: PC_ID` | **No** — analyzer looks up type width |
| **Explicit override** | `address (48 bits): u64` | `(N bits)` only |
| **Codec-filled value** | `payload_size: u16 [computed]` | **`[computed]`** |

#### Original `[derived]` (PFCP `bits=derived`)

In old PFCP notation, `[bits=derived]` on `message_type` meant “width comes from `PFCPMessageType` enum declaration”, not “user doesn't set this”.

**Recommendation: drop `[derived]` entirely.**

- Width-from-type → **default** when `(N bits)` is omitted (`S0110` if type has no width).
- Codec-filled values → **`[computed]`** (e.g. `payload_size`; `message_type` may be implicit when encoding a `Message` — tag follows `payload` variant without marking every header field).

#### `[computed]` (codec-filled values)

Fields the user does **not** supply when building a message; the codec sets them on encode.

**eCPRI `Header` example (updated):**

```mek
structure(msb0) Header {
    revision: ProtocolRevision,       // width: 4 bits from enum
    reserved (3 bits): null [unused],
    concatenation (1 bit): boolean,
    message_type: MessageType,        // width: 8 bits from enum; tag from payload on encode
    payload_size: u16 [computed],     // width: 16 from u16; value = size(payload)
}
```

- **`message_type`**: width inferred from `enumerated(…, 8 bits) MessageType`. On encode, tag is set from which `Payload` variant is being written — may not need `[computed]` if that rule is structural.
- **`payload_size`**: needs `[computed]` — value is `size(encoded payload)`, not inferable from type alone.

**Generated C (user struct):**

```c
typedef struct {
    ecpri_protocol_revision_t revision;
    bool concatenation;
    ecpri_payload_t payload;
} ecpri_message_t;
/* header message_type + payload_size only exist inside encode/decode */
```

### Choice catch-all: opaque bytes (not runtime errors)

For C (and as the **default** across smiths), an unmatched or reserved variant should **decode successfully** into raw bytes — not return an error code the caller must handle everywhere.

**DSL:**

```mek
choice(msb0) Payload(message_type: MessageType, payload_size: u16) on message_type {
    MessageType::iq_data => IQData(payload_size),
    _ => OpaquePayload(payload_size),
}
```

Sugar: `_ => opaque` expands to `OpaquePayload(payload_size)` when the choice has a `payload_size` parameter.

**Generated C (sketch):**

```c
typedef struct {
    ecpri_message_type_t tag;
    union {
        ecpri_iq_data_t iq_data;
        struct {
            uint8_t *data;
            size_t len;
        } opaque;
    } u;
} ecpri_payload_t;
```

Decode uses `switch (tag)` with a `default:` arm that copies `payload_size` bytes into `u.opaque` — no error return for “unknown but valid on wire” types.

**Why not `error("…")` in C?** Opaque storage lets the user inspect `tag` and `u.opaque` when needed, and forward/ignore otherwise. `Result`/error arms can be opt-in for Rust later.

**Analyzer:** `S0304` only fires when neither explicit arms cover the enum **nor** a `_ =>` catch-all exists.

### No “root” type — multiple entry points

Agreed. A protocol defines **many** message types (`Pdu`, `Message`, `IQData`, …). The smith emits **one encode/decode pair per structure** (and per `choice`):

```c
int ecpri_pdu_decode(const uint8_t *buf, size_t len, ecpri_pdu_t *out);
int ecpri_message_decode(const uint8_t *buf, size_t len, ecpri_message_t *out);
int ecpri_iq_data_decode(const uint8_t *buf, size_t len, uint16_t payload_size, ecpri_iq_data_t *out);
```

The user picks the API matching how many bytes they have (full UDP payload → `pdu_decode`; already stripped header → `iq_data_decode` with `payload_size` param).

Optional CLI: `meksmith emit --root Pdu` generates only that subtree — convenience, not required.

---

## Open questions (your answers)

Fill in below or reply in chat; decisions affect analyzer strictness and smith output.

### Q1 — Choice exhaustiveness

When a `choice` does not cover all enum variants (e.g. `Payload` without arms for `reserved`):

- [x] **Error** (`S0304`) — safest for code generation
- [ ] **Warning** (`W0301`) — allow opaque fallback arm
- [ ] **Allow** if an explicit `OpaquePayload` / catch-all arm exists

**Your answer:** Error unless a catch-all is written. Catch-all should decode as **opaque bytes**, not a runtime error (especially in C).

**Resolution:** `_ => OpaquePayload(payload_size)` or `_ => opaque` sugar. See [Design FAQ](#design-faq) § “Choice catch-all”.

**Planned syntax:**

```mek
choice(msb0) Payload(...) on message_type {
    MessageType::iq_data => IQData(payload_size),
    _ => OpaquePayload(payload_size),   /* or: _ => opaque */
}
```

---

### Q2 — Unused structure / choice parameters

`IQData(payload_size: u16)` — parameter is used in expressions. If unused:

- [ ] **Warning** (`W0601`)
- [x] **Error** (`S0601`) — *relaxed to **warning** in [Recorded decisions](#recorded-decisions)*

**Your answer:** Prefer error for safety; open to warning.

**Resolution:** **Warning** by default — symmetric parameter lists across choice arms often leave a param unused in some structures.

---

### Q3 — Non-byte-aligned structures

If total field bits % 8 ≠ 0:

- [ ] **Error** (`S0107`) — only whole-byte protocols
- [x] **Allow** — emit bit-packing logic (harder smith)

**Your answer:** Depends on protocol; should be allowed.

---

### Q4 — C bitfield strategy

For `(N bits)` fields packed in a byte:

- [ ] **ISO C bitfields** (implementation-defined layout risks)
- [x] **Explicit mask/shift** encode/decode (portable, more code)
- [ ] **Both** — flag per structure or protocol

**Your answer:** Host type (e.g. `u32`) with `(22 bits)` on wire — layout and codec must agree; read/write only 22 bits.

---

### Q5 — `[computed]` fields in generated C structs

- [x] **Omit** from user struct; only in wire encode path
- [ ] **Include** but mark read-only / document as codec-filled
- [ ] **Separate** `X` and `X_Wire` types

**Your answer:** Unclear initially; later clarified that **`[derived]` originally meant width-from-type**, not codec-filled.

**Resolution:**

- **Drop `[derived]`** — wire width is implicit from type when `(N bits)` is omitted.
- Use **`[computed]`** only for codec-filled **values** (`payload_size`, etc.).
- See [Design FAQ](#design-faq) § “Field width vs `[computed]`”.

---

### Q6 — Root entry type for codegen

- [ ] Always `Pdu` when present in file
- [ ] Explicit `root Pdu;` directive
- [x] Generate all types; user picks at call site (CLI `--root` optional)

**Your answer:** No explicit root; user calls `X::decode()` for the type they need; protocols have multiple messages.

---

### Q7 — Diagnostic rendering

- [ ] Plain text (line:col + message) first
- [x] **ariadne** / `codespan-reporting` from the start
- [ ] WASM-friendly minimal renderer for website only

**Your answer:** ariadne if it can feed the website later.

**Resolution:** Shared `Diagnostic` struct; full ariadne in CLI/native tests; thin WASM renderer in `website/` that reads the same fields (line, col, message, labels).

---

## Suggested milestone order

1. **M0** — `frontend/` module, `parse_source`, eCPRI example parses — **done**
2. **M1** — AST spans + unified `Diagnostic` + better parse errors — **mostly done** (snapshot tests pending)
3. **M2** — `analyze/` symbol table + type resolution + core `S0001`–`S0109` checks — **done** (expr eval pending)
4. **M3** — dependency graph + `CheckedFile` — **done**
5. **M4** — `smith` trait + C backend (types only) — **done**
6. **M5** — C encode/decode for `Pdu` / `Message` + eCPRI golden test — **in progress** (see [progress.md](./progress.md))
7. **M6** — C++ backend, website integration — **not started**
