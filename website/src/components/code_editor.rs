use crate::components::text::TextWithAnimatedGradient;
use leptos::prelude::*;

#[component]
pub fn CodeEditor(
    width: u8,
    height: u8,
    disabled: bool,
    #[prop(into)] code: ReadSignal<String>,
    #[prop(into)] set_code: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <textarea
            class="code-editor"
            rows=height
            cols=width
            disabled=disabled
            placeholder="Type your meklang code here..."
            on:input:target=move |ev| {
                set_code.set(ev.target().value());
            }
        >
            {code}
        </textarea>
    }
}

#[component]
pub fn CodeEditorWithOutput(
    width: u8,
    height: u8,
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
                <CodeEditor disabled=disable_input width height code set_code/>
            </div>
            <div class="flex-1">
                <h3>"Generated output in C"</h3>
                <CodeEditor disabled=true width height code=parsed_code set_code=set_parsed_code/>
            </div>
        </section>
    }
}
