use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;

use crate::components::code_editor::{
    CodeEditorLanguage, CodeEditorOptions, CodeEditorWithOutput, EmitTarget,
};
use crate::components::text::TextWithAnimatedGradient;

/// [`include_example`] macro extracts the contents of a file from the `meksmith/examples`
/// directory. It is used to include examples during compilation, allowing examples to be
/// embedded directly into the WASM code.
macro_rules! include_example {
    ($name:literal) => {
        include_str!(concat!("../../../meksmith/examples/", $name, ".mek"))
    };
}

macro_rules! include_fixture {
    ($name:literal) => {
        include_str!(concat!("../../../meksmith/tests/protocols/", $name, ".mek"))
    };
}

#[derive(Clone, Debug)]
struct Example {
    name: &'static str,
    example_code: &'static str,
}

static EXAMPLES: &[Example] = &[
    Example {
        name: "bit-packed header (synthetic demo)",
        example_code: include_fixture!("bitpack_header"),
    },
    Example {
        name: "evolved Common Public Radio Interface (eCPRI)",
        example_code: include_example!("ecpri"),
    },
];

#[component]
pub fn Examples() -> impl IntoView {
    let (selected_example, set_selected_example) = signal(EXAMPLES[0].clone());
    let (code, set_code) = signal(String::from(selected_example.get().example_code));
    let (emit_target, set_emit_target) = signal(EmitTarget::C);

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
                show_target_selector=false
                emit_target=emit_target
                set_emit_target=set_emit_target
                code
                set_code
            />
            <div class="example-controls w-1600">
                <label for="example-select" class="common-label">"Example: "</label>
                <select class="common-select example-select" id="example-select" on:change=move |event| {
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
                <label for="emit-target-select" class="common-label">"Output target: "</label>
                <select
                    id="emit-target-select"
                    class="common-select emit-target-select"
                    on:change=move |event| {
                        let value = event
                            .target()
                            .unwrap()
                            .unchecked_into::<web_sys::HtmlSelectElement>()
                            .value();
                        set_emit_target.set(if value == "cpp" {
                            EmitTarget::Cpp
                        } else {
                            EmitTarget::C
                        });
                    }
                >
                    <option value="c" selected=move || emit_target.get() == EmitTarget::C>
                        "C"
                    </option>
                    <option value="cpp" selected=move || emit_target.get() == EmitTarget::Cpp>
                        "C++23"
                    </option>
                </select>
            </div>
        </div>
    }
}
