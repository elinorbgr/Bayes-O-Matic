use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_functions.js")]
extern "C" {
    #[wasm_bindgen(js_name = "graph_render")]
    pub fn graph_render(dot: JsValue, svg: JsValue);
    #[wasm_bindgen(js_name = "mathjax_typeset")]
    pub fn mathjax_typeset();
    #[wasm_bindgen(js_name = "make_json_download")]
    pub fn make_json_download(filename: JsValue, text: JsValue);
}
