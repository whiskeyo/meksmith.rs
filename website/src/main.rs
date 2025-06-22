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
                    <Route path=leptos_router::path!("/docs") view=Documentation/>
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
                    <li><a class="hyperlink" href="/"><TextWithAnimatedGradient text="meksmith.rs" /> { " v".to_string() + env!("CARGO_PKG_VERSION") } </a></li>
                </ul>
                <ul class="nav-right">
                    <li><a class="hyperlink" href="/code-generator">"code generator"</a></li>
                    | <li><a class="hyperlink" href="/docs">"docs"</a></li>
                    | <li><a class="hyperlink" href="/about">"about"</a></li>
                    | <li><a class="hyperlink" href="https://github.com/whiskeyo/meksmith.rs" rel="external">"repo"</a></li>
                    | <li><a class="hyperlink" href="https://github.com/whiskeyo/meksmith.rs/issues" rel="external">"issues"</a></li>
                    | <li><a class="hyperlink" href="https://github.com/whiskeyo/meksmith.rs/pulls" rel="external">"PRs"</a></li>
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn Home() -> impl IntoView {
    let example_code = r#"enum MyEnum {
    x = 1;
    y = 2..4;
};

struct MyStruct {
    [bits=3]
    myEnum: MyEnum;
    [bits=5]
    hello: uint8;
};
"#
    .to_string();

    view! {
        <div class="hero">
            <h1>
                "Define your protocols with " <TextWithAnimatedGradient text="meksmith.rs" />
            </h1>
            <section class="w-800">
                <p>
                    "If you came here for the first time, you probably think: what is "
                    <TextWithAnimatedGradient text="meksmith.rs" />
                    "?"
                </p>
                <p>
                    <TextWithAnimatedGradient text="meksmith.rs" />
                    " is a combination of a "
                    <TextWithAnimatedGradient text="meklang" />
                    ", which is a DSL with syntax similar to C, but it contains a few extensions, which allow you"
                    " to define binary protocols (such as "
                    <ExternalHyperlink
                        href="https://www.cpri.info/downloads/eCPRI_v_2.0_2019_05_10c.pdf"
                        children=view! { <span class="no-break">"eCPRI"</span> }
                    />
                    ", "
                    <ExternalHyperlink
                        href="https://docs.o-ran-sc.org/projects/o-ran-sc-o-du-phy/en/latest/Transport-Layer-and-ORAN-Fronthaul-Protocol-Implementation_fh.html"
                        children=view! { <span class="no-break">"ORAN FH"</span> }
                    />
                    " and many others), and a code generator"
                    " that produces ready-to-use code in C language. Don't worry, more languages will be supported"
                    " in the future."
                </p>
            </section>
            <CodeEditorWithOutput
                width=40
                height=16
                extra_section_classes="w-800"
                meklang_code=example_code
                disable_input=true
            />
            <section class="w-800">
                <h2>"Are you interested in using " <TextWithAnimatedGradient text="meksmith.rs" /> "?"</h2>
                <p>
                    "Check out the "
                    <Hyperlink
                        href="/docs"
                        children=view! { "documentation" }
                    />
                    " and start creating your protocols using the "
                    <Hyperlink
                        href="/code-generator"
                        children=view! { <TextWithAnimatedGradient text="meksmith.rs" /> " code generator" }
                    />
                    "! If you have any questions or suggestions, feel free to "
                    <ExternalHyperlink
                        href="https://github.com/whiskeyo/meksmith.rs/issues/new"
                        children=view! { "open an issue" }
                    />
                    " on GitHub."
                </p>
            </section>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="about">
            <h1>"About " <TextWithAnimatedGradient text="meksmith.rs" /></h1>
            <p>"Placeholder for the about page content."</p>
        </div>
    }
}

#[component]
fn Documentation() -> impl IntoView {
    view! {
        <div>
            <h1>"Documentation"</h1>
            <p>"Placeholder for the documentation content."</p>
        </div>
    }
}

#[component]
fn CodeGenerator() -> impl IntoView {
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

#[component]
fn NotFound() -> impl IntoView {
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
                <Hyperlink href="/" children=view! {
                    <TextWithAnimatedGradient text="meksmith.rs" />
                } />
            </h3>
        </div>
    }
}

#[component]
fn CodeEditor(
    width: u8,
    height: u8,
    disabled: bool,
    #[prop(into)] code: ReadSignal<String>,
    #[prop(into)] set_code: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <textarea
            class="code-editor"
            rows=height
            cols=width
            disabled=disabled
            placeholder="Type your meklang code here..."
            on:input:target=move |ev| {
                set_code.set(ev.target().value());
            }
        >
            {code}
        </textarea>
    }
}

#[component]
fn CodeEditorWithOutput(
    width: u8,
    height: u8,
    extra_section_classes: &'static str,
    meklang_code: String,
    disable_input: bool,
) -> impl IntoView {
    let (code, set_code) = signal(meklang_code);
    let (parsed_code, set_parsed_code) = signal(String::new());

    Effect::new(move |_| {
        set_parsed_code.set(
            match meksmith::smith_c::generate_c_code_from_string(code.get().as_str()) {
                Ok(c_code) => c_code,
                Err(e) => format!("Error: {}", e),
            },
        );
    });

    view! {
        <section class={extra_section_classes.to_string() + " flex-container flex-row"}>
            <div class="flex-1">
                <h3>"Input in " <TextWithAnimatedGradient text="meklang" /> </h3>
                <CodeEditor disabled=disable_input width height code set_code/>
            </div>
            <div class="flex-1">
                <h3>"Generated output in C"</h3>
                <CodeEditor disabled=true width height code=parsed_code set_code=set_parsed_code/>
            </div>
        </section>
    }
}

#[component]
fn TextWithAnimatedGradient(text: &'static str) -> impl IntoView {
    view! {
        <span class="animated-green-gradient-text">{ text }</span>
    }
}

#[component]
fn Hyperlink(href: &'static str, children: impl IntoView) -> impl IntoView {
    view! {
        <a class="hyperlink" href=href>
            { children }
        </a>
    }
}

#[component]
fn ExternalHyperlink(href: &'static str, children: impl IntoView) -> impl IntoView {
    view! {
        <a class="hyperlink" href=href rel="external">
            { children }
        </a>
    }
}
