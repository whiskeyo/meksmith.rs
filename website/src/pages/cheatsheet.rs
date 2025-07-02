use leptos::prelude::*;

use crate::components::code_editor::{CodeEditor, CodeEditorOptions};
use crate::components::text::TextWithAnimatedGradient;

const MEKLANG_BNF_GRAMMAR: &str = r#"<protocol> ::= (<definition> | <comment>)+
<comment> ::= '#' <text> '\n'
<definition> ::=
      <enumeration_definition>
    | <structure_definition>
    | <union_definition>
    | <type_definition>

<enumeration_definition> ::= 'enum' <identifier> <left_brace> <enumeration_field>+ <right_brace> <semicolon>
<enumeration_field> ::= <identifier> <equal> (<unsigned_integer> | <range>) <semicolon>

<structure_definition> ::= 'struct' <identifier> <left_brace> <structure_field>+ <right_brace> <semicolon>
<structure_field> ::= [<attributes>] <identifier> <colon> <type_identifier> <semicolon>

<union_definition> ::= 'union' <identifier> <left_brace> <union_field>+ <right_brace> <semicolon>
<union_field> ::= (<unsigned_integer> | <range>) <maps_to> <identifier> <colon> <type_identifier> <semicolon>

<attribute> ::=
      'discriminated_by' <equal> <identifier>
    | 'bits' <equal> <unsigned_integer>
    | 'bytes' <equal> <unsigned_integer>
<attribute_tail> ::= <comma> <attribute>
<attributes> ::= <left_bracket> <attribute> <attribute_tail>* <right_bracket>

<type_definition> ::= 'using' <identifier> <equal> <type_identifier> <semicolon>

<type_identifier> ::=
      <builtin_type>
    | <user_defined_type>
    | <static_array_type>
    | <dynamic_array_type>

<builtin_type> ::=
      'int8' | 'int16' | 'int32' | 'int64'
    | 'uint8' | 'uint16' | 'uint32' | 'uint64'
    | 'float32' | 'float64'
    | 'bit' | 'byte'
<user_defined_type> ::= <identifier>
<static_array_type> ::=
      <builtin_type> <left_bracket> <unsigned_integer> <right_bracket>
    | <user_defined_type> <left_bracket> <unsigned_integer> <right_bracket>
<dynamic_array_type> ::=
      <builtin_type> <left_bracket> <right_bracket>
    | <user_defined_type> <left_bracket> <right_bracket>

<range> ::= <unsigned_integer> <double_dot> <unsigned_integer>
<identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*

<unsigned_integer> ::= <hexadecimal> | <binary> | <decimal>
<hexadecimal> ::= "0x" [0-9a-fA-F]+
<binary> ::= "0b" [01]+
<decimal> ::= [0-9]+

<text> ::= [^\n]*

<left_brace> ::= '{'
<right_brace> ::= '}'
<left_bracket> ::= '['
<right_bracket> ::= ']'
<semicolon> ::= ';'
<colon> ::= ':'
<maps_to> ::= '=>'
<equal> ::= '='
<comma> ::= ','
<double_dot> ::= '..'"#;

const MEKLANG_BUILTIN_TYPES: &str = r#"int8, int16, int32, int64,
uint8, uint16, uint32, uint64,
float32, float64,
bit, byte"#;

const MEKLANG_STRUCTURE_EXAMPLE: &str = r#"struct StructureName {
    first_field: uint8;
    second_field: int16;
    third_field: bit;
};"#;

const MEKLANG_ENUMERATION_EXAMPLE: &str = r#"enum EnumerationName {
    single_value = 1;
    another_single_value = 2;
    range_of_values = 3..10;
};"#;

const MEKLANG_UNION_EXAMPLE: &str = r#"union UnionName {
    0 => first_field: uint8;
    1 => second_field: int16;
    2 => third_field: bit;
};"#;

const MEKLANG_ATTRIBUTES_EXAMPLE: &str = r#"[discriminated_by=name_of_field]
[bits=size_in_bits]
[bytes=size_in_bytes]"#;

const MEKLANG_DISCRIMINATED_BY_ATTRIBUTE_EXAMPLE: &str = r#"structure StructureName {
    my_field: uint8;
    [discriminated_by=my_field]
    some_union: UnionName;
};"#;

const MEKLANG_BITS_BYTES_ATTRIBUTE_EXAMPLE: &str = r#"structure StructureName {
    [bits=6]
    my_field: uint8;
    [bytes=3]
    another_field: uint32;
};"#;

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
                        description="There are a few supported built-in types, which are appropriately mapped to built-in types of various languages by smiths."
                        code_example=MEKLANG_BUILTIN_TYPES
                    />
                    <CheatsheetBox
                        title="smiths"
                        description="\"smiths\" are the code generators that produce code in a specific language. Currently, only C is supported, but more languages are planned to be added in the future, such as Rust, Python, C++, Go, and possibly even Wireshark dissectors."
                    />
                    <CheatsheetBoxWithCode
                        title="structures"
                        description="Simple structure containing a few fields with different types."
                        code_example=MEKLANG_STRUCTURE_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="enumerations"
                        description="Enumerations can be defined in a similar way to C language, but they also support ranges of values."
                        code_example=MEKLANG_ENUMERATION_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="(discriminated) unions"
                        description="Unions allow you to define a field that can hold different types, similar to C unions. The value before => is the discriminator."
                        code_example=MEKLANG_UNION_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="attributes"
                        description="Structure fields can contain attributes that specify additional properties or behaviors in encoding/decoding."
                        code_example=MEKLANG_ATTRIBUTES_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="discriminated_by attribute"
                        description="The discriminated_by attribute \"connects\" a union to its discriminator field. The discriminator field can be any field in the structure and might be either integer, byte or enumeration. If enum is used, not existing values may cause issues in smiths."
                        code_example=MEKLANG_DISCRIMINATED_BY_ATTRIBUTE_EXAMPLE
                    />
                    <CheatsheetBoxWithCode
                        title="bits and bytes attributes"
                        description="The bits and bytes attributes allow you to specify the size of a field in bits or bytes. Since there is no padding in meklang, the output size will be 6 bits + 3 bytes = 27 bits."
                        code_example=MEKLANG_BITS_BYTES_ATTRIBUTE_EXAMPLE
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
    let height = code_example.lines().count() as u16 * 24;
    let (code, set_code) = signal(code_example.to_string());

    view! {
        <div class="documentation-box">
            <h2 class="documentation-box-title">{title}</h2>
            <p>{description}</p>
            <div class="center">
                <CodeEditor
                    code_editor_options=CodeEditorOptions {
                        width_in_pixels: 343,
                        height_in_pixels: height,
                    }
                    code
                    set_code
                    disabled=true
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
