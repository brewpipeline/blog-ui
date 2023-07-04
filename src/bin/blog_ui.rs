use blog_ui::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); // TODO: `wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));`

    let document = gloo::utils::document();
    let element = document.query_selector("#app").unwrap().unwrap();
    yew::Renderer::<App>::with_root(element).render();
}
