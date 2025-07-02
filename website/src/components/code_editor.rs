use crate::components::text::TextWithAnimatedGradient;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct CodeEditorOptions {
    pub(crate) width_in_pixels: u16,
    pub(crate) height_in_pixels: u16,
}

impl CodeEditorOptions {
    pub fn style(&self) -> String {
        format!(
            "width: {}px; min-height: {}px; max-height: {}px;",
            self.width_in_pixels, self.height_in_pixels, self.height_in_pixels
        )
    }
}

#[component]
pub fn CodeEditor(
    code_editor_options: CodeEditorOptions,
    disabled: bool,
    #[prop(into)] code: ReadSignal<String>,
    #[prop(into)] set_code: WriteSignal<String>,
) -> impl IntoView {
    let line_numbers = move || get_line_numbers(&code.get());
    let code_editor_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if let Some(div) = code_editor_ref.get() {
            let content = div.inner_text();
            let code_val = code.get();
            if content != code_val {
                div.set_inner_text(&code_val);
            }
        }
    });

    let on_input = move |_| {
        if let Some(div) = code_editor_ref.get() {
            set_code.set(div.inner_text());
        }
    };

    view! {
        <div class="code-editor-container" style=code_editor_options.style()>
            <div class="code-editor-scrollable">
                <div class="code-editor-line-numbers">
                    { line_numbers }
                </div>
                <div
                    node_ref=code_editor_ref
                    on:input=on_input
                    contenteditable={!disabled}
                    class="code-editor"
                    spellcheck=false
                >
                </div>
            </div>
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
                <CodeEditor disabled=disable_input code_editor_options code=code set_code=set_code />
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
                <CodeEditor disabled=true code_editor_options code=parsed_code set_code=set_parsed_code />
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
        code.lines().count()
    };

    (1..=number_of_lines)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_numbers() {
        assert_eq!(get_line_numbers(""), "1");
        assert_eq!(get_line_numbers("line1\nline2"), "1\n2");
        assert_eq!(get_line_numbers("line1\nline2\nline3"), "1\n2\n3");
        assert_eq!(get_line_numbers("line1\nline2\nline3\n"), "1\n2\n3");
        assert_eq!(get_line_numbers("line1\nline2\nline3\nline4"), "1\n2\n3\n4");
    }

    #[test]
    fn test_get_line_numbers_with_multiple_empty_lines() {
        assert_eq!(get_line_numbers("\n\n\n\n\n"), "1\n2\n3\n4\n5");
    }
}
