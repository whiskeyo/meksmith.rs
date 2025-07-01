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
    let line_numbers = move || {
        let code_str = code.get();
        let num_lines = if code_str.is_empty() {
            1
        } else {
            code_str.lines().count().max(1)
        };
        (1..=num_lines)
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    };

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
                    { move || line_numbers }
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

    Effect::new(move |_| {
        set_parsed_code.set(
            match meksmith::smith_c::generate_c_code_from_string(code.get().as_str()) {
                Ok(c_code) => c_code,
                Err(e) => format!("Error: {e}"),
            },
        );
    });

    view! {
        <section class={extra_section_classes.to_string() + " flex-container flex-row"}>
            <div class="flex-1">
                <h3>"Input in " <TextWithAnimatedGradient text="meklang" /> </h3>
                <CodeEditor disabled=disable_input code_editor_options code=code set_code=set_code />
            </div>
            <div class="flex-1">
                <h3>"Generated output in C"</h3>
                <CodeEditor disabled=true code_editor_options code=parsed_code set_code=set_parsed_code />
            </div>
        </section>
    }
}
