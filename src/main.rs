pub mod app;
pub mod glue;
pub mod pages;
pub mod components;
pub mod hooks;

use std::future::Future;

use app::App;
use yew::prelude::*;

pub async fn emit_callback_if_ok<T, E: std::fmt::Display, F: Future<Output = Result<T, E>>>(future: F, callback: Callback<T>) {
    match future.await {
        Ok(result) => callback.emit(result),
        Err(err) => log::error!("Could not emit callback due to error in future:\n{}", err)
    };
}

#[derive(Debug)]
pub enum Error {
    BackendError(common_data_lib::BackendError),
    SerdeWasmBindgenError(serde_wasm_bindgen::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendError(e) => write!(f, "Backend error: {}", e),
            Self::SerdeWasmBindgenError(e) => write!(f, "Serde WASM bindgen error: {}", e)
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
