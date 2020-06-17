use super::caret::Caret;
use super::Position;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlElement, KeyboardEvent};
use yew::prelude::*;
use yew::utils::window;

pub struct Editor {
    link: ComponentLink<Self>,
    content_ref: NodeRef,
    canvas_ref: NodeRef,
    caret_position: Position,
    content: Vec<String>,
}
#[derive(Debug)]
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

impl Editor {
    fn recalculate_position(&self) {
        let c: Element = self
            .content_ref
            .cast()
            .expect("Could not find content element");
        let rect = c.get_bounding_client_rect();
        log::debug!("Window Pos -> x: {}, y: {}", rect.x(), rect.y());
        let pos = self.caret_position;
        log::debug!("Caret Pos  -> x: {}, y: {}", pos.line, pos.column);
    }
}

impl Component for Editor {
    type Message = EditorMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Editor {
            link,
            content_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            caret_position: Position::new(),
            content: vec![String::new()],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::trace!("msg: {:?}", msg);

        match msg {
            EditorMsg::KeyPressed(k) => {
                let action_did_something = match k {
                    // Appends characters to the content
                    KeyboardKey::Printable(mut ch) => {
                        let line: &mut String =
                            self.content.get_mut(self.caret_position.line).unwrap();

                        let ch = ch.pop().unwrap();
                        line.insert(self.caret_position.column, ch);
                        self.caret_position.column += 1;
                        true
                    }
                    // Removes characters from the content
                    KeyboardKey::Backspace => {
                        // If we are in the middle of hte text
                        // - Remove the previous character
                        // - Update the caret position
                        if self.caret_position.column > 0 {
                            let line = self.content.get_mut(self.caret_position.line).unwrap();
                            self.caret_position.column -= 1;
                            line.remove(self.caret_position.column);
                            true
                        // If we are not in the first line and in the first character
                        // - Concatenate the current line to the previous one
                        // - Remove the current line from the state
                        // - Update position to previous line to where the concatenation happened
                        } else if self.caret_position.line > 0 {
                            let current_line =
                                self.content.get(self.caret_position.line).unwrap().clone();
                            let previous_line =
                                self.content.get_mut(self.caret_position.line - 1).unwrap();

                            self.caret_position.column = previous_line.len();
                            previous_line.push_str(&current_line);
                            self.content.remove(self.caret_position.line);
                            self.caret_position.line -= 1;
                            true
                        } else {
                            false
                        }
                    }
                    // Adds new lines
                    KeyboardKey::Enter => {
                        let mut new_line = String::new();
                        let existing_line = self.content.get_mut(self.caret_position.line).unwrap();
                        new_line.push_str(
                            &existing_line[self.caret_position.column..existing_line.len()],
                        );
                        *existing_line =
                            String::from(&existing_line[0..self.caret_position.column]);
                        self.content.push(new_line);
                        self.caret_position.line += 1;
                        self.caret_position.column = 0;
                        true
                    }
                    KeyboardKey::ArrowLeft => {
                        if self.caret_position.column > 0 {
                            self.caret_position.column -= 1;
                            true
                        } else if self.caret_position.line > 0 {
                            let pre_line = self.content.get(self.caret_position.line - 1).unwrap();
                            self.caret_position.column = pre_line.len();
                            self.caret_position.line -= 1;
                            true
                        } else {
                            false
                        }
                    }
                    KeyboardKey::ArrowRight => {
                        let line = self.content.get(self.caret_position.line).unwrap();
                        if self.caret_position.column < line.len() {
                            self.caret_position.column += 1;
                            true
                        } else if self.caret_position.column == line.len()
                            && self.content.len() > self.caret_position.line + 1
                        {
                            self.caret_position.line += 1;
                            self.caret_position.column = 0;
                            true
                        } else {
                            false
                        }
                    }
                    KeyboardKey::ArrowUp => {
                        if self.caret_position.line > 0 {
                            self.caret_position.line -= 1;
                            let pre_line = self.content.get(self.caret_position.line).unwrap();
                            if pre_line.len() < self.caret_position.column {
                                self.caret_position.column = pre_line.len();
                            }
                            true
                        } else if self.caret_position.column > 0 {
                            self.caret_position.column = 0;
                            true
                        } else {
                            false
                        }
                    }
                    KeyboardKey::ArrowDown => {
                        if self.caret_position.line + 1 < self.content.len() {
                            self.caret_position.line += 1;
                            let next_line = self.content.get(self.caret_position.line).unwrap();
                            if next_line.len() < self.caret_position.column {
                                self.caret_position.column = next_line.len();
                            }
                            true
                        } else {
                            let line = self.content.get(self.caret_position.line).unwrap();
                            if self.caret_position.column < line.len() {
                                self.caret_position.column = line.len();
                                true
                            } else {
                                false
                            }
                        }
                    }
                    _ => false, /* Ignore key press */
                };
                if action_did_something {
                    self.recalculate_position()
                }
                action_did_something
            }
            _ => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let content: String = self.content.iter().fold(String::new(), |mut acc, line| {
            acc.push_str(line);
            acc.push('\n');
            acc
        });

        let position = self
            .content_ref
            .cast()
            .map(|elem: HtmlElement| {
                let css = window()
                    .get_computed_style(&elem)
                    .expect("Error retrieving computed style")
                    .unwrap();

                let font = css
                    .get_property_value("font")
                    .expect("Error getting font from css");
                let canvas: HtmlCanvasElement = self.canvas_ref.cast().unwrap();
                let context = canvas
                    .get_context("2d")
                    .expect("Error getting 2d context on canvas")
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                context.set_font(&font);
                let x_pos = context
                    .measure_text(&content[..self.caret_position.column])
                    .expect("Error getting text measurement for content")
                    .width();

                let line_height = css
                    .get_property_value("line-height")
                    .expect("Error getting the line height from css");
                let line_height: usize = line_height[..line_height.len() - 2].parse().unwrap();
                let y_pos = self.caret_position.line * line_height;

                log::debug!("{:?}", line_height);
                Position {
                    line: y_pos,
                    column: x_pos as usize,
                }
            })
            .unwrap_or_default();

        html! {
            <div class="h-full w-full bg-gray-200 p-3">
                <div
                    class="h-full w-full outline-none"
                    tabindex=1
                    onkeydown=self.link.callback(|e: KeyboardEvent| {
                        e.prevent_default();
                        EditorMsg::KeyPressed(e.into())
                    })
                >
                    <Caret position=position/>
                    <pre ref=self.content_ref.clone() class="top--4 relative leading-tight">{content}</pre>
                    <canvas ref=self.canvas_ref.clone() class="hidden"></canvas>
                </div>
            </div>
        }
    }
}
