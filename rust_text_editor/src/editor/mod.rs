// mod editor_content;

use crate::console;
use serde::Deserialize;
use web_sys::KeyboardEvent;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

pub struct Editor {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    cursor_position: Position,
    content: Vec<String>,
}

pub enum EditorMsg {
    KeyPressed(KeyboardKey),
    Error,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum KeyboardKey {
    Enter,
    Backspace,
    Delete,
    Escape,
    Shift,
    Control,
    ArrowDown,
    ArrowUp,
    ArrowLeft,
    ArrowRight,
    Printable(String),
    #[serde(other)]
    Other,
}

impl From<KeyboardEvent> for KeyboardKey {
    fn from(ke: KeyboardEvent) -> Self {
        let key = ke.key();
        if let Ok(k) = serde_plain::from_str(&key) {
            if k == KeyboardKey::Other {
                return KeyboardKey::Printable(key);
            }
            k
        } else {
            KeyboardKey::Other
        }
    }
}

impl Component for Editor {
    type Message = EditorMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Editor {
            link,
            node_ref: NodeRef::default(),
            cursor_position: Position::new(),
            content: vec![String::new()],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            EditorMsg::KeyPressed(k) => {
                console::log(&format!("Pressed: {:?}", k));

                match k {
                    // Appends characters to the content
                    KeyboardKey::Printable(ch) => {
                        let line: &mut String =
                            self.content.get_mut(self.cursor_position.line).unwrap();

                        line.push_str(&ch);
                        self.cursor_position.column += 1;
                        console::log(&format!("{:?}", self.content));
                        true
                    }
                    // Removes characters from the content
                    KeyboardKey::Backspace => {
                        // If we are in the middle of hte text
                        // - Remove the previous character
                        // - Update the cursor position
                        if self.cursor_position.column > 0 {
                            let line = self.content.get_mut(self.cursor_position.line).unwrap();
                            self.cursor_position.column -= 1;
                            line.remove(self.cursor_position.column);
                            true
                        // If we are not in the first line and in the first character
                        // - Remove the previous line newline character
                        // - Concatenate the current line to the previous one
                        // - Remove the current line from the state
                        // - Update position to previous line to where the concatenation happened
                        } else if self.cursor_position.line > 0 {
                            let current_line =
                                self.content.get(self.cursor_position.line).unwrap().clone();
                            // Remove the new line character from the previuos line
                            let previous_line =
                                self.content.get_mut(self.cursor_position.line - 1).unwrap();

                            previous_line.remove(previous_line.len() - 1);
                            self.cursor_position.column = previous_line.len();
                            previous_line.push_str(&current_line);
                            self.content.remove(self.cursor_position.line);
                            self.cursor_position.line -= 1;
                            true
                        } else {
                            false
                        }
                    }
                    // Adds new lines
                    KeyboardKey::Enter => {
                        let current_line = self.content.get_mut(self.cursor_position.line).unwrap();
                        current_line.push('\n');
                        let new_line = String::new();
                        self.content.push(new_line);
                        self.cursor_position.line += 1;
                        self.cursor_position.column = 0;
                        true
                    }
                    _ => false, /* Ignore key press */
                }
            }
            _ => false,
        }
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let content: Vec<Html> = self
            .content
            .iter()
            .map(|line| html! {<div class="line">{{line}}</div>})
            .collect();
        let content = ChildrenRenderer::new(content);
        html! {
            <div
                class="editor"
                tabindex=1
                ref=self.node_ref.clone()
                onkeydown=self.link.callback(|e: KeyboardEvent| {
                    e.prevent_default();
                    EditorMsg::KeyPressed(e.into())
                })
            >
                {content.render()}
            </div>
        }
    }
}

struct EditorContent {
    link: ComponentLink<Self>,
}

impl Component for EditorContent {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        EditorContent { link }
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div class="editor_content">
            </div>
        }
    }
}

// impl Component for Strut {
//     type Message = ();
//     type Properties = ();
//     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//         todo!()
//     }
//     fn update(&mut self, _: Self::Message) -> ShouldRender {
//         todo!()
//     }
//     fn change(&mut self, _: Self::Properties) -> ShouldRender {
//         todo!()
//     }
//     fn view(&self) -> Html {
//         todo!()
//     }
// }

#[derive(Debug, Copy, Clone)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    fn new() -> Self {
        Position { line: 0, column: 0 }
    }
}
