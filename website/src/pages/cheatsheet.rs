use leptos::prelude::*;

use crate::components::code_editor::{CodeEditor, CodeEditorLanguage, CodeEditorOptions};
use crate::components::text::TextWithAnimatedGradient;

const MEKLANG_BNF_GRAMMAR: &str = r#"<file> ::= [<protocol_decl>] <item>*
<comment> ::= '//' <text> '\n' | '#' <text> '\n'

<protocol_decl> ::= 'protocol' <identifier> ';'

<item> ::=
      <structure_def>
    | <enumerated_def>
    | <choice_def>
    | <type_alias>

<structure_def> ::=
    'structure' '(' <bit_order> ')' <identifier> [<params>] <left_brace> <field_list> <right_brace>
<enumerated_def> ::=
    'enumerated' '(' <bit_order> <comma> <width_bits> ')' <identifier> <left_brace> <variant_list> <right_brace>
<choice_def> ::=
    'choice' '(' <bit_order> ')' <identifier> [<params>] 'on' <identifier> <left_brace> <arm_list> <right_brace>
<type_alias> ::= 'type' <identifier> [<width_bits>] <equal> <type_expr> <semicolon>

<params> ::= <left_paren> <param> (<comma> <param>)* <right_paren>
<param> ::= <identifier> <colon> <identifier>

<field_list> ::= <field> (<comma> <field>)* <comma>?
<field> ::= <identifier> [<width_bits>] <colon> <type_expr> [<attributes>]

<variant_list> ::= <enum_variant> (<comma> <enum_variant>)* <comma>?
<enum_variant> ::= <identifier> <equal> (<integer> | <range>)

<arm_list> ::= <choice_arm> (<comma> <choice_arm>)* <comma>?
<choice_arm> ::= <choice_pattern> <fat_arrow> <type_expr>
<choice_pattern> ::=
      <integer>
    | <identifier> <double_colon> <identifier>
    | '_'

<attributes> ::= <left_bracket> <identifier> (<comma> <identifier>)* <right_bracket>

<type_expr> ::=
      'null'
    | <identifier> [<generics>] [<type_args>]
<generics> ::= <left_angle> <expr> (<comma> <expr>)* <right_angle>
<type_args> ::= <left_paren> <expr> (<comma> <expr>)* <right_paren>

<width_bits> ::= <left_paren> <expr> ' bits' <right_paren>
<bit_order> ::= 'msb0' | 'lsb0'
<range> ::= <integer> <double_dot> <integer>

<expr> ::= <integer> | <identifier> | <expr> <dot> <identifier> | <call> | '(' <expr> ')' | <binary_expr>
<call> ::= <expr> <left_paren> [<expr> (<comma> <expr>)*] <right_paren>
<binary_expr> ::= <expr> ('+' | '-' | '*' | '/') <expr>

<identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*
<integer> ::= <decimal> | <hexadecimal> | <binary>
<decimal> ::= [0-9]+
<hexadecimal> ::= '0x' [0-9a-fA-F]+
<binary> ::= '0b' [01]+
<text> ::= [^\n]*

<left_brace> ::= '{'
<right_brace> ::= '}'
<left_bracket> ::= '['
<right_bracket> ::= ']'
<left_paren> ::= '('
<right_paren> ::= ')'
<left_angle> ::= '<'
<right_angle> ::= '>'
<semicolon> ::= ';'
<colon> ::= ':'
<fat_arrow> ::= '=>'
<double_colon> ::= '::'
<equal> ::= '='
<comma> ::= ','
<dot> ::= '.'
<double_dot> ::= '..'"#;

const MEKLANG_BUILTIN_TYPES: &str = r#"int8, int16, int32, int64,
uint8, uint16, uint32, uint64,
float32, float64,
bit, byte, boolean, null"#;

const MEKLANG_PROTOCOL_EXAMPLE: &str = r#"protocol Demo;"#;

const MEKLANG_STRUCTURE_EXAMPLE: &str = r#"structure(msb0) Header {
    revision: ProtocolRevision,
    reserved (3 bits): null [unused],
    flag (1 bit): boolean,
    body_size: u16 [computed],
}"#;

const MEKLANG_ENUMERATION_EXAMPLE: &str = r#"enumerated(msb0, 8 bits) MessageType {
    alpha = 0,
    beta = 1,
    reserved = 2..15,
}"#;

const MEKLANG_CHOICE_EXAMPLE: &str = r#"choice(msb0) Payload(message_type: MessageType, payload_size: u16)
    on message_type {
    MessageType::alpha => AlphaBody(payload_size),
    MessageType::beta => BetaBody(payload_size),
    _ => OpaquePayload(payload_size),
}"#;

const MEKLANG_TYPE_ALIAS_EXAMPLE: &str = r#"type PC_ID (16 bits) = u16;"#;

const MEKLANG_ATTRIBUTES_EXAMPLE: &str = r#"[computed]
[unused]"#;

const MEKLANG_PARAMETRIC_STRUCTURE_EXAMPLE: &str = r#"structure(msb0) IQData(payload_size: u16) {
    pc_id: PC_ID,
    samples: DynamicArray<byte, payload_size - 4>,
}"#;

#[component]
pub fn Cheatsheet() -> impl IntoView {
    view! {
        <div class="center">
            <h2><TextWithAnimatedGradient text="meksmith.rs" /> " cheatsheet"</h2>
        </div>
        <div class="flex-container">
            <div class="flex-1 documentation-box">
                <h2 class="documentation-box-title">"grammar in BNF"<i>"ish"</i>" notation"</h2>
                <MeklangBNFNotation />
            </div>
            <div class="flex-1">
                <div class="documentation-grid">
                    <CheatsheetBoxWithCode
                        title="built-in types"
                        description="Built-in scalar types map to host-language primitives. boolean and null are also available for flags and reserved bitfields."
                        code_example=MEKLANG_BUILTIN_TYPES
                    />
                    <CheatsheetBox
                        title="smiths"
                        description="Smiths are code generators that turn meklang into ready-to-use wire codecs. C and C++23 backends are available today; more targets may follow."
                    />
                    <CheatsheetBoxWithCode
                        title="protocol"
                        description="Optional protocol name scopes generated C/C++ symbols (for example ecpri_message_encode)."
                        code_example=MEKLANG_PROTOCOL_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="structures"
                        description="Structures declare bit order with msb0 or lsb0. Fields may specify an explicit wire width in bits and optional attributes."
                        code_example=MEKLANG_STRUCTURE_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="enumerations"
                        description="Enumerations take a bit order and fixed width. Variants may be single values or inclusive ranges."
                        code_example=MEKLANG_ENUMERATION_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="choices"
                        description="Choices dispatch on a discriminant field or parameter. Arms use EnumType::variant, integer literals, or _ for catch-all."
                        code_example=MEKLANG_CHOICE_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="type aliases"
                        description="Type aliases attach a name (and optional wire width) to another type expression."
                        code_example=MEKLANG_TYPE_ALIAS_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="attributes"
                        description="Field attributes control codec behavior. computed fields are filled by the encoder; unused fields are on the wire but ignored on decode."
                        code_example=MEKLANG_ATTRIBUTES_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="parametric structures"
                        description="Structures and choices may take parameters used in field types and array lengths."
                        code_example=MEKLANG_PARAMETRIC_STRUCTURE_EXAMPLE
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn CheatsheetBoxWithCode(
    title: &'static str,
    description: &'static str,
    code_example: &'static str,
) -> impl IntoView {
    let height = code_example.lines().count() as u32 * 26;
    let (code, set_code) = signal(code_example.to_string());

    view! {
        <div class="documentation-box">
            <h2 class="documentation-box-title">{title}</h2>
            <p>{description}</p>
            <div class="center">
                <CodeEditor
                    code_editor_options=CodeEditorOptions {
                        width: 375,
                        height,
                        language: CodeEditorLanguage::Meklang,
                        disabled: true,
                    }
                    code
                    set_code
                />
            </div>
        </div>
    }
}

#[component]
fn CheatsheetBox(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="documentation-box">
            <h2 class="documentation-box-title">{title}</h2>
            <p>{description}</p>
        </div>
    }
}

fn replace_between(
    input: &str,
    start: char,
    end: char,
    wrap_fn: impl Fn(&str) -> String,
) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == start {
            let mut content = String::new();
            while let Some(&next) = chars.peek() {
                chars.next();
                if next == end {
                    break;
                } else {
                    content.push(next);
                }
            }
            output.push_str(&wrap_fn(&content));
        } else {
            output.push(c);
        }
    }

    output
}

fn highlight_bnf_code(code: &str) -> String {
    fn keep_whitespaces(input: &str) -> String {
        input
            .replace(" ", "&nbsp;")
            .replace("\t", "&nbsp;&nbsp;&nbsp;&nbsp;")
    }

    code.lines()
        .map(|line| {
            let mut line = keep_whitespaces(line);

            line = replace_between(&line, '<', '>', |content| {
                format!(r#"<span class="bnf-nonterminal">&lt;{content}&gt;</span>"#)
            });

            line = line
                .replace("::=", r#"<span class="bnf-operator">::=</span>"#)
                .replace("|", r#"<span class="bnf-operator">|</span>"#)
                .replace("(", r#"<span class="bnf-operator">(</span>"#)
                .replace(")", r#"<span class="bnf-operator">)</span>"#)
                .replace("[", r#"<span class="bnf-operator">[</span>"#)
                .replace("]", r#"<span class="bnf-operator">]</span>"#)
                .replace("+", r#"<span class="bnf-operator">+</span>"#)
                + "<br>";

            line = replace_between(&line, '\'', '\'', |content| {
                format!(r#"<span class="bnf-keyword">'{content}'</span>"#)
            });

            line
        })
        .collect()
}

#[component]
fn MeklangBNFNotation() -> impl IntoView {
    view! {
        <div inner_html={highlight_bnf_code(MEKLANG_BNF_GRAMMAR)} />
    }
}
