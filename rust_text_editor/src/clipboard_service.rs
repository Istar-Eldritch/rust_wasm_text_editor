use js_sys::JsString;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use yew::utils::window;

pub struct ClipboardService {}

#[cfg(web_sys_unstable_apis)]
impl ClipboardService {
    pub async fn read_text(cb: Callback<String>) {
        let clipboard = window().navigator().clipboard();
        let text: String = JsFuture::from(clipboard.read_text())
            .await
            .unwrap()
            .dyn_into::<JsString>()
            .unwrap()
            .into();
        cb.emit(text);
    }
}
