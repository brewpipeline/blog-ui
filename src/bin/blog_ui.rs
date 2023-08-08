use blog_ui::*;

fn main() {
    if cfg!(debug_assertions) {
        wasm_logger::init(wasm_logger::Config::default());
    }

    let renderer = app_renderer();

    #[cfg(feature = "hydration")]
    renderer.hydrate();
    #[cfg(not(feature = "hydration"))]
    renderer.render();
}
