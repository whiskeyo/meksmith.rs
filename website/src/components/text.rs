use leptos::prelude::*;

#[component]
pub fn TextWithAnimatedGradient(text: &'static str) -> impl IntoView {
    view! {
        <span class="animated-green-gradient-text">{ text }</span>
    }
}
