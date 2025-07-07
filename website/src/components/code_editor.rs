use crate::components::text::TextWithAnimatedGradient;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub(crate) enum CodeEditorLanguage {
    None,
    Meklang,
    C,
}

impl CodeEditorLanguage {
    fn get_highlighter(&self) -> LanguageHighlighter {
        match self {
            CodeEditorLanguage::None => LanguageHighlighter {
                keywords: vec![],
                builtin_types: vec![],
            },
            CodeEditorLanguage::Meklang => LanguageHighlighter {
                keywords: vec!["enum ", "struct ", "union ", "using "],
                builtin_types: vec![
                    "uint8", "uint16", "uint32", "uint64", "int8", "int16", "int32", "int64",
                    "float32", "float64", "bit", "byte",
                ],
            },
            CodeEditorLanguage::C => LanguageHighlighter {
                keywords: vec![
                    "if", "else", "while", "for", "return", "struct", "union", "enum", "typedef",
                ],
                builtin_types: vec![],
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CodeEditorOptions {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) language: CodeEditorLanguage,
}

impl CodeEditorOptions {
    pub(crate) fn get_formatted_size(&self) -> String {
        format!("width: {}px; height: {}px;", self.width, self.height)
    }

    pub(crate) fn highlight_code(&self, code: &str) -> String {
        match self.language {
            CodeEditorLanguage::None => self.language.get_highlighter().highlight(code),
            CodeEditorLanguage::Meklang => self.language.get_highlighter().highlight(code),
            CodeEditorLanguage::C => self.language.get_highlighter().highlight(code),
        }
    }
}

struct LanguageHighlighter {
    keywords: Vec<&'static str>,
    builtin_types: Vec<&'static str>,
}

impl LanguageHighlighter {
    fn highlight(&self, code: &str) -> String {
        fn escape_html(input: &str) -> String {
            input
                .replace("&", "&amp;")
                .replace("<", "&lt;")
                .replace(">", "&gt;")
        }

        let mut highlighted_code = escape_html(code);

        for keyword in &self.keywords {
            highlighted_code = highlighted_code.replace(
                keyword,
                &format!("<span class=\"code-editor-highlight-keyword\">{keyword}</span>"),
            );
        }

        for builtin_type in &self.builtin_types {
            highlighted_code = highlighted_code.replace(
                builtin_type,
                &format!(
                    "<span class=\"code-editor-highlight-builtin-type\">{builtin_type}</span>"
                ),
            );
        }

        if highlighted_code.ends_with('\n') {
            highlighted_code.push(' ');
        }

        highlighted_code
    }
}

#[component]
pub fn CodeEditor(
    code_editor_options: CodeEditorOptions,
    disabled: bool,
    #[prop(into)] code: ReadSignal<String>,
    #[prop(into)] set_code: WriteSignal<String>,
) -> impl IntoView {
    let textarea_code_ref: NodeRef<leptos::html::Textarea> = NodeRef::new();
    let code_editor_options_for_textarea = code_editor_options.clone();
    textarea_code_ref.on_load(move |textarea| {
        textarea.set_cols(code_editor_options_for_textarea.width);
        textarea.set_rows(code_editor_options_for_textarea.height);
        textarea.set_value(&code.get());
        textarea.set_spellcheck(false);
        textarea.set_class_name("code-editor");

        if disabled {
            textarea.set_attribute("disabled", "true").unwrap();
        }
    });

    let pre_parsed_code_ref: NodeRef<leptos::html::Pre> = NodeRef::new();
    let code_editor_options_for_pre = code_editor_options.clone();
    pre_parsed_code_ref.on_load(move |pre| {
        pre.set_class_name("code-editor-highlighted");
        pre.set_scroll_top(textarea_code_ref.get().unwrap().scroll_top());
        pre.set_inner_html(
            &code_editor_options_for_pre
                .clone()
                .highlight_code(&code.get()),
        );
    });

    let pre_line_numbers_ref: NodeRef<leptos::html::Pre> = NodeRef::new();
    pre_line_numbers_ref.on_load(move |pre| {
        pre.set_class_name("code-editor-line-numbers");
        pre.set_scroll_top(textarea_code_ref.get().unwrap().scroll_top());
        pre.set_text_content(Some(&get_line_numbers(&code.get())));
    });

    let code_editor_options_for_sync = code_editor_options.clone();
    let sync = move |_| {
        let textarea = textarea_code_ref.get().unwrap();
        let pre_parsed_code = pre_parsed_code_ref.get().unwrap();
        let pre_line_numbers = pre_line_numbers_ref.get().unwrap();

        set_code.set(textarea.value());
        pre_parsed_code.set_inner_html(
            code_editor_options_for_sync
                .clone()
                .highlight_code(&textarea.value())
                .as_str(),
        );
        pre_line_numbers
            .set_text_content(Some(get_line_numbers(textarea.value().as_str()).as_str()));

        // Synchronize scroll position between textarea, rendered code, and line numbers
        let scroll_top = textarea.scroll_top();
        let scroll_left = textarea.scroll_left();

        pre_parsed_code.set_scroll_top(scroll_top);
        pre_parsed_code.set_scroll_left(scroll_left);
        pre_line_numbers.set_scroll_top(scroll_top);
        pre_line_numbers.set_scroll_left(scroll_left);
    };

    let keydown = move |event: web_sys::KeyboardEvent| {
        if event.key() == "Tab" {
            event.prevent_default();
            let textarea = textarea_code_ref.get().unwrap();
            let start = textarea.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;
            let end = textarea.selection_end().unwrap_or(Some(0)).unwrap_or(0) as usize;
            let value = textarea.value();

            // Insert a tab character at the cursor position
            let new_value = format!("{}\t{}", &value[..start], &value[end..]);
            set_code.set(new_value.clone());
            textarea.set_value(&new_value);
            textarea
                .set_selection_range((start + 1) as u32, (start + 1) as u32)
                .unwrap();
        }
    };

    Effect::new({
        let code_editor_options_for_effect = code_editor_options.clone();
        move |_| {
            if let Some(textarea) = textarea_code_ref.get() {
                if textarea.value() != code.get() {
                    textarea.set_value(&code.get());
                }
            }

            if let Some(pre) = pre_parsed_code_ref.get() {
                pre.set_inner_html(&code_editor_options_for_effect.highlight_code(&code.get()));
            }
        }
    });

    view! {
        <div class="code-editor-container" style=code_editor_options.clone().get_formatted_size()>
            <pre node_ref=pre_line_numbers_ref></pre>
            <pre node_ref=pre_parsed_code_ref></pre>
            <textarea node_ref=textarea_code_ref
                on:input=sync.clone()
                on:scroll=sync.clone()
                on:keydown=keydown
            ></textarea>
        </div>
    }
}

#[component]
pub fn CodeEditorWithOutput(
    input_code_editor_options: CodeEditorOptions,
    output_code_editor_options: CodeEditorOptions,
    extra_section_classes: &'static str,
    meklang_code: String,
    disable_input: bool,
) -> impl IntoView {
    let (code, set_code) = signal(meklang_code);
    let (parsed_code, set_parsed_code) = signal(String::new());
    let (parsing_error, set_parsing_error) = signal(String::new());

    Effect::new(move |_| {
        match meksmith::smith_c::generate_c_code_from_string(code.get().as_str()) {
            Ok(c_code) => {
                set_parsed_code.set(c_code);
                set_parsing_error.set(String::new());
            }
            Err(e) => set_parsing_error.set(e),
        }
    });

    view! {
        <section class={extra_section_classes.to_string() + " flex-container flex-row"}>
            <div class="flex-1">
                <h3>"Input in " <TextWithAnimatedGradient text="meklang" /> </h3>
                <CodeEditor disabled=disable_input code_editor_options=input_code_editor_options.clone() code=code set_code=set_code />
                <Show
                    when=move || !parsing_error.get().is_empty()
                >
                    <div class="code-editor-error-box">
                        {move || parsing_error.get()}
                    </div>
                </Show>
            </div>
            <div class="flex-1">
                <h3>"Generated output in C"</h3>
                <CodeEditor disabled=true code_editor_options=output_code_editor_options.clone() code=parsed_code set_code=set_parsed_code />
            </div>
        </section>
    }
}

/// Returns all line numbers separated by a newline in the given code string.
/// The number of lines is determined by counting the number of newline characters
/// in the code, supporting also multiple empty lines. Numbering starts from 1
/// and each line (including empty lines) is numbered sequentially.
fn get_line_numbers(code: &str) -> String {
    let number_of_lines = if code.is_empty() {
        1
    } else {
        code.split('\n').count()
    };

    (1..=number_of_lines)
        .map(|n| n.to_string() + "\n")
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_editor_options_get_formatted_size() {
        let options = CodeEditorOptions {
            width: 800,
            height: 600,
            language: CodeEditorLanguage::None,
        };

        assert_eq!(options.get_formatted_size(), "width: 800px; height: 600px;");
    }

    #[test]
    fn test_get_line_numbers() {
        assert_eq!(get_line_numbers(""), "1\n");
        assert_eq!(get_line_numbers("line1\nline2"), "1\n2\n");
        assert_eq!(get_line_numbers("line1\nline2\nline3"), "1\n2\n3\n");
        assert_eq!(get_line_numbers("line1\nline2\nline3\n"), "1\n2\n3\n4\n");
        assert_eq!(
            get_line_numbers("line1\nline2\nline3\nline4"),
            "1\n2\n3\n4\n"
        );
    }

    #[test]
    fn test_get_line_numbers_with_multiple_empty_lines() {
        assert_eq!(get_line_numbers("\n\n\n\n\n"), "1\n2\n3\n4\n5\n6\n");
    }
}
