use leptos::prelude::*;

use crate::components::code_editor::CodeEditorWithOutput;
use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn CodeGenerator() -> impl IntoView {
    view! {
        <div class="center">
            <h2><TextWithAnimatedGradient text="meksmith.rs" /> " code generator"</h2>
            <CodeEditorWithOutput
                width=100
                height=45
                extra_section_classes="w-1600"
                meklang_code=String::new()
                disable_input=false
            />
        </div>
    }
}
