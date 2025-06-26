use leptos::prelude::*;

#[component]
pub fn InternalHyperlink(href: &'static str, children: impl IntoView) -> impl IntoView {
    view! {
        <a class="hyperlink" href=href>
            { children }
        </a>
    }
}

#[component]
pub fn ExternalHyperlink(href: &'static str, children: impl IntoView) -> impl IntoView {
    view! {
        <a class="hyperlink" href=href rel="external">
            { children }
        </a>
    }
}
