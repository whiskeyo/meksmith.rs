mod components;
mod pages;
mod utils;

use crate::components::navbar::NavigationBar;
use crate::pages::about::About;
use crate::pages::cheatsheet::Cheatsheet;
use crate::pages::code_generator::CodeGenerator;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <NavigationBar/>
            <main>
                <Routes fallback=NotFound>
                    <Route path=leptos_router::path!("/") view=Home/>
                    <Route path=leptos_router::path!("/code-generator") view=CodeGenerator/>
                    <Route path=leptos_router::path!("/cheatsheet") view=Cheatsheet/>
                    <Route path=leptos_router::path!("/about") view=About/>
                </Routes>
            </main>
        </Router>
    }
}
