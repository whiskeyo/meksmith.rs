use crate::components::text::TextWithAnimatedGradient;
use crate::utils::static_regex::static_regex;

use leptos::prelude::*;
use regex_lite::Regex;

#[derive(Clone, Debug)]
pub(crate) enum CodeEditorLanguage {
    #[allow(dead_code)]
    PlainText,
    Meklang,
    C,
}

static_regex!(MEKLANG_KEYWORDS_REGEX, r"\b(enum|struct|union|using)\b");
static_regex!(
    MEKLANG_BUILTIN_TYPES_REGEX,
    r"\b(uint8|uint16|uint32|uint64|int8|int16|int32|int64|float32|float64|bit|byte)\b"
);
static_regex!(MEKLANG_COMMENT_REGEX, r"#.*");

static_regex!(C_KEYWORDS_REGEX, r"\b(enum|struct|union|typedef|static)\b");
static_regex!(
    C_BUILTIN_TYPES_REGEX,
    r"\b(int|unsigned|long|uint8_t|uint16_t|uint32_t|uint64_t|int8_t|int16_t|int32_t|int64_t|float|double|bool|char)\b"
);

impl CodeEditorLanguage {
    fn get_highlighter(&self) -> LanguageHighlighter {
        const KEYWORD_CLASS: &str = "code-editor-highlight-keyword";
        const BUILTIN_TYPE_CLASS: &str = "code-editor-highlight-builtin-type";
        const COMMENT_CLASS: &str = "code-editor-highlight-comment";

        match self {
            CodeEditorLanguage::PlainText => LanguageHighlighter { rules: vec![] },
            CodeEditorLanguage::Meklang => LanguageHighlighter {
                rules: vec![
                    (KEYWORD_CLASS, &MEKLANG_KEYWORDS_REGEX),
                    (BUILTIN_TYPE_CLASS, &MEKLANG_BUILTIN_TYPES_REGEX),
                    (COMMENT_CLASS, &MEKLANG_COMMENT_REGEX),
                ],
            },
            CodeEditorLanguage::C => LanguageHighlighter {
                rules: vec![
                    (KEYWORD_CLASS, &C_KEYWORDS_REGEX),
                    (BUILTIN_TYPE_CLASS, &C_BUILTIN_TYPES_REGEX),
                ],
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CodeEditorOptions {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) language: CodeEditorLanguage,
    pub(crate) disabled: bool,
}

impl CodeEditorOptions {
    pub(crate) fn get_formatted_size(&self) -> String {
        format!("width: {}px; height: {}px;", self.width, self.height)
    }

    pub(crate) fn highlight_code(&self, code: &str) -> String {
        self.language.get_highlighter().highlight(code)
    }
}

type CssClass = &'static str;

#[derive(Clone, Debug)]
struct LanguageHighlighter {
    rules: Vec<(CssClass, &'static Regex)>,
}

impl LanguageHighlighter {
    fn highlight(&self, code: &str) -> String {
        let mut highlighted_code = code
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");

        for (css_class, regex) in &self.rules {
            highlighted_code = regex
                .replace_all(&highlighted_code, |caps: &regex_lite::Captures| {
                    format!(r#"<span class="{}">{}</span>"#, css_class, &caps[0])
                })
                .into_owned();
        }

        if highlighted_code.ends_with('\n') {
            highlighted_code.push(' ');
        }

        highlighted_code
    }
}

#[component]
pub fn CodeEditor(
    code_editor_options: CodeEditorOptions,
    #[prop(into)] code: ReadSignal<String>,
    #[prop(into)] set_code: WriteSignal<String>,
) -> impl IntoView {
    let language_highlighter = code_editor_options.language.get_highlighter();

    let textarea_code_ref: NodeRef<leptos::html::Textarea> = NodeRef::new();
    let code_editor_options_for_textarea = code_editor_options.clone();
    textarea_code_ref.on_load(move |textarea| {
        textarea.set_cols(code_editor_options_for_textarea.width);
        textarea.set_rows(code_editor_options_for_textarea.height);
        textarea.set_value(&code.get());
        textarea.set_spellcheck(false);
        textarea.set_class_name("code-editor");
        textarea.set_disabled(code_editor_options_for_textarea.disabled);
    });

    let pre_parsed_code_ref: NodeRef<leptos::html::Pre> = NodeRef::new();
    let code_editor_options_for_pre = code_editor_options.clone();
    pre_parsed_code_ref.on_load(move |pre| {
        pre.set_class_name("code-editor-highlighted");
        pre.set_scroll_top(textarea_code_ref.get().unwrap().scroll_top());
        pre.set_inner_html(
            &code_editor_options_for_pre
                .clone()
                .highlight_code(&code.get()),
        );
    });

    let pre_line_numbers_ref: NodeRef<leptos::html::Pre> = NodeRef::new();
    pre_line_numbers_ref.on_load(move |pre| {
        pre.set_class_name("code-editor-line-numbers");
        pre.set_scroll_top(textarea_code_ref.get().unwrap().scroll_top());
        pre.set_text_content(Some(&get_line_numbers(&code.get())));
    });

    let language_highlighter_for_input_sync = language_highlighter.clone();
    let input_sync = move |_| {
        let textarea = textarea_code_ref.get().unwrap();
        let pre_parsed_code = pre_parsed_code_ref.get().unwrap();
        let pre_line_numbers = pre_line_numbers_ref.get().unwrap();

        set_code.set(textarea.value());
        pre_parsed_code.set_inner_html(
            language_highlighter_for_input_sync
                .highlight(&textarea.value())
                .as_str(),
        );
        pre_line_numbers
            .set_text_content(Some(get_line_numbers(textarea.value().as_str()).as_str()));

        let scroll_top = textarea.scroll_top();
        let scroll_left = textarea.scroll_left();

        pre_parsed_code.set_scroll_top(scroll_top);
        pre_parsed_code.set_scroll_left(scroll_left);
        pre_line_numbers.set_scroll_top(scroll_top);
        pre_line_numbers.set_scroll_left(scroll_left);
    };

    let scroll_sync = move |_| {
        let textarea = textarea_code_ref.get().unwrap();
        let pre_parsed_code = pre_parsed_code_ref.get().unwrap();
        let pre_line_numbers = pre_line_numbers_ref.get().unwrap();

        let scroll_top = textarea.scroll_top();
        let scroll_left = textarea.scroll_left();

        pre_parsed_code.set_scroll_top(scroll_top);
        pre_parsed_code.set_scroll_left(scroll_left);
        pre_line_numbers.set_scroll_top(scroll_top);
        pre_line_numbers.set_scroll_left(scroll_left);
    };

    let keydown = move |event: web_sys::KeyboardEvent| {
        CodeEditorShortcut::from(event.clone()).handle_event(event, &textarea_code_ref, &set_code);
    };

    let language_highlighter_for_effect = language_highlighter.clone();
    Effect::new({
        move |_| {
            if let Some(textarea) = textarea_code_ref.get() {
                if textarea.value() != code.get() {
                    textarea.set_value(&code.get());
                }
            }

            if let Some(pre) = pre_parsed_code_ref.get() {
                pre.set_inner_html(&language_highlighter_for_effect.highlight(&code.get()));
            }

            if let Some(pre) = pre_line_numbers_ref.get() {
                pre.set_text_content(Some(get_line_numbers(&code.get()).as_str()));
            }
        }
    });

    view! {
        <div class="code-editor-container" style=code_editor_options.clone().get_formatted_size()>
            <pre node_ref=pre_line_numbers_ref></pre>
            <pre node_ref=pre_parsed_code_ref></pre>
            <textarea node_ref=textarea_code_ref
                on:input=input_sync
                on:scroll=scroll_sync
                on:keydown=keydown
                aria-label="Code editor"
            ></textarea>
        </div>
    }
}

#[component]
pub fn CodeEditorWithOutput(
    input_code_editor_options: CodeEditorOptions,
    output_code_editor_options: CodeEditorOptions,
    extra_section_classes: &'static str,
    #[prop(into)] code: ReadSignal<String>,
    #[prop(into)] set_code: WriteSignal<String>,
) -> impl IntoView {
    let (parsed_code, set_parsed_code) = signal(String::new());
    let (parsing_error, set_parsing_error) = signal(String::new());

    Effect::new(move |_| {
        match meksmith::smith_c::generate_c_code_from_string(code.get().as_str()) {
            Ok(c_code) => {
                set_parsed_code.set(c_code);
                set_parsing_error.set(String::new());
            }
            Err(e) => set_parsing_error.set(e),
        }
    });

    view! {
        <section class={extra_section_classes.to_string() + " flex-container flex-row"}>
            <div class="flex-1">
                <h3>"Input in " <TextWithAnimatedGradient text="meklang" /> </h3>
                <CodeEditor code_editor_options=input_code_editor_options.clone() code=code set_code=set_code />
                <Show
                    when=move || !parsing_error.get().is_empty()
                >
                    <div class="code-editor-error-box">
                        {move || parsing_error.get()}
                    </div>
                </Show>
            </div>
            <div class="flex-1">
                <h3>"Generated output in C"</h3>
                <CodeEditor code_editor_options=output_code_editor_options.clone() code=parsed_code set_code=set_parsed_code />
            </div>
        </section>
    }
}

/// Returns all line numbers separated by a newline in the given code string.
/// The number of lines is determined by counting the number of newline characters
/// in the code, supporting also multiple empty lines. Numbering starts from 1
/// and each line (including empty lines) is numbered sequentially.
fn get_line_numbers(code: &str) -> String {
    let number_of_lines = if code.is_empty() {
        1
    } else {
        code.split('\n').count()
    };

    (1..=number_of_lines)
        .map(|n| n.to_string() + "\n")
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Clone, Debug, PartialEq)]
enum CodeEditorShortcut {
    Tab,
    ShiftTab,
    CtrlLeftBracket,
    CtrlRightBracket,
    CtrlX,
    AltDownArrow,
    AltUpArrow,
    Other,
}

impl CodeEditorShortcut {
    pub fn handle_event(
        &self,
        event: web_sys::KeyboardEvent,
        textarea_ref: &NodeRef<leptos::html::Textarea>,
        set_code: &WriteSignal<String>,
    ) {
        match self {
            CodeEditorShortcut::Tab => self.tab(event, textarea_ref, set_code),
            CodeEditorShortcut::ShiftTab => self.outdent(event, textarea_ref, set_code),
            CodeEditorShortcut::CtrlLeftBracket => self.outdent(event, textarea_ref, set_code),
            CodeEditorShortcut::CtrlRightBracket => self.indent(event, textarea_ref, set_code),
            CodeEditorShortcut::CtrlX => self.cut_or_remove_line(event, textarea_ref, set_code),
            CodeEditorShortcut::AltDownArrow => self.move_line_down(event, textarea_ref, set_code),
            CodeEditorShortcut::AltUpArrow => self.move_line_up(event, textarea_ref, set_code),
            CodeEditorShortcut::Other => {}
        }
    }

    fn tab(
        &self,
        event: web_sys::KeyboardEvent,
        textarea_code_ref: &NodeRef<leptos::html::Textarea>,
        set_code: &WriteSignal<String>,
    ) {
        event.prevent_default();
        with_textarea(textarea_code_ref, |textarea, start, _end, value| {
            let mut new_value = value.clone();
            new_value.insert(start, '\t');
            set_code.set(new_value.clone());
            textarea.set_value(&new_value);
            textarea
                .set_selection_range((start + 1) as u32, (start + 1) as u32)
                .unwrap();
        });
    }

    fn indent(
        &self,
        event: web_sys::KeyboardEvent,
        textarea_code_ref: &NodeRef<leptos::html::Textarea>,
        set_code: &WriteSignal<String>,
    ) {
        event.prevent_default();
        with_textarea(textarea_code_ref, |textarea, start, _end, value| {
            let line_start = value[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

            let mut new_value = value.clone();
            new_value.insert(line_start, '\t');

            let new_start = start + 1;
            let new_end = _end + 1;

            set_code.set(new_value.clone());
            textarea.set_value(&new_value);
            textarea
                .set_selection_range(new_start as u32, new_end as u32)
                .unwrap();
        });
    }

    fn outdent(
        &self,
        event: web_sys::KeyboardEvent,
        textarea_code_ref: &NodeRef<leptos::html::Textarea>,
        set_code: &WriteSignal<String>,
    ) {
        event.prevent_default();
        with_textarea(textarea_code_ref, |textarea, start, end, value| {
            let line_start = value[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let line = &value[line_start..start];

            let mut removed = 0;
            let mut new_value = value.clone();

            if line.starts_with('\t') {
                new_value.replace_range(line_start..line_start + 1, "");
                removed = 1;
            } else {
                let spaces = line.chars().take_while(|&c| c == ' ').count().min(4);
                if spaces > 0 {
                    new_value.replace_range(line_start..line_start + spaces, "");
                    removed = spaces;
                }
            }

            if removed > 0 {
                set_code.set(new_value.clone());
                textarea.set_value(&new_value);
                let new_start = start.saturating_sub(removed);
                let new_end = end.saturating_sub(removed);
                textarea
                    .set_selection_range(new_start as u32, new_end as u32)
                    .unwrap();
            }
        });
    }

    fn cut_or_remove_line(
        &self,
        event: web_sys::KeyboardEvent,
        textarea_code_ref: &NodeRef<leptos::html::Textarea>,
        set_code: &WriteSignal<String>,
    ) {
        with_textarea(textarea_code_ref, |textarea, start, end, value| {
            event.prevent_default();
            let mut new_value = value.clone();

            if start != end {
                new_value.replace_range(start..end, "");
                set_code.set(new_value.clone());
                textarea.set_value(&new_value);
                textarea
                    .set_selection_range(start as u32, start as u32)
                    .unwrap();
            } else {
                let line_start = value[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                let line_end = value[start..].find('\n').map_or(value.len(), |i| start + i);

                let mut remove_end = line_end;
                if remove_end < value.len() && value.as_bytes()[remove_end] == b'\n' {
                    remove_end += 1;
                }
                new_value.replace_range(line_start..remove_end, "");

                let new_pos = line_start.min(new_value.len());
                set_code.set(new_value.clone());
                textarea.set_value(&new_value);
                textarea
                    .set_selection_range(new_pos as u32, new_pos as u32)
                    .unwrap();
            }
        });
    }

    fn move_line_down(
        &self,
        event: web_sys::KeyboardEvent,
        textarea_code_ref: &NodeRef<leptos::html::Textarea>,
        set_code: &WriteSignal<String>,
    ) {
        event.prevent_default();
        with_textarea(textarea_code_ref, |textarea, start, _end, value| {
            let line_start = value[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let line_end = value[start..].find('\n').map_or(value.len(), |i| start + i);

            if line_end < value.len() {
                let next_line_start = line_end + 1;
                let next_line_end = value[next_line_start..]
                    .find('\n')
                    .map_or(value.len(), |i| next_line_start + i);
                let current_line = &value[line_start..line_end];
                let next_line = &value[next_line_start..next_line_end];

                let mut new_value = value.clone();
                new_value.replace_range(line_start..next_line_end, "");
                new_value.insert_str(line_start, &format!("{next_line}\n{current_line}"));

                set_code.set(new_value.clone());
                textarea.set_value(&new_value);

                let column = start - line_start;
                let new_line_start = line_start + next_line.len() + 1;
                let new_cursor = new_line_start + column.min(current_line.len());

                textarea
                    .set_selection_range(new_cursor as u32, new_cursor as u32)
                    .unwrap();
            }
        });
    }

    fn move_line_up(
        &self,
        event: web_sys::KeyboardEvent,
        textarea_code_ref: &NodeRef<leptos::html::Textarea>,
        set_code: &WriteSignal<String>,
    ) {
        event.prevent_default();
        with_textarea(textarea_code_ref, |textarea, start, _end, value| {
            let line_start = value[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let line_end = value[start..].find('\n').map_or(value.len(), |i| start + i);

            if line_start > 0 {
                let prev_line_start = value[..line_start - 1]
                    .rfind('\n')
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let prev_line_end = line_start - 1;
                let prev_line = &value[prev_line_start..prev_line_end];
                let current_line = &value[line_start..line_end];

                let mut new_value = value.clone();
                new_value.replace_range(prev_line_start..line_end, "");
                new_value.insert_str(prev_line_start, &format!("{current_line}\n{prev_line}"));

                set_code.set(new_value.clone());
                textarea.set_value(&new_value);

                let column = start - line_start;
                let new_line_start = prev_line_start;
                let new_cursor = new_line_start + column.min(current_line.len());

                textarea
                    .set_selection_range(new_cursor as u32, new_cursor as u32)
                    .unwrap();
            }
        });
    }
}

impl From<web_sys::KeyboardEvent> for CodeEditorShortcut {
    fn from(event: web_sys::KeyboardEvent) -> Self {
        const SHIFT: bool = true;
        const NO_SHIFT: bool = false;
        const CTRL: bool = true;
        const NO_CTRL: bool = false;
        const ALT: bool = true;
        const NO_ALT: bool = false;

        match (
            event.ctrl_key(),
            event.alt_key(),
            event.shift_key(),
            event.key().as_str(),
        ) {
            (NO_CTRL, NO_ALT, NO_SHIFT, "Tab") => CodeEditorShortcut::Tab,
            (NO_CTRL, NO_ALT, SHIFT, "Tab") => CodeEditorShortcut::ShiftTab,
            (CTRL, NO_ALT, NO_SHIFT, "[") => CodeEditorShortcut::CtrlLeftBracket,
            (CTRL, NO_ALT, NO_SHIFT, "]") => CodeEditorShortcut::CtrlRightBracket,
            (CTRL, NO_ALT, NO_SHIFT, "x") => CodeEditorShortcut::CtrlX,
            (NO_CTRL, ALT, NO_SHIFT, "ArrowDown") => CodeEditorShortcut::AltDownArrow,
            (NO_CTRL, ALT, NO_SHIFT, "ArrowUp") => CodeEditorShortcut::AltUpArrow,
            _ => CodeEditorShortcut::Other,
        }
    }
}

fn with_textarea<Function: FnOnce(web_sys::HtmlTextAreaElement, usize, usize, String)>(
    textarea_ref: &NodeRef<leptos::html::Textarea>,
    function: Function,
) {
    let textarea = textarea_ref.get().unwrap();
    let start = textarea.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;
    let end = textarea.selection_end().unwrap_or(Some(0)).unwrap_or(0) as usize;
    let value = textarea.value();
    function(textarea, start, end, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_editor_options_get_formatted_size() {
        let options = CodeEditorOptions {
            width: 800,
            height: 600,
            language: CodeEditorLanguage::PlainText,
            disabled: false,
        };

        assert_eq!(options.get_formatted_size(), "width: 800px; height: 600px;");
    }

    #[test]
    fn test_get_line_numbers() {
        assert_eq!(get_line_numbers(""), "1\n");
        assert_eq!(get_line_numbers("line1\nline2"), "1\n2\n");
        assert_eq!(get_line_numbers("line1\nline2\nline3"), "1\n2\n3\n");
        assert_eq!(get_line_numbers("line1\nline2\nline3\n"), "1\n2\n3\n4\n");
        assert_eq!(
            get_line_numbers("line1\nline2\nline3\nline4"),
            "1\n2\n3\n4\n"
        );
    }

    #[test]
    fn test_get_line_numbers_with_multiple_empty_lines() {
        assert_eq!(get_line_numbers("\n\n\n\n\n"), "1\n2\n3\n4\n5\n6\n");
    }
}
