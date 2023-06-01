use blog_ui::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); // TODO: `wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));`

    app_renderer().render();
}
