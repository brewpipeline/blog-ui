use blog_ui::*;

fn main() {
    if cfg!(debug_assertions) {
        wasm_logger::init(wasm_logger::Config::default());
    }

    let document = gloo::utils::document();
    let element = document.query_selector("#app").unwrap().unwrap();
    yew::Renderer::<App>::with_root(element).render();
}
