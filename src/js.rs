use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "logAnalytic")]
    fn log_analytic_js(event: JsValue, data: JsValue);
}

pub fn log_analytic<E: AsRef<str>, K: serde::Serialize, V: serde::Serialize>(
    event: E,
    data: &std::collections::HashMap<K, V>,
) {
    use gloo::utils::format::JsValueSerdeExt;

    log_analytic_js(
        JsValue::from_str(event.as_ref()),
        JsValue::from_serde(data).unwrap(),
    )
}
