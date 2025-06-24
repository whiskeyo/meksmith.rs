use leptos::prelude::*;

use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="center">
            <h1>"About " <TextWithAnimatedGradient text="meksmith.rs" /></h1>
            <p>"Placeholder for the about page content."</p>
        </div>
    }
}
