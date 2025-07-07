use leptos::prelude::*;

use crate::components::code_editor::{CodeEditorLanguage, CodeEditorOptions, CodeEditorWithOutput};
use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn CodeGenerator() -> impl IntoView {
    view! {
        <div class="center">
            <h2><TextWithAnimatedGradient text="meksmith.rs" /> " code generator"</h2>
            <CodeEditorWithOutput
                input_code_editor_options=CodeEditorOptions {
                    width: 785,
                    height: 600,
                    language: CodeEditorLanguage::Meklang,
                }
                output_code_editor_options=CodeEditorOptions {
                    width: 785,
                    height: 600,
                    language: CodeEditorLanguage::C,
                }
                extra_section_classes="w-1600"
                // meklang_code=String::new()
                meklang_code=String::from(include_str!("../../../meksmith/examples/ecpri.mek"))
                disable_input=false
            />
        </div>
    }
}
