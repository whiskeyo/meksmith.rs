use crate::components::hyperlink::{ExternalHyperlink, InternalHyperlink};
use crate::components::text::TextWithAnimatedGradient;

use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="center w-800">
            <h1>"Oh smith! You entered the wrong link!"</h1>
            <h3>"The page you are looking for does not exist."</h3>
            <br/>
            <p>
                "There is a small chance that there was a page one day. If you are sure that something should be here, please "
                <ExternalHyperlink
                    href="https://www.github.com/whiskeyo/meksmith.rs/issues/new"
                    children=view! { "open an issue on GitHub" }
                />
                " and describe the problem, I will surely try to help you."
            </p>
            <p>
                "Otherwise, feel free to check the website again in a few days, or just use these links that you can easily find on the page. "
            </p>
            <br/>
            <br/>
            <br/>
            <h3>
                "Go back to main page of "
                <InternalHyperlink href="/" children=view! {
                    <TextWithAnimatedGradient text="meksmith.rs" />
                } />
            </h3>
        </div>
    }
}
