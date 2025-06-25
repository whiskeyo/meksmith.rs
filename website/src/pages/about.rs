use leptos::prelude::*;

use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="center">
            <h2>"about " <TextWithAnimatedGradient text="meksmith.rs" /></h2>
            <p>"Placeholder for the about page content."</p>
        </div>
    }
}
