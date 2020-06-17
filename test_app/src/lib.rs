use rust_text_editor::editor::Editor;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex flex-col h-full p-20">
                <h3 class="">{"Editor:"}</h3>
                <div class="flex-grow mt-4">
                    <Editor/>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Trace).expect("Error initializing log system");
    log::info!("App started");
    yew::start_app::<App>();
    Ok(())
}
