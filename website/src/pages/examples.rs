use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;

use crate::components::code_editor::{CodeEditorLanguage, CodeEditorOptions, CodeEditorWithOutput};
use crate::components::text::TextWithAnimatedGradient;

/// [`include_example`] macro extracts the contents of a file from the `meksmith/examples`
/// directory. It is used to include examples during compilation, allowing examples to be
/// embedded directly into the WASM code.
macro_rules! include_example {
    ($name:literal) => {
        include_str!(concat!("../../../meksmith/examples/data/", $name, ".mek"))
    };
}

#[derive(Clone, Debug)]
struct Example {
    name: &'static str,
    example_code: &'static str,
}

static EXAMPLES: &[Example] = &[
    Example {
        name: "evolved Common Public Radio Interface (eCPRI)",
        example_code: include_example!("ecpri"),
    },
    Example {
        name: "CAN Bus (base frame format)",
        example_code: include_example!("can-bus"),
    },
    Example {
        name: "Ping-Pong Protocol",
        example_code: include_example!("ping-pong"),
    },
];

#[component]
pub fn Examples() -> impl IntoView {
    let (selected_example, set_selected_example) = signal(EXAMPLES[0].clone());
    let (code, set_code) = signal(String::from(selected_example.get().example_code));

    view! {
        <div class="center">
            <h2><TextWithAnimatedGradient text="meksmith.rs" /> " examples"</h2>
            <CodeEditorWithOutput
                input_code_editor_options=CodeEditorOptions {
                    width: 785,
                    height: 600,
                    language: CodeEditorLanguage::Meklang,
                    disabled: true,
                }
                output_code_editor_options=CodeEditorOptions {
                    width: 785,
                    height: 600,
                    language: CodeEditorLanguage::C,
                    disabled: true,
                }
                extra_section_classes="w-1600"
                code
                set_code
            />
            <div class="flex-container flex-row w-1600">
                <div class="flex-1">
                    <label for="example-select" class="common-label">"Example: "</label>
                    <select class="common-select" id="example-select" on:change=move |event| {
                        let selected_value = event.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>().value();
                        if let Some(example) = EXAMPLES.iter().find(|e| e.name == selected_value) {
                            set_selected_example.set(example.clone());
                            set_code.set(String::from(example.example_code));
                        }
                    }>
                        { EXAMPLES.iter().map(|example| view! {
                            <option value=example.name>{ example.name }</option>
                        }).collect_view() }
                    </select>
                </div>
                <div class="flex-1">
                    <label for="language-select" class="common-label">"Output language: "</label>
                    <select class="common-select" id="language-select" disabled=true>
                        <option value="c" selected="selected" disabled>"C"</option>
                    </select>
                </div>
            </div>
        </div>
    }
}
