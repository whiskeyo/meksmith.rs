use crate::components::text::TextWithAnimatedGradient;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct CodeEditorOptions {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl CodeEditorOptions {
    pub(crate) fn get_formatted_size(&self) -> String {
        format!("width: {}px; height: {}px;", self.width, self.height)
    }
}

const MEKLANG_KEYWORDS: [&str; 4] = ["enum ", "struct ", "union ", "using "];

fn escape_html(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

fn highlight_meklang_code(code: &str) -> String {
    let mut highlighted_code = escape_html(code);

    MEKLANG_KEYWORDS.iter().for_each(|keyword| {
        highlighted_code = highlighted_code.replace(
            keyword,
            &format!("<span class=\"code-editor-highlight-keyword\">{keyword}</span>"),
        );
    });

    if highlighted_code.ends_with('\n') {
        highlighted_code.push(' ');
    }

    highlighted_code
}

#[component]
pub fn CodeEditor(
    code_editor_options: CodeEditorOptions,
    disabled: bool,
    #[prop(into)] code: ReadSignal<String>,
    #[prop(into)] set_code: WriteSignal<String>,
) -> impl IntoView {
    let line_numbers = move || get_line_numbers(&code.get());
    // let (parsed_code, set_parsed_code) = signal(highlight_meklang_code(&code.get()));
    let parsed_code = move || highlight_meklang_code(&code.get());

    let textarea_code_ref: NodeRef<leptos::html::Textarea> = NodeRef::new();
    textarea_code_ref.on_load(move |textarea| {
        textarea.set_cols(code_editor_options.width);
        textarea.set_rows(code_editor_options.height);
        textarea.set_value(&code.get());

        if disabled {
            textarea.set_attribute("disabled", "true").unwrap();
        }
    });

    let pre_parsed_code_ref: NodeRef<leptos::html::Pre> = NodeRef::new();
    pre_parsed_code_ref.on_load(move |pre| {
        pre.set_scroll_top(textarea_code_ref.get().unwrap().scroll_top());
    });

    let pre_line_numbers_ref: NodeRef<leptos::html::Pre> = NodeRef::new();
    pre_line_numbers_ref.on_load(move |pre| {
        pre.set_scroll_top(textarea_code_ref.get().unwrap().scroll_top());
    });

    let sync = move |_| {
        let code_value = textarea_code_ref.get().unwrap().value();
        set_code.set(code_value.clone());
        // set_parsed_code.set(highlight_meklang_code(&code_value));
        pre_parsed_code_ref
            .get()
            .unwrap()
            .set_inner_html(&highlight_meklang_code(
                &textarea_code_ref.get().unwrap().value(),
            ));

        pre_line_numbers_ref.get().unwrap().set_text_content(Some(
            get_line_numbers(textarea_code_ref.get().unwrap().value().as_str()).as_str(),
        ));

        // Synchronize scroll positions
        let textarea = textarea_code_ref.get().unwrap();
        let scroll_top = textarea.scroll_top();
        let scroll_left = textarea.scroll_left();

        pre_parsed_code_ref
            .get()
            .unwrap()
            .set_scroll_top(scroll_top);

        pre_parsed_code_ref
            .get()
            .unwrap()
            .set_scroll_left(scroll_left);

        pre_line_numbers_ref
            .get()
            .unwrap()
            .set_scroll_top(scroll_top);

        pre_line_numbers_ref
            .get()
            .unwrap()
            .set_scroll_left(scroll_left);

        leptos::logging::log!(
            "Number of lines in code editor's components: textarea: {}, pre_parsed: {}, pre_line_numbers: {}",
            textarea_code_ref.get().unwrap().value().lines().count(),
            pre_parsed_code_ref
                .get()
                .unwrap()
                .text_content()
                .unwrap_or_default()
                .lines()
                .count(),
            pre_line_numbers_ref
                .get()
                .unwrap()
                .text_content()
                .unwrap_or_default()
                .lines()
                .count()
        );
    };

    Effect::new({
        move |_| {
            if let Some(textarea) = textarea_code_ref.get() {
                if textarea.value() != code.get() {
                    textarea.set_value(&code.get());
                }
            }

            if let Some(pre) = pre_parsed_code_ref.get() {
                pre.set_inner_html(&highlight_meklang_code(&code.get()));
            }
        }
    });

    view! {
        <div class="code-editor-container" style=code_editor_options.get_formatted_size()>
            <pre node_ref=pre_line_numbers_ref class="code-editor-line-numbers">
                { move || line_numbers }
            </pre>
            <pre node_ref=pre_parsed_code_ref class="code-editor-highlighted">
                { move || parsed_code }
            </pre>
            <textarea
                node_ref=textarea_code_ref
                on:input=sync
                on:scroll=sync
                class="code-editor"
                spellcheck="false"
            >
            </textarea>
        </div>
    }
}

#[component]
pub fn CodeEditorWithOutput(
    code_editor_options: CodeEditorOptions,
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
                <CodeEditor disabled=disable_input code_editor_options=code_editor_options.clone() code=code set_code=set_code />
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
                <CodeEditor disabled=true code_editor_options=code_editor_options.clone() code=parsed_code set_code=set_parsed_code />
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
