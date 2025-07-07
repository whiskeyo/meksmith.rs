use leptos::prelude::*;

use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn NavigationBar() -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let path = move || location.pathname.get();

    view! {
        <nav>
            <div class="nav-flex">
                <ul class="nav-left">
                    <li><img data-trunk src="assets/images/logo.svg" alt="meksmith Logo" class="logo"/></li>
                    <li><a class="hyperlink" href="/"><TextWithAnimatedGradient text="meksmith.rs" /> { " v".to_string() + env!("CARGO_PKG_VERSION") } </a></li>
                </ul>
                <ul class="nav-right">
                    <li class={move || if path().ends_with("/code-generator") { "active" } else { "" }}>
                        <a class="hyperlink" href="/code-generator">"code generator"</a>
                    </li>
                    |
                    <li class={move || if path().ends_with("/cheatsheet") { "active" } else { "" }}>
                        <a class="hyperlink" href="/cheatsheet">"cheatsheet"</a>
                    </li>
                    |
                    <li>
                        <a class="hyperlink" href="https://github.com/whiskeyo/meksmith.rs" rel="external">"repo"</a>
                    </li>
                    |
                    <li>
                        <a class="hyperlink" href="https://github.com/whiskeyo/meksmith.rs/issues" rel="external">"issues"</a>
                    </li>
                    |
                    <li>
                        <a class="hyperlink" href="https://github.com/whiskeyo/meksmith.rs/pulls" rel="external">"PRs"</a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
