use leptos::prelude::*;

use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn Documentation() -> impl IntoView {
    view! {
        <div class="center">
            <h2><TextWithAnimatedGradient text="meksmith.rs" /> " docs"</h2>
        </div>
        <div class="flex-container">
            <div class="flex-1 documentation-box">
                <h2 class="documentation-box-title">"meklang grammar in BNF-like notation"</h2>
                <div class="documentation-box-content-scrollable">
                    <MeklangBNFNotation />
                </div>
            </div>
            <div class="flex-1">
                <div class="documentation-grid">
                    <div class="documentation-box">
                        <h2 class="documentation-box-title">"meklang code example 1"</h2>
                        <p>"Description for example 1."</p>
                    </div>
                    <div class="documentation-box">
                        <h2 class="documentation-box-title">"meklang code example 2"</h2>
                        <p>"Description for example 2."</p>
                    </div>
                    <div class="documentation-box">
                        <h2 class="documentation-box-title">"meklang code example 3"</h2>
                        <p>"Description for example 3."</p>
                    </div>
                    <div class="documentation-box">
                        <h2 class="documentation-box-title">"meklang code example 4"</h2>
                        <p>"Description for example 4."</p>
                    </div>
                </div>
            </div>
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
pub fn MeklangBNFNotation() -> impl IntoView {
    let meklang_bnf_grammar = r#"<protocol> ::= (<definition> | <comment>)+
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

    view! {
        <div inner_html={highlight_bnf_code(meklang_bnf_grammar)} />
    }
}
