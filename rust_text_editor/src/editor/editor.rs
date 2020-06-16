use super::cursor::Cursor;
use super::Position;
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
                    KeyboardKey::Printable(mut ch) => {
                        let line: &mut String =
                            self.content.get_mut(self.cursor_position.line).unwrap();

                        let ch = ch.pop().unwrap();
                        line.insert(self.cursor_position.column, ch);
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
                        // - Concatenate the current line to the previous one
                        // - Remove the current line from the state
                        // - Update position to previous line to where the concatenation happened
                        } else if self.cursor_position.line > 0 {
                            let current_line =
                                self.content.get(self.cursor_position.line).unwrap().clone();
                            let previous_line =
                                self.content.get_mut(self.cursor_position.line - 1).unwrap();

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
                        let mut new_line = String::new();
                        let existing_line =
                            self.content.get_mut(self.cursor_position.line).unwrap();
                        new_line.push_str(
                            &existing_line[self.cursor_position.column..existing_line.len()],
                        );
                        *existing_line =
                            String::from(&existing_line[0..self.cursor_position.column]);
                        self.content.push(new_line);
                        self.cursor_position.line += 1;
                        self.cursor_position.column = 0;
                        true
                    }
                    KeyboardKey::ArrowLeft => {
                        if self.cursor_position.column > 0 {
                            self.cursor_position.column -= 1;
                            true
                        } else if self.cursor_position.line > 0 {
                            let pre_line = self.content.get(self.cursor_position.line - 1).unwrap();
                            self.cursor_position.column = pre_line.len();
                            self.cursor_position.line -= 1;
                            true
                        } else {
                            false
                        }
                    }
                    KeyboardKey::ArrowRight => {
                        let line = self.content.get(self.cursor_position.line).unwrap();
                        if self.cursor_position.column < line.len() {
                            self.cursor_position.column += 1;
                            true
                        } else if self.cursor_position.column == line.len()
                            && self.content.len() > self.cursor_position.line + 1
                        {
                            self.cursor_position.line += 1;
                            self.cursor_position.column = 0;
                            true
                        } else {
                            false
                        }
                    }
                    KeyboardKey::ArrowUp => {
                        if self.cursor_position.line > 0 {
                            self.cursor_position.line -= 1;
                            let pre_line = self.content.get(self.cursor_position.line).unwrap();
                            if pre_line.len() < self.cursor_position.column {
                                self.cursor_position.column = pre_line.len();
                            }
                            true
                        } else if self.cursor_position.column > 0 {
                            self.cursor_position.column = 0;
                            true
                        } else {
                            false
                        }
                    }
                    KeyboardKey::ArrowDown => {
                        if self.cursor_position.line + 1 < self.content.len() {
                            self.cursor_position.line += 1;
                            let next_line = self.content.get(self.cursor_position.line).unwrap();
                            if next_line.len() < self.cursor_position.column {
                                self.cursor_position.column = next_line.len();
                            }
                            true
                        } else {
                            let line = self.content.get(self.cursor_position.line).unwrap();
                            if self.cursor_position.column < line.len() {
                                self.cursor_position.column = line.len();
                                true
                            } else {
                                false
                            }
                        }
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
            <Cursor blinking=true, visible = true, position=self.cursor_position/>
                {content.render()}
            </div>
        }
    }
}
