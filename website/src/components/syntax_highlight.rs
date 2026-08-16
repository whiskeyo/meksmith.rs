#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeEditorLanguage {
    #[allow(dead_code)]
    PlainText,
    Meklang,
    C,
    Cpp,
}

const CLASS_KEYWORD: &str = "code-editor-highlight-keyword";
const CLASS_BUILTIN: &str = "code-editor-highlight-builtin-type";
const CLASS_COMMENT: &str = "code-editor-highlight-comment";

struct LangRules {
    keywords: &'static [&'static str],
    builtins: &'static [&'static str],
    hash_line_comment: bool,
    slash_line_comment: bool,
}

const MEKLANG: LangRules = LangRules {
    keywords: &[
        "choice",
        "enumerated",
        "null",
        "protocol",
        "structure",
        "using",
    ],
    builtins: &[
        "bit", "byte", "float32", "float64", "int16", "int32", "int64", "int8", "uint16", "uint32",
        "uint64", "uint8",
    ],
    hash_line_comment: true,
    slash_line_comment: false,
};

const C: LangRules = LangRules {
    keywords: &["enum", "static", "struct", "typedef", "union"],
    builtins: &[
        "bool", "char", "double", "float", "int", "int16_t", "int32_t", "int64_t", "int8_t",
        "long", "size_t", "uint16_t", "uint32_t", "uint64_t", "uint8_t", "unsigned", "void",
    ],
    hash_line_comment: true,
    slash_line_comment: true,
};

const CPP: LangRules = LangRules {
    keywords: &[
        "class",
        "concept",
        "const",
        "constexpr",
        "enum",
        "inline",
        "namespace",
        "nodiscard",
        "requires",
        "static",
        "struct",
        "template",
        "typename",
        "union",
        "using",
    ],
    builtins: &[
        "bool", "char", "expected", "int16_t", "int32_t", "int64_t", "int8_t", "optional",
        "size_t", "span", "std", "uint16_t", "uint32_t", "uint64_t", "uint8_t", "variant",
        "vector",
    ],
    hash_line_comment: true,
    slash_line_comment: true,
};

fn rules_for(language: CodeEditorLanguage) -> Option<&'static LangRules> {
    match language {
        CodeEditorLanguage::PlainText => None,
        CodeEditorLanguage::Meklang => Some(&MEKLANG),
        CodeEditorLanguage::C => Some(&C),
        CodeEditorLanguage::Cpp => Some(&CPP),
    }
}

pub fn highlight(code: &str, language: CodeEditorLanguage) -> String {
    let Some(rules) = rules_for(language) else {
        return escape_html(code);
    };

    let mut out = String::with_capacity(code.len() + code.len() / 8);
    let bytes = code.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if rules.hash_line_comment && bytes[index] == b'#' {
            let start = index;
            index = next_line_end(bytes, index);
            write_span(&mut out, &code[start..index], CLASS_COMMENT);
            continue;
        }

        if rules.slash_line_comment
            && index + 1 < bytes.len()
            && bytes[index] == b'/'
            && bytes[index + 1] == b'/'
        {
            let start = index;
            index = next_line_end(bytes, index);
            write_span(&mut out, &code[start..index], CLASS_COMMENT);
            continue;
        }

        if bytes[index] == b'"' {
            let start = index;
            index += 1;
            while index < bytes.len() {
                if bytes[index] == b'\\' {
                    index = (index + 2).min(bytes.len());
                    continue;
                }
                if bytes[index] == b'"' {
                    index += 1;
                    break;
                }
                index += 1;
            }
            write_raw(&mut out, &code[start..index]);
            continue;
        }

        if is_ident_start(bytes[index]) {
            let start = index;
            index += 1;
            while index < bytes.len() && is_ident_continue(bytes[index]) {
                index += 1;
            }
            let word = &code[start..index];
            let class = if contains_sorted(rules.keywords, word) {
                CLASS_KEYWORD
            } else if contains_sorted(rules.builtins, word) {
                CLASS_BUILTIN
            } else {
                write_raw(&mut out, word);
                continue;
            };
            write_span(&mut out, word, class);
            continue;
        }

        write_raw_char(&mut out, bytes[index] as char);
        index += 1;
    }

    if out.ends_with('\n') {
        out.push(' ');
    }

    out
}

fn next_line_end(bytes: &[u8], mut index: usize) -> usize {
    while index < bytes.len() && bytes[index] != b'\n' {
        index += 1;
    }
    if index < bytes.len() {
        index += 1;
    }
    index
}

fn is_ident_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_ident_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn contains_sorted(haystack: &[&str], needle: &str) -> bool {
    haystack.binary_search(&needle).is_ok()
}

fn write_raw(out: &mut String, text: &str) {
    for ch in text.chars() {
        write_raw_char(out, ch);
    }
}

fn write_raw_char(out: &mut String, ch: char) {
    match ch {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        _ => out.push(ch),
    }
}

fn write_span(out: &mut String, text: &str, class: &str) {
    out.push_str(r#"<span class=""#);
    out.push_str(class);
    out.push_str(r#"">"#);
    write_raw(out, text);
    out.push_str("</span>");
}

fn escape_html(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    write_raw(&mut out, code);
    if out.ends_with('\n') {
        out.push(' ');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpp_highlights_without_nested_span_corruption() {
        let source = "enum class Tag : std::uint8_t {\n    alpha = 0,\n};\n#include <expected>";
        let highlighted = highlight(source, CodeEditorLanguage::Cpp);

        assert!(!highlighted.contains("<span <span"));
        assert!(!highlighted.contains("class</span>="));
        assert!(highlighted.contains(r#"class="code-editor-highlight-keyword">class</span>"#));
        assert!(highlighted.contains(r#"class="code-editor-highlight-builtin-type">std</span>"#));
        assert!(highlighted.contains("&lt;expected&gt;"));
    }

    #[test]
    fn meklang_comments_are_highlighted() {
        let highlighted = highlight("# comment\nprotocol P;", CodeEditorLanguage::Meklang);
        assert!(highlighted.contains(r#"class="code-editor-highlight-comment"># comment"#));
        assert!(highlighted.contains(r#"class="code-editor-highlight-keyword">protocol</span>"#));
    }

    #[test]
    fn cpp_line_comments_are_highlighted() {
        let highlighted = highlight("// note\nnamespace n {}", CodeEditorLanguage::Cpp);
        assert!(highlighted.contains(r#"class="code-editor-highlight-comment">// note"#));
        assert!(highlighted.contains(r#"class="code-editor-highlight-keyword">namespace</span>"#));
    }
}
