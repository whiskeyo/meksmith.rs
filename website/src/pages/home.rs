use leptos::prelude::*;

use crate::components::code_editor::CodeEditorWithOutput;
use crate::components::hyperlink::{ExternalHyperlink, InternalHyperlink};
use crate::components::text::TextWithAnimatedGradient;

#[component]
pub fn Home() -> impl IntoView {
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
                    <InternalHyperlink
                        href="/cheatsheet"
                        children=view! { "cheatsheet" }
                    />
                    " and start creating your protocols using the "
                    <InternalHyperlink
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
