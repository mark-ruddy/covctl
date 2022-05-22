use log::info;

mod components;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("WASM logger started!");
    yew::start_app::<components::App>();
}
