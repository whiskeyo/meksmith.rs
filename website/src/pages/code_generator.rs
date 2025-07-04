use leptos::prelude::*;

use crate::components::code_editor::{CodeEditorOptions, CodeEditorWithOutput};
use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn CodeGenerator() -> impl IntoView {
    view! {
        <div class="center">
            <h2><TextWithAnimatedGradient text="meksmith.rs" /> " code generator"</h2>
            <CodeEditorWithOutput
                code_editor_options=CodeEditorOptions {
                    width_in_pixels: 750,
                    height_in_pixels: 600,
                }
                extra_section_classes="w-1600"
                meklang_code=String::new()
                disable_input=false
            />
        </div>
    }
}
