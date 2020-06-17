// mod editor_content;
mod caret;
mod editor;

pub use editor::Editor;
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

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    fn new() -> Self {
        Position { line: 0, column: 0 }
    }
}
