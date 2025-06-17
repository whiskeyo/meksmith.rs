use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};

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
                    <Route path=leptos_router::path!("/about") view=About/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NavigationBar() -> impl IntoView {
    view! {
        <nav>
            <div class="nav-flex">
                <ul class="nav-left">
                    <li><img data-trunk src="assets/images/logo.png" alt="meksmith Logo" class="logo"/></li>
                    <li><A href="/">"meksmith.rs" { " v".to_string() + env!("CARGO_PKG_VERSION") } </A></li>
                </ul>
                <ul class="nav-right">
                    <li><A href="/about">"About"</A></li>
                    | <li><A href="https://github.com/whiskeyo/meksmith.rs">"Repo"</A></li>
                    | <li><A href="https://github.com/whiskeyo/meksmith.rs/issues">"Issues"</A></li>
                    | <li><A href="https://github.com/whiskeyo/meksmith.rs/pulls">"PRs"</A></li>
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="home">
            <h1>"Welcome to meksmith.rs"</h1>
            <p>"Placeholder for the home page content."</p>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="about">
            <h1>"About meksmith.rs"</h1>
            <p>"Placeholder for the about page content."</p>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404 - Page Not Found"</h1>
            <p>"The page you are looking for does not exist."</p>
            <p><a href="/">"Go back to Home"</a></p>
        </div>
    }
}
