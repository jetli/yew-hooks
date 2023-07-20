use yew_app::app::App;

// This is the entry point for the web app
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
