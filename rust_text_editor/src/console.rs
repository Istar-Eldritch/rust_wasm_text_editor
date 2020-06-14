use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    pub fn prompt(s: &str) -> String;

    #[wasm_bindgen(js_namespace = console, js_name = info)]
    pub fn info(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn error(s: &str);
}
