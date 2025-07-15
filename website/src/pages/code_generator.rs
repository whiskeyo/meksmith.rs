use leptos::prelude::*;

use crate::components::code_editor::{CodeEditorLanguage, CodeEditorOptions, CodeEditorWithOutput};
use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn CodeGenerator() -> impl IntoView {
    let (code, set_code) = signal(String::new());

    view! {
        <div class="center">
            <h2><TextWithAnimatedGradient text="meksmith.rs" /> " code generator"</h2>
            <CodeEditorWithOutput
                input_code_editor_options=CodeEditorOptions {
                    width: 785,
                    height: 600,
                    language: CodeEditorLanguage::Meklang,
                    disabled: false,
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
        </div>
    }
}
