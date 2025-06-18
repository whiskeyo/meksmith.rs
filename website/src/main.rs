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
                    <Route path=leptos_router::path!("/tool") view=Tool/>
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
                    <li><a class="hyperlink" href="/"><TextWithAnimatedGradient text="meksmith.rs".to_string() /> { " v".to_string() + env!("CARGO_PKG_VERSION") } </a></li>
                </ul>
                <ul class="nav-right">
                    <li><a class="hyperlink" href="/tool">"tool"</a></li>
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
                "Define your protocols with " <TextWithAnimatedGradient text="meksmith.rs".to_string() />
            </h1>
            <section class="w-800">
                <p>
                    "If you came here for the first time, you probably think: what is "
                    <TextWithAnimatedGradient text="meksmith.rs".to_string() />
                    "?"
                </p>
                <p>
                    <TextWithAnimatedGradient text="meksmith.rs".to_string() />
                    " is a combination of a "
                    <TextWithAnimatedGradient text="meklang".to_string() />
                    ", which is a DSL with syntax similar to C, but it contains a few extensions, which allow you"
                    " to define binary protocols (such as "
                    <a
                        href="https://www.cpri.info/downloads/eCPRI_v_2.0_2019_05_10c.pdf"
                        rel="external"
                        class="hyperlink"
                    >
                        "eCPRI"
                    </a>
                    ", "
                    <a
                        href="https://docs.o-ran-sc.org/projects/o-ran-sc-o-du-phy/en/latest/Transport-Layer-and-ORAN-Fronthaul-Protocol-Implementation_fh.html"
                        rel="external"
                        class="hyperlink"
                    >
                        <span class="no-break">"ORAN FH"</span>
                    </a>
                    " and many others), and a code generator"
                    " that produces ready-to-use code in C language. Don't worry, more languages will be supported"
                    " in the future."
                </p>
            </section>
            <CodeEditorWithOutput
                width=40
                height=16
                extra_section_classes="w-800".to_string()
                meklang_code=example_code
                disable_input=true
            />
            <section class="w-800">
                <h2>"Are you interested in using " <TextWithAnimatedGradient text="meksmith.rs".to_string() /> "?"</h2>
                <p>
                    "Then be sure to check out the documentation"
                </p>
            </section>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="about">
            <h1>"About " <TextWithAnimatedGradient text="meksmith.rs".to_string() /></h1>
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
fn Tool() -> impl IntoView {
    view! {
        <div class="center">
            <h2><TextWithAnimatedGradient text="meksmith.rs".to_string() /> " generator tool"</h2>
            <CodeEditorWithOutput
                width=100
                height=45
                extra_section_classes="w-1600".to_string()
                meklang_code=String::new()
                disable_input=false
            />
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
    extra_section_classes: String,
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
        <section class={extra_section_classes + " flex-container flex-row"}>
            <div class="flex-1">
                <h3>"Input in " <TextWithAnimatedGradient text="meklang".to_string() /> </h3>
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
fn MeksmithRs() -> impl IntoView {
    view! {
        <span class="animated-green-gradient-text">"meksmith.rs"</span>
    }
}

#[component]
fn Meklang() -> impl IntoView {
    view! {
        <span class="animated-green-gradient-text">"meklang"</span>
    }
}

#[component]
fn TextWithAnimatedGradient(text: String) -> impl IntoView {
    view! {
        <span class="animated-green-gradient-text">{ text }</span>
    }
}
