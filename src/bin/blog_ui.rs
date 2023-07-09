use blog_ui::*;

fn main() {
    if cfg!(debug_assertions) {
        wasm_logger::init(wasm_logger::Config::default());
    }

    app_renderer().render();
}
